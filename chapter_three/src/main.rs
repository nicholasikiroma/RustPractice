fn main() {

    // const X: u32 = 5;

    // TOPIC: Mutability
    // let mut x = 5;
    // println!("The value of x is: {x}");
  
    // x = 6;
    // println!("The value of x is: {x}");

    // TOPIC: Shadowing

    let x = 5;
    let x = x  + 1;

    {
        let x = x * 3;
        println!("The value of x in the inner scope is: {x}")
    }

    println!("The value of x is: {x}")
  }