fn main() {
    // i32 means that the value could contains the sign negative
    let x: i32 = -0o400;

    // u32 can only be positive
    let y: u32 = 10_999;

    // all number operations
    let z: f64 = 13.13 * 15.15 + 7.7 / 2.8 % 9.9;

    let t: bool = true;

    // print value inside string
    println!("The value of 'x' is: {x}, y is: {y}");

    println!("Value of z is: {z} and {t}");

    // it's a tuple that is similar to an array,
    // but here we can have multiple types of variables
    let tup: (f64, bool, char, &str) = (13.13, false, 'H', "Rust");

    let (float, boolean, character, string) = tup;
    println!("{character}, {float}, {boolean}, {string}");

    let float = tup.0;

    println!("{float}");

    // here are an array being declared
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // this code is equals to 'let a = [3, 3, 3, 3, 3];'
    let _a = [3; 5];

    {
        let x = x * 2;

        println!("The value of x in the inner scope is: {x}")
    }

    println!("And now the value of x is: {x}");

    const TESTE_VALUE: u32 = 10 * 10;

    println!("The const value is: {TESTE_VALUE}");

    // the same var can be string and int
    let spaces = "   ";
    println!("{spaces}");

    // here are the var being used as int
    let spaces = spaces.len();
    println!("{spaces}");
}
