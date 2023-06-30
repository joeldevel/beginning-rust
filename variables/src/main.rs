const A_CONSTANT: u32 = 10 * 12;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    println!("The value of a constant: {A_CONSTANT}");

    // A_CONSTANT = 1; // this is not possible

    // shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");
}
