fn main() {
  let v = vec![101,250,330,400];
  // vector v own sobject in heap
  let v2 = v;
  // here 2 variables own one heap value 
  // it is not allowed
  println!("{:?}",v2);
}
