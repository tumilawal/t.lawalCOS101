use std::io;
use std::io::Read;

fn main() {
    println!("\nEnter your role");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Enter the correct Role");
    let input1 = input1.trim();

    if input1 == "Administrator" {
        let mut file = std::fs::File::open("globacom_backup.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if input1 == "Project Manager" {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if input1 == "Employee" {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if input1 == "Customer" {
        let mut file = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if input1 == "Vendor" {
        let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else {
        println!("Invalid role inputed");
    }
}


