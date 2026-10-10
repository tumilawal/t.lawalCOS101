use std::io;
fn main() {
    loop {
    println!("\n ==SELECT YOUR DESIRED SHAPE!!");
    println!(" 1. Trapezium (area)");
    println!(" 2. Rhombus (area)");
    println!(" 3. Parrallelogram (area)");
    println!(" 4. Cube (Surface area)");
    println!("5.  Cylinder (Volume)");
    println!("6. Exit");

   let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim(){
            "1" => calculate_trapezium(),
            "2" => calculate_rhombus(),
            "3" => calculate_parallelogram(),
            "4" => calculate_cube(),
            "5" => calculate_cylinder(),
            "6" =>{
                println!("Exiting program");
                break;
            }
            _ => println!("Invalid option, Please select 1-6"),

        }

        fn calculate_trapezium() {
    println!("\n-- Trapezium Area --");
    
    let height = read_number("Enter height: ");
    let base1 = read_number("Enter base 1: ");
    let base2 = read_number("Enter base 2: ");

    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {:.2}", area);
}

fn calculate_rhombus() {
    println!("\n-- Rhombus Area --");
    
    let diagonal1 = read_number("Enter diagonal 1: ");
    let diagonal2 = read_number("Enter diagonal 2: ");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {:.2}", area);
}

fn calculate_parallelogram() {
    println!("\n-- Parallelogram Area --");
    
    let base = read_number("Enter base: ");
    let altitude = read_number("Enter altitude: ");

    let area = base * altitude;
    println!("Area of Parallelogram = {:.2}", area);
}

fn calculate_cube() {
    println!("\n-- Cube Surface Area --");
    
    let side = read_number("Enter side length: ");

    let surface_area = 6.0 * side * side;
    println!("Surface Area of Cube = {:.2}", surface_area);
}

fn calculate_cylinder() {
    println!("\n-- Cylinder Volume --");
    
    let radius = read_number("Enter radius: ");
    let height = read_number("Enter height: ");

    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of Cylinder = {:.2}", volume);
}
fn read_number(prompt: &str) -> f64 {
    use std::io::{self, Write};
    
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse::<f64>().unwrap_or(0.0)
}
}




}
