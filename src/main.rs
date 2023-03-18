fn main() {
    use virtual_machine::mem::Memory;
    let mut mem = Memory::new(1);
    mem.write_u8(0,10);
    println!("{}",mem.read_u8(0));
    println!("{}",mem.read_u16(0));
    println!("{}",mem.read_u32(0));
    println!("{}",mem.read_u64(0));
}
