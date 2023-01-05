fn main() {
    // Variables can be defined let
    let x = 5;
    println!("The value of x is {}.", x);

    // By default variables are immutable.
    // That means value can't be changed later on.
    // To make a variable mutable, use 'mut' keyword
    let mut x = 1;
    println!("The value (original) of x is {}.", x);
    x = 2;
    println!("The value (changed) of x is {}.", x);

    // Constants can be defined using 'const' keyword
    const PI: f32 = 3.14;
    println!("The value of PI is {}.", PI);

    // Datatypes and hints can be defined using : during definition
    // Integers have following variations support
    // - signed - i8, i16, i32, i64, i128
    // - unsigned - u8, u16, u32, u64, u128
    let _x: i8 = -128;
    let _x: i16 = 255;
    let _x: u8 = 255;

    // Binary, Hexadecimals and Octals are stored as ints default to i32
    let _x = 0b1111_1111;
    let _x = 0xff;
    let _x = 0o377;
    let _x = 02_55;

    // Floats have 32 bit and 64 bit precisions. Default is f64
    let _x: f32 = 1.0;
    let _x: f64 = 1.0;
    let _x = 1.0;

    // Booleans
    let _x: bool = false;
    let _x: bool = true;

    // Bytes
    let _byte_a = b'A';

    // Characters
    let _x = 'c';

    // String pointer
    let _x: &str = "Hello world!";
}
