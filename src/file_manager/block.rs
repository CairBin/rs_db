use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug,Hash,Clone)]
pub struct BlockId{
    filename: String,
    block_number: u64,
}

impl BlockId{
    pub fn new(filename:&str, block_number:u64) -> Self{
        Self{
            filename: filename.to_string(),
            block_number: block_number,
        }
    }

    pub fn get_block_number(&self) -> u64{
        self.block_number
    }

    pub fn get_filename(&self) -> &str{
        &self.filename
    }

    pub fn equal(&self, other:&Self) -> bool{
        self.filename == other.filename && self.block_number == other.block_number
    }

    pub fn get_hash_code(&self) -> u64{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}