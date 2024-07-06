use std::io::Write;

#[derive(Debug)]
pub struct Page{
    buffer: Vec<u8>,
}

impl Page{
    pub fn new_as_bytes(bytes:&[u8]) -> Self{
        Self{
            buffer: bytes.to_vec(),
        }
    }

    pub fn new_as_size(size:usize)->Self{
        Self{
            buffer: vec![0;size],
        }
    }

    pub fn to_mutex(self) -> std::sync::Mutex<Self>{
        std::sync::Mutex::new(self)
    }

    pub fn get_u64(&self, offset:usize)->Result<u64, Box<dyn std::error::Error>>{
        let mut bytes_buf:[u8;8] = [0;8]; 
        bytes_buf.clone_from_slice(&self.buffer[offset..offset+8]);
        Ok(u64::from_be_bytes(bytes_buf))
    }

    pub fn set_u64(&mut self, offset:usize, val:u64)->Result<(),Box<dyn std::error::Error>>{
        let buf:[u8;8] = val.to_be_bytes();
        (&mut self.buffer[offset..offset+8]).write(&buf)?;
        Ok(())
    }

    pub fn get_bytes(&self, offset:usize) -> Result<&[u8],Box<dyn std::error::Error>>{
        let len = self.get_u64(offset)? as usize;
        Ok(&self.buffer[offset+8..offset+8+len])

    }

    pub fn set_bytes(&mut self, offset:usize, bytes:&[u8]) -> Result<(), Box<dyn std::error::Error>>{
        let len = bytes.len() as u64;
        let len_buf = len.to_be_bytes();
        (&mut self.buffer[offset..offset+8]).copy_from_slice(&len_buf);
        (&mut self.buffer[offset+8..offset+8+bytes.len()]).copy_from_slice(bytes);
        Ok(())
    }

    pub fn get_string(&self, offset:usize) -> Result<&str, Box<dyn std::error::Error>>{
        let bytes = self.get_bytes(offset)?;
        Ok(std::str::from_utf8(bytes)?)
    }

    pub fn set_string(&mut self, offset:usize, string:&str) -> Result<(), Box<dyn std::error::Error>>{
        let bytes = string.as_bytes();
        self.set_bytes(offset, bytes);

        Ok(())
    }

    /// get buffer's mut reference
    pub fn contents(&mut self)->&mut [u8]{
        &mut self.buffer
    }


    pub fn max_length_for_string(string:&str)->usize{
        let bytes = string.as_bytes();
        return 8 + bytes.len();
    }

}