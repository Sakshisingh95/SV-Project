



//error code of ownership 
// fn main() {
//     // S1 is the owner of the String
//     let mut s1 = String::from("S1 is my owner");
    
//     // S2 takes ownership of the data from s1
//    let mut s2: String = s1;
    
//     // Now s1 is no longer valid since its ownership has been transferred to s2
//     // s2 can mutate the data
//     s2.push_str(" and I am mutable");

//     // Trying to print s2 is fine since s2 is the current owner of the data
//     println!("s2 = {}", s2);

//     // This line will cause a compile-time error because s1 is no longer valid
//     // since its ownership has been transferred to s2
//     println!("s1= {}", s1);
// }



//fix code 
fn main() {
    // S1 is the owner of the String
    let mut s1 = String::from("S1 is my owner");

    // S2 takes a reference to the data from s1
    let s2 = &mut s1;

    // Now s1 is still valid since its ownership hasn't been transferred to s2
    // s2 can't mutate the data
    s2.push_str(" and I am not mutable");

    // Trying to print s2 is fine since s2 is a reference to the original data
    println!("s2 = {}", s2);

    // You can also print s1 since its ownership hasn't been transferred
    println!("s1 = {}", s1);
}
