fn main() {
	let _names=["Toshiba","Mac","Hp","Dell","Acer"];
	let qty=[2,1,3,3,1];
	let amount=[450_000.0,1500_000.0,750_000.0,2850_000.0,250_000.0];
	//calculating the sum and average
	let total_amount: f64 = amount.iter().sum();
	let total_qty: u32 = qty.iter().sum();
	let avg_per_unit = total_amount / (total_qty as f64);
	println!("Total Sales: {:.2}", total_amount);
println!("Total Quantity Sold: {}", total_qty);
println!("Average Price per Unit: {:.2}", avg_per_unit);
}