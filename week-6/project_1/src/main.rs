 use std::io;
fn main() {
    let mut choice = String::new();
    println!("Enter your choice");
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();

    //To make our selection
    let(food_name, price) = match choice{
        "P" =>("Poundo yam/Edikainko soup", 3200),
        "F" =>("Fried Rice & Chicken", 3000),
        "A" =>("Amala and Ewedu soup", 2500),
        "E" =>("Eba & Egusi Soup", 2000),
        "W" =>("White Rice & Stew", 2500),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    let mut quantity = String::new();
    println!("Enter the quantity you want");
    io::stdin().read_line(&mut quantity).expect("Failed to understand your input");
    let quantity:f32=quantity.trim().parse().expect("Failed to receive input");
    // to calculate the total price
    let total = price as f32 * quantity;
     // to calculate the discount
      let final_price = if total >10000.0{
        let discount = total * 0.05 ;
        total  - discount
     }
     else{
        total 
     };

     println!("Item: {}",food_name );
     println!("Total price: N{}",final_price );
}
