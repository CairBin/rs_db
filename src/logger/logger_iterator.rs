use crate::file_manager::{
    FileManager,
    block::BlockId,
    page::Page
};
pub struct LogIterator<'a>{
    file_manager: &'a std::sync::Mutex<FileManager>,
    block: BlockId,
    page: Page,
    current_pos: u64,
    boundary: u64,
}

impl<'a> LogIterator<'a> {
    pub fn new(file_manager: &'a std::sync::Mutex<FileManager>, block: BlockId)->Result<Self, Box<dyn std::error::Error> >{
        let page = Page::new_as_size(file_manager.lock().unwrap().get_block_size().try_into().unwrap());
        let boundary = page.get_u64(0)?;
        Ok(Self{
            file_manager: file_manager,
            block: block,
            current_pos: 0,
            page: page,
            boundary: boundary
        })
    }

    pub fn move_to_block(&mut self, block:&BlockId)->Result<(), Box<dyn std::error::Error> >{
        self.file_manager.lock()
        .unwrap()
        .read_without_mutex(block, &mut self.page)?;

        self.boundary = self.page.get_u64(0)?;
        self.current_pos = self.boundary;
        Ok(())
    }

    fn has_next(&self) -> bool{
        self.current_pos < self.file_manager.lock().unwrap().get_block_size()
        || self.block.get_block_number() > 0
    }

}

impl<'a> Iterator for LogIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next(){
            return None;
        }

        let block_size = self.file_manager.lock().unwrap().get_block_size();

        if self.current_pos == block_size {
            let block = BlockId::new(
                self.block.get_filename(), 
                self.block.get_block_number()-1
            );
            match self.move_to_block(&block){
                Ok(_) => {},
                Err(_) => {
                    return None;
                }
            }
            self.block = block;
        }

        let record = match self.page.get_bytes(self.current_pos as usize){

            Ok(r) => r,
            Err(_) => {
                return None;
            }
        };

        self.current_pos = (record.len() + super::U64_BYTES) as u64;
    
        return Some(record.to_vec());
    }
}