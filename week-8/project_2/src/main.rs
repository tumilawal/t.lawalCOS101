use std::io;
fn main() {
   let mut developers: Vec<(String, i32)> = Vec::new();
    println!("~Welcome to the Experience Checker~");
    println!("Input Number of candidates you want to check for");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input try again");
    let count:usize = input1.trim().parse().expect("Unable to process this");
    // to collect user input
    for i in 0..count{
        println!("\n Entering detail for applicant {}",i + 1 );

    println!("Enter developer's name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim().to_string();

        println!("Enter years of experience:");
        let mut exp_input = String::new(); io::stdin().read_line(&mut exp_input).expect("Failed to read experience");
        let experience: i32 = exp_input
            .trim()
            .parse()
        .expect("Please enter a valid integer for experience!");
         developers.push((name,experience));
      
    }
    
          let mut best_applicant = &developers[0];
   // To check for which candidate (applicant) has the highest years of experience
    for developer in &developers{
        if developer.1 > best_applicant.1{
            best_applicant = developer;
        }
    }
  println!("The best applicant for the job is {} with {} years of experience",best_applicant.0,best_applicant.1);
}
