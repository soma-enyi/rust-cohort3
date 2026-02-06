const X: u32 = 5;
fn main() {
    let mut name = "John";
    println!("Hello, world! {}", X);
    println!("{}", name);

    // let y = x + 23;
    // X = 23;
    name = "Jane";
    println!("{}", name);
    println!(" ------ {}", X);

    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = 5 + 4;
    let x = x * 3;
    println!("{}", x);
}