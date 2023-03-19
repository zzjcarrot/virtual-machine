use crate::reg::Reg;

const CPU_BITS:usize = 64;

#[derive(Debug)]
pub struct CPU<'a> {
    al:Reg<'a>,ah:Reg<'a>,bl:Reg<'a>,bh:Reg<'a>,
    cl:Reg<'a>,ch:Reg<'a>,dl:Reg<'a>,dh:Reg<'a>,
    ax:Reg<'a>,bx:Reg<'a>,cx:Reg<'a>,dx:Reg<'a>,
    cr1:Reg<'a>,
    rflags:Reg<'a>,
    ebp:Reg<'a>,edi:Reg<'a>,
    rax:Reg<'a>,rbx:Reg<'a>,rsp:Reg<'a>,rip:Reg<'a>,
}

impl CPU<'_> {
    pub fn new() -> Self {
        static BUFFER:[u8;CPU_BITS] = [0;CPU_BITS];
        Self {
            al:Reg::new(&(BUFFER[0..1]),1),
            ah:Reg::new(&(BUFFER[1..2]),1),
            bl:Reg::new(&(BUFFER[2..3]),1),
            bh:Reg::new(&(BUFFER[3..4]),1),
            cl:Reg::new(&(BUFFER[4..5]),1),
            ch:Reg::new(&(BUFFER[5..6]),1),
            dl:Reg::new(&(BUFFER[6..7]),1),
            dh:Reg::new(&(BUFFER[7..8]),1),
            ax:Reg::new(&(BUFFER[0..2]),2),
            bx:Reg::new(&(BUFFER[2..4]),2),
            cx:Reg::new(&(BUFFER[4..6]),2),
            dx:Reg::new(&(BUFFER[6..8]),2),
            cr1:Reg::new(&(BUFFER[8..16]),8),
            rflags:Reg::new(&(BUFFER[16..24]),8),
            ebp:Reg::new(&(BUFFER[24..28]),4),
            edi:Reg::new(&(BUFFER[28..32]),4),
            rax:Reg::new(&(BUFFER[32..40]),8),
            rbx:Reg::new(&(BUFFER[40..48]),8),
            rsp:Reg::new(&(BUFFER[48..56]),8),
            rip:Reg::new(&(BUFFER[56..64]),8),
        }
    }
    pub fn to_string(&self) -> String {
        format!("{:?}",self)
    }
}
