fn main() {
    use virtual_machine::mmu::*;
    println!("creating MMU.");
    let mut mmu = MMU::new(10);
    println!("start");
    mmu.set_mode(MMUMode::Paging);
    mmu.write_u64(0, 200);
    println!("{}",mmu.read_u64(0));
}
