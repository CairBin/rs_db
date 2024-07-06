use std::os::unix::fs::FileExt;

use block::BlockId;
use page::Page;

pub mod block;
pub mod page;

#[derive(Debug)]
pub struct FileManager{
    db_directory: String,
    block_size: u64,
    is_new: bool,
    open_files: std::collections::HashMap<String, std::fs::File>,
}

impl FileManager{
    pub fn new(dir:&str, block_size:u64) -> Result<Self, Box<dyn std::error::Error>>{
        let path = std::path::Path::new(dir);

        if !path.exists(){
            std::fs::create_dir_all(path)?;
        }

        // walk the directory and delete the files whose prefix is "temp"
        for entry in std::fs::read_dir(path)?{
            let entry = entry?;
            let path = entry.path();
            if path.file_name().unwrap().to_str().unwrap().starts_with("temp"){
                std::fs::remove_file(path)?;
            }
        }

        Ok(Self{
            db_directory: dir.to_string(),
            block_size: block_size,
            is_new: false,
            open_files: std::collections::HashMap::new(),
        })
    }

    pub fn to_mutex(self) -> std::sync::Mutex<Self>{
            std::sync::Mutex::new(self)
    }

    fn set_file(&mut self, filename:&str) ->Result<(), Box<dyn std::error::Error>> {
        if self.open_files.contains_key(filename){
            return Ok(());
        }

        let path = std::path::Path::new(&self.db_directory).join(filename);
        let file = match path.exists(){
            true =>{
                let mut fp = std::fs::OpenOptions::new();
                fp.read(true).write(true).open(path)?
            }

            false =>{
                let mut fp = std::fs::OpenOptions::new();
                fp.read(true)
                    .write(true)
                    .create(true)
                    .open(path)?
            }
        };
        self.open_files.insert(filename.to_string(), file);

        Ok(())
    }

    pub fn get_file_mut(&mut self, filename:&str)->Option<&mut std::fs::File>{
        self.open_files.get_mut(filename)
    }

    pub fn get_file(&self, filename:&str)->Option<&std::fs::File>{
        self.open_files.get(filename)
    }

    pub fn exists(&self, filename:&str) ->bool{
        self.open_files.contains_key(filename)
    }


    pub fn get_block_size(&self)->u64{
        self.block_size
    }

    pub fn read(manager: &std::sync::Mutex<Self>, block:&BlockId, page:&std::sync::Mutex<Page>)->Result<usize, Box<dyn std::error::Error>>{
        let mut mg = manager.lock().unwrap();
        let (flag, block_size) = (
            mg.exists(block.get_filename()), 
            mg.get_block_size()
        );

        // if target file is not in the map
        if !flag{
            // create std::fs::File to map
            mg.set_file(block.get_filename());
        }
        let file = mg.get_file_mut(block.get_filename()).unwrap();

        let mut p = page.lock().unwrap();
        let buf = p.contents();

        // read bytes from file starting at given offset to buffer
        Ok(file.read_at(buf, block_size * block.get_block_number())?)
    }


    pub fn write(manager:&std::sync::Mutex<Self>, block:&BlockId, page:&std::sync::Mutex<Page>) -> Result<usize, Box<dyn std::error::Error>>{
        let mut mg = manager.lock().unwrap();
        let (flag, block_size) = (
            mg.exists(block.get_filename()), 
            mg.get_block_size()
        );
         // if target file is not in the map
         if !flag{
            // create std::fs::File to map
            mg.set_file(block.get_filename());
        }
        let file = mg.get_file_mut(block.get_filename()).unwrap();


        let mut p = page.lock().unwrap();
        // write bytes from buffer to file starting from a given offset
        Ok(file.write_at(p.contents(), block.get_block_number() * block_size)?)
    }

    pub fn block_num(&self, filename:&str)->Result<u64, Box<dyn std::error::Error>>{
        let file = match self.get_file(filename) {
            Some(f) =>{
                f
            },
            _ =>{
                return Err("Cant find target file from map. Please use set_file firstly!".into());
            }
        };

        Ok(file.metadata()?.len()/self.block_size)
    }

    pub fn append(&mut self, filename:&str) -> Result<BlockId, Box<dyn std::error::Error>>{
        let block_num = self.block_num(filename)?;
        let block_size = self.get_block_size();

        // block number starts from 0
        let block = BlockId::new(filename, block_num as u64);
        let file = match self.exists(filename){
            true =>{
                self.get_file_mut(filename).unwrap()
            }

            false =>{
                self.set_file(filename)?;
                self.get_file_mut(filename).unwrap()
            }
        };
        
        let buf:Vec<u8> = vec![0; block_size as usize];
        file.write_at(&buf, block.get_block_number() * block_size)?;
        Ok(block)

    }
}