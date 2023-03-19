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
            page_dir: PageDir::new(mem_size / 1024 / 1024),
            memory: Memory::new(mem_size),
        }
    }
    fn virtual2phisycal(&self, addr: usize) -> usize {
        let page_no = addr / 1024 / 1024;
        let page_to = addr % 1024;
        let page = self.page_dir.pages[page_no];
        page.to + page_to
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
        self.memory.read_u16(self.get_addr(addr))
    }
    pub fn read_u32(&mut self, addr: usize) -> u32 {
        self.memory.read_u32(self.get_addr(addr))
    }
    pub fn read_u64(&mut self, addr: usize) -> u64 {
        self.memory.read_u64(self.get_addr(addr))
    }
    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.memory.write_u8(self.get_addr(addr), value);
    }
    pub fn write_u16(&mut self, addr: usize, value: u16) {
        self.memory.write_u16(self.get_addr(addr), value);
    }
    pub fn write_u32(&mut self, addr: usize, value: u32) {
        self.memory.write_u32(self.get_addr(addr), value);
    }
    pub fn write_u64(&mut self, addr: usize, value: u64) {
        self.memory.write_u64(self.get_addr(addr), value);
    }
}
