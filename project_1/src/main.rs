use std::io;
fn main() {
    let mut inputA = String::new();
    let mut inputB = String::new();
    let mut inputC = String::new();

    println!("\n Enter your value for A:");
    io::stdin().read_line(&mut inputA).expect("Wrong value try again");
    let  a:f64 =inputA.trim().parse().expect("Please input again");

    println!("\n Enter your value for B:");
    io::stdin().read_line(&mut inputB).expect("Wrong value try again");
    let b:f64 =inputB.trim().parse().expect("Please input again");

    println!("\n Enter your value for C:");
    io::stdin().read_line(&mut inputC).expect("Wrong value try again");
    let c:f64 = inputC.trim().parse().expect("Please input again");

    let d:f64 = (b.powf(2.0) - 4.0 * a *c).sqrt();
    // E is the variable for the positive function
    let e:f64 = (-b + d) / (2.0*a);
    // F is the variable for the negative function
    let f:f64 = (-b - d)/ (2.0 * a);

    if d > 0.0{
        println!("Your roots are {} and {}", e, f);
    } else if e==f{
        println!("Your root is {}",e);
    }
    else {
        println!("Your root is not real");
    }
}
