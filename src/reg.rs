pub struct Reg<'a>{
    data:&'a mut [u8],
    size:usize,
}

impl Reg<'_> {
    pub fn new(data:&'_ mut [u8], size:usize) -> Reg{
        Reg{data, size}
    }

    pub fn read(&self) -> u64{
        let mut bytes:u64 = 0;
        for i in 0..self.size{
            bytes <<= 8;
            bytes += self.data[i] as u64;
        }
        bytes
    }
    pub fn read_byte(&self,index:usize) -> u8{
        if index > self.size {
            return 0;
        }
        self.data[index]
    }

    pub fn write_byte(&mut self, index:usize, value:u8){
        self.data[index] = value;
    }

    pub fn write(&mut self,value:u64){
        for i in 0..self.size{
            self.data[i] = (value >> ((self.size - i - 1) * 8)) as u8;
        }
    }

    pub fn get_size(&self) -> usize{
        self.size
    }
}

