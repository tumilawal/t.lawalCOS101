fn main() {
    let v = vec![10,20,30];
    // vector owns object in heap
    let v2 = v; //move ownership to v2
    display(v2);
    //v2 moved to display and v2 is invalidated
    println!("In main{:?}",v);

}
fn display(v:&Vec<i32>){
    println!("Inside display{:?}",v);
    return v;
}
