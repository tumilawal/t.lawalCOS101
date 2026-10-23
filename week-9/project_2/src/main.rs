use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("File could not be created");
    let mut student_name:Vec<(String)> = Vec::new();
    student_name.push(String::from("Oluchi Mordi"));
    student_name.push(String::from("Adams Aliyu"));
    student_name.push(String::from("Shania Bolade"));
    student_name.push(String::from("Adekunle Gold"));
    student_name.push(String::from("Bianca Edemoh"));

    let mut matric_number:Vec<(String)> = Vec::new();
    matric_number.push(String::from("ACC10211111"));
    matric_number.push(String::from("ECO10110101"));
    matric_number.push(String::from("CSC10228828"));
    matric_number.push(String::from("EEE11020202"));
    matric_number.push(String::from("MEE10202001"));

    let mut department:Vec<(String)> = Vec::new();
    department.push(String::from("Accounting"));
    department.push(String::from("Economics"));
    department.push(String::from("Computer"));
    department.push(String::from("Electrical"));
    department.push(String::from("Mechanical"));

    let mut level:Vec<(String)> =Vec::new();
    level.push(String::from("300"));
    level.push(String::from("100"));
    level.push(String::from("200"));
    level.push(String::from("200"));
    level.push(String::from("100"));

    writeln!(file, "Student Name,Matric Number,Department,Level").unwrap();
    writeln!(file, "{},{},{},{}", student_name[0], matric_number[0], department[0], level[0]).unwrap();
    writeln!(file, "{},{},{},{}", student_name[1], matric_number[1], department[1], level[1]).unwrap();
    writeln!(file, "{},{},{},{}", student_name[2], matric_number[2], department[2], level[2]).unwrap();
    writeln!(file, "{},{},{},{}", student_name[3], matric_number[3], department[3], level[3]).unwrap();
    writeln!(file, "{},{},{},{}", student_name[4], matric_number[4], department[4], level[4]).unwrap();

      println!("File has been succesfully created!");

}

  

