pub mod logger_iterator;

use crate::file_manager::{
    FileManager as FileManager,
    page::Page as Page,
    block::BlockId as BlockId,
};

pub const U64_BYTES:usize = 8;

pub struct Logger<'a>{
    file_manager: &'a std::sync::Mutex<FileManager>,
    log_file: String,
    log_page: Page,
    current_block: BlockId,
    latest_sequence_number: u64,
    last_saved_sequence_number: u64
}

impl<'a> Logger<'a>{
    pub fn new(fm: &'a std::sync::Mutex<FileManager>, log_file: &str)->Result<Self, Box<dyn std::error::Error>>{
        let (mut log_page, log_size, block_size) = {
            let mut mg = fm.lock().unwrap();
            (
                Page::new_as_size(mg.get_block_size() as usize),
                mg.block_num(log_file)?,
                mg.get_block_size()
            )
        };


        let block = match log_size{
            // if file is empty then add a new block
            0 => {
                let mut mg = fm.lock().unwrap();
                let blk = mg.append(log_file)?;
                log_page.set_u64(0, block_size)?;
                mg.write_without_mutex(&blk, &log_page)?;
                blk
            }
            _ =>{
                let blk = BlockId::new(
                    log_file,
                    log_size - 1
                );

                fm.lock()
                .unwrap()
                .read_without_mutex(&blk, &mut log_page)?;
                
                blk
            }
        };
  
        Ok(Self{
            file_manager: fm,
            log_file: log_file.to_string(),
            log_page: log_page,
            last_saved_sequence_number: 0,
            latest_sequence_number: 0,
            current_block: block
        })
    }

    /// Add a new block at the end of the file.
    /// When the page buffer exhausted, the function should be called.
    fn append_new_block(&mut self)->Result<(), Box<dyn std::error::Error> >{
        let mut mg = self.file_manager.lock().unwrap();
        let block = mg.append(&self.log_file)?;
        self.log_page.set_u64(0, mg.get_block_size() as u64)?;
        mg.write_without_mutex(&block, &self.log_page)?;

        self.current_block = block;
        Ok(())
    }


    pub fn flush_by_sequence_number(&mut self, n: u64) -> Result<(), Box<dyn std::error::Error> >{
        if n > self.last_saved_sequence_number{
            self.flush()?;
        }
        self.last_saved_sequence_number = n;
        Ok(())
    }

    /// write page buffer to disk
    pub fn flush(&self)->Result<(), Box<dyn std::error::Error>>{
        self.file_manager.lock()
        .unwrap()
        .write_without_mutex(&self.current_block, &self.log_page)?;

        Ok(())
    }

    pub fn append(&mut self, record: &[u8]) -> Result<(), Box<dyn std::error::Error> >{
        // the first 8 bytes is offsett of the writable position
        let mut boundary = self.log_page.get_u64(0)? as i64;
        let record_size = record.len() as i64;
        let need_size = (U64_BYTES + record_size as usize) as i64;

        println!("{},{}", boundary,need_size);
        // space not enough
        if boundary < need_size + U64_BYTES as i64  {
            self.flush()?;
            // alloc a new block's space
            self.append_new_block()?;

            //calc offset of current writable position
            boundary = self.log_page.get_u64(0)? as i64;
        }

        let record_pos = boundary - need_size;
        // println!("{}",record_pos);

        self.log_page.set_bytes(record_pos as usize, record)?;
        self.log_page.set_u64(0, record_pos as u64)?;  //reset offset
        self.latest_sequence_number += 1;
        Ok(())
    }

    pub fn get_latest_sequence_number(&self) -> u64{
        self.latest_sequence_number
    }

    pub fn get_latest_saved_sequence_number(&self) -> u64{
        self.last_saved_sequence_number
    }


    pub fn iter(&self)->Result<logger_iterator::LogIterator, Box<dyn std::error::Error>>{
        self.flush()?;
        logger_iterator::LogIterator
        ::new(self.file_manager, self.current_block.clone())
    }
}