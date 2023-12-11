mod registers;

enum Instruction {
    ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct CPU {}

impl CPU {
    // fn execute(&mut self, instruction: Instruction) {}
    // fn add(&mut self, value: u8) -> u8 {}
}

fn main() {
    println!("Hello, Gamebivo!");
}
