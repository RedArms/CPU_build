//8bit register cpu
struct CPU{
    reg_a:u8,
    reg_b:u8,
    reg_c:u8,
    reg_d:u8,
    ram : [u8; 1024]
}

fn main() {
    let mut ram : [u8; 1024] = [0x00;1024];

    let cpu = CPU{
        reg_a:0,
        reg_b:0,
        reg_c:0,
        reg_d:0,
        ram:ram
    };
    
    // memory of 1024 x 1 byte (8 bits) = 1kb inited at 0x00
    // so adresses go from $00 to $3FF (can be stored on 10 bits)
    println!("{}",ram[0]);
    ram[0] = 0xFF;
    println!("{}",ram[0]);

    println!("Hello, world the true one!");
}
