use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("categories_of_drinks.csv").expect("Could not create");
   
   let headers = "Lager,Stout,Non-alcoholic\n";
    let column_1 = "33 Export,Legend,Maltina\n";
    let column_2 = "Desperados, Turbo King, Amstel Malta\n";
    let column_3 = "Goldberg, Wiiliams,Malta Gold\n";
    let column_4 = "Gulder, , \n";
    let column_5 = "Heineken,,\n";
    let column_6 = "Star,, \n";


file.write_all(headers.as_bytes()).expect("Failed to display");
file.write_all(column_1.as_bytes()).expect("Failed to display");
file.write_all(column_2.as_bytes()).expect("Failed to display");
file.write_all(column_3.as_bytes()).expect("Failed to display");
file.write_all(column_4.as_bytes()).expect("Failed to display");
file.write_all(column_5.as_bytes()).expect("Failed to display");
file.write_all(column_6.as_bytes()).expect("Failed to display");








println!("The file has been succesfully created");

}
