use std::io;
fn main() {
    println!("\nEnter your age");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Enter the correct Variable");
    let mut num:i32=input1.trim().parse().expect("Enter the correct variable");

    println!("\nAre you experienced(yes/no)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to receive input");
     //This is for the yes/ no part
     let exp_input =input2.trim().to_lowercase();

      if exp_input=="yes" && num >= 40 {
    println!("Your incentive/salary is ₦1,560,000.0");
             }   

    else if exp_input=="yes" && num >= 30 && num <= 39{
    println!("Your incentive/salary is ₦1,480,000.0");
            }  

    else if exp_input=="yes" && num < 28 {
    println!("Your incentive/salary is ₦1,300,000.0");
                            }

        else if  exp_input=="no"{
            println!("Your incentive/salary is ₦100,000.0");
        }

}

 