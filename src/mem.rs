fn alloc_vec(size:usize) -> Vec<u8>{
    let mut v = Vec::new();
    for _ in 0..size{
        v.push(0);
    }
    v
}

pub struct Memory{
    buffer:Vec<u8>,
    size:usize,
}

impl Memory{
    pub fn new(size:usize) -> Self {
        Self {
            buffer:alloc_vec(size*1024),
            size:size*1024,
        }
    }
    pub fn write_u8(&mut self,addr:usize, bytes:u8) {
        if addr >= self.size {
            return;
        }
        self.buffer[addr] = bytes;
    }
    pub fn write_u16(&mut self,addr:usize, bytes:u16){
        if addr >= self.size {
            return;
        }
        let mut i = 0;
        while i < 2{
            self.buffer[addr+i] = (bytes >> (i*8)) as u8;
            i += 1;
        }
    }
    pub fn write_u32(&mut self,addr:usize, bytes:u32){
        if addr >= self.size {
            return;
        }
        let mut i = 0;
        while i < 4{
            self.buffer[addr+i] = (bytes >> (i*8)) as u8;
            i += 1;
        }
    }
    pub fn write_u64(&mut self,addr:usize, bytes:u64){
        if addr >= self.size {
            return;
        }
        let mut i = 0;
        while i < 8{
            self.buffer[addr+i] = (bytes >> (i*8)) as u8;
            i += 1;
        }
    }
    pub fn read_u8(&mut self,addr:usize) -> u8{
        if addr >= self.size {
            return 0;
        }
        return self.buffer[addr];
    }
    pub fn read_u16(&mut self,addr:usize) -> u16{
        if addr >= self.size {
            return 0;
        }
        let mut i = 0;
        let mut bytes = 0;
        while i < 2{
            bytes <<= 8;
            bytes += self.buffer[addr+i] as u16;
            i = i + 1;
        }
        bytes
    }
    pub fn read_u32(&mut self,addr:usize) -> u32{
        if addr >= self.size {
            return 0;
        }
        let mut i = 0;
        let mut bytes = 0;
        while i < 4{
            bytes <<= 8;
            bytes += self.buffer[addr+i] as u32;
            i = i + 1;
        }
        bytes
    }
    pub fn read_u64(&mut self,addr:usize) -> u64{
        if addr >= self.size {
            return 0;
        }
        let mut i = 0;
        let mut bytes = 0;
        while i < 8{
            bytes <<= 8;
            bytes += self.buffer[addr+i] as u64;
            i = i + 1;
        }
        bytes
    }
}

