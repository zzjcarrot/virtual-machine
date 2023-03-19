use crate::mem::Memory;

pub enum MMUMode {
    Paging, //分页
    None,   //无
}

#[derive(Clone, Copy)]
pub struct Page {
    //页描述符(1MB每页)
    pub to: usize,  //映射页号
    pub flags: u64, //标志(0x00只读,0x01读写)
    pub dpl: u64,   //访问特权级最低多少
}

fn get_pages(num: usize) -> Vec<Page> {
    let mut pages = Vec::new();
    for i in 0..num {
        pages.push(Page {
            to: i,
            flags: 0x01,
            dpl: 3,
        });
    }
    pages
}

pub struct PageDir {
    //每一级页表32KB
    pages: Vec<Page>,
}

impl PageDir {
    pub fn new(page_num: usize) -> Self {
        Self {
            pages: get_pages(page_num),
        }
    }
}

pub struct MMU {
    mode: MMUMode,
    page_dir: PageDir,
    memory: Memory,
}

impl MMU {
    pub fn new(mem_size: usize) -> Self {
        Self {
            mode: MMUMode::None,
            page_dir: PageDir::new(mem_size),
            memory: Memory::new(mem_size),
        }
    }
    pub fn set_mode(&mut self,mode: MMUMode) {
        self.mode = mode;
    }
    fn virtual2phisycal(&self, addr: usize) -> usize {
        let page_no = addr / 1024 / 1024;
        let page_to = addr % 1024;
        let page = self.page_dir.pages[page_no];
        page.to*1024*1024 + page_to
    }
    fn get_addr(&self, addr: usize) -> usize {
        match self.mode {
            MMUMode::None => addr,
            MMUMode::Paging => self.virtual2phisycal(addr),
        }
    }
    pub fn read_u8(&mut self, addr: usize) -> u8 {
        self.memory.read_u8(self.get_addr(addr))
    }
    pub fn read_u16(&mut self, addr: usize) -> u16 {
        let mut i = 0;
        let mut bytes = 0;
        while i < 2{
            bytes <<= 8;
            bytes += self.read_u8(self.get_addr(addr + i)) as u16;
            i = i + 1;
        }
        bytes
    }
    pub fn read_u32(&mut self, addr: usize) -> u32 {
        let mut i = 0;
        let mut bytes = 0;
        while i < 4{
            bytes <<= 8;
            bytes += self.read_u8(self.get_addr(addr + i)) as u32;
            i = i + 1;
        }
        bytes
    }
    pub fn read_u64(&mut self, addr: usize) -> u64 {
        let mut i = 0;
        let mut bytes = 0;
        while i < 8{
            bytes <<= 8;
            bytes += self.read_u8(self.get_addr(addr + i)) as u64;
            i = i + 1;
        }
        bytes
    }
    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.memory.write_u8(self.get_addr(addr), value);
    }
    pub fn write_u16(&mut self, addr: usize, value: u16) {
        const SIZE:usize = 2;
        let mut i = 0;
        while i < SIZE{
            self.write_u8(addr+SIZE-i-1,(value >> (i*8)) as u8);
            i += 1;
        }
    }
    pub fn write_u32(&mut self, addr: usize, value: u32) {
        const SIZE:usize = 4;
        let mut i = 0;
        while i < SIZE{
            self.write_u8(addr+SIZE-i-1,(value >> (i*8)) as u8);
            i += 1;
        }
    }
    pub fn write_u64(&mut self, addr: usize, value: u64) {
        const SIZE:usize = 8;
        let mut i = 0;
        while i < SIZE{
            self.write_u8(addr+SIZE-i-1,(value >> (i*8)) as u8);
            i += 1;
        }
    }
}
