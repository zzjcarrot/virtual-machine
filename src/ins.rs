pub struct Instruction {
    codes:[u8;4]
}

impl Instruction
{
    pub fn new(codes:[u8;4]) -> Self{
        Self {
            codes
        }
    }
    pub fn get_ins(&self) -> u8 {
        self.codes[0]
    }
    pub fn get_type(&self) -> u8 {
        self.codes[1]
    }
    pub fn get_op1(&self) -> u8 {
        self.codes[2]
    }
    pub fn get_op2(&self) -> u8 {
        self.codes[3]
    }
}

