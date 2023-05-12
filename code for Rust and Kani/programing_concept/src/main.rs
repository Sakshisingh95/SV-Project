
fn main() {
    //S1 is owner of hello
    let s1 = String::from ("S1 is my owner");
    // S2 is referncing s1 means there is 1 copy and may varibale can reference to that location
    let s2 = &s1;
    println!("s2 = {}", s2);
    //now s3 is borrowing from s1
    let s3 = &s1;
    println!("s3 = {}", s3);
   // S4 is referencing to s2
   let s4 = &s2 ;
   println!("s4 = {}, with len={}", s4 , s4.len());
}
