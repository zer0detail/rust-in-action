fn main() {
    // 0b prefix to declare a base 2 (binary) number
    let three = 0b11;

    // 0o to declare an octal (base 8) number
    let thirty = 0o36;

    // 0x to declare a hex (base 16) number
    let three_hundred = 0x12c;

    println!("Base 10: {} {} {}", three, thirty, three_hundred);
    println!("Base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("Base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("Base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
