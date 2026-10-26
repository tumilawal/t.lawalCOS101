struct Laptop {
    Brand: String,
    Qty:i32,
    price:i32,
}
impl Laptop{
    fn total_cost(&self,)->i32{
        self.price * 3
    }
}
fn main(){
    let lap1 = Laptop{
        Brand:String::from("HP"),
        Qty:10,
        price:650_000
    };
    let lap2 = Laptop{
        Brand:String::from("IBM"),
        Qty:6,
        price:755_000,
         };
    let lap3 = Laptop{
        Brand:String::from("Toshiba"),
        Qty:10,
        price:550_000,
           };
    let lap4 = Laptop{
        Brand:String::from("Dell"),
        Qty:4,
        price:850_000,
    
        };
        let tc1 =lap1.total_cost();
        let tc2 = lap2.total_cost();
        let tc3 = lap3.total_cost();
        let tc4 = lap4.total_cost();
          // let's call all the three sets of laptop the man has purchased from each brand Batches
        println!("Individual cost for the 1st Batch is : {}",tc1);
         println!("Individual cost for the 2nd Batch is : {}",tc2);
         println!("Individual cost for the 3rd Batch is : {}",tc3);
         println!("Individual cost for the 4th Batch is : {}",tc4);
          
          let grand_total_cost = tc1 + tc2 + tc3 + tc4;

         println!("Grand Total cost the man is paying is : {}",grand_total_cost );

        

       

}
