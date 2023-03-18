fn main() {
    use virtual_machine::reg::Reg;
    let mut buffer:[u8;8] = [0;8];
    let mut reg = Reg::new(&mut buffer,8);
    reg.write(1024);
    for i in 0..8 {
        print!("{}",reg.read_byte(i));
    }
    println!("\n{}",reg.read());
}
