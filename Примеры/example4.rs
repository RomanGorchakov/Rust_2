fn main() {
    let a = 0b1100;
    let b = 0b1010;

    println!("a И b: {:04b}", a & b); // 1000
    println!("a ИЛИ b: {:04b}", a | b); // 1110
    println!("a XOR b: {:04b}", a ^ b); // 0110
    println!("НЕ a: {:04b}", !a);       
    // зависит от разрядности
    println!("a << 1: {:04b}", a << 1); // 11000
    println!("a >> 2: {:04b}", a >> 2); // 0011
}