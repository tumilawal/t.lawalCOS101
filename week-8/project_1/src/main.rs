use std::io;
fn main() {
    let office_admin = vec!["Intern","Administrator","Senior Administrator","Office manager","Director","CEO"];
    let aca_demic = vec!["-","Research Assistant","PHD candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let law_yer = vec!["Paralegal","Junior associate","Associate","Senior Asscoiate 1-2","Senior associate 3-4","Partner"];
    let tea_cher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
  
    let aps_level=vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    
    
    println!("\n Enter the category you belong too");
    println!(" 0. Office admin");
    println!(" 1. Academic ");
    println!(" 2. Lawyer");
    println!(" 3. Teacher");

      let mut category = String::new();
    println!("Enter the category you belong to");
    io::stdin().read_line(&mut category).expect("Failed to read line");
    let category = category.trim();




     let selected_vector = match category{
        "0"=> &office_admin,
        "1"=> &aca_demic,
         "2"=> &law_yer,
         "3"=> &tea_cher,
         _ =>{
             println!("Invalid option, Please select 0-4");
             return;

            }

        } ;

     println!("Enter your level index (0-5)");
       let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let level_index : usize = input.trim().parse().expect("Please enter a valid number");
        
        if level_index < selected_vector.len() {
            let grade_band = aps_level[level_index];
            let position = selected_vector[level_index];
            
           
            println!("Your APS Level is: {}", grade_band);
            println!("Your Position is: {}", position);
           
        } 
        else {
            println!("Error: Index out of range! Please choose between 0 and 5.");
        }


}