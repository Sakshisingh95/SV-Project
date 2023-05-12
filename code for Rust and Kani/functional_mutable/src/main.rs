
 //creating a function which will borrow s1
 fn borrows1(t: &mut String)
 {
    t.push_str("borrowing now");

 }
fn main() {
    //S1 is owner of hello
    let mut s1 = String::from ("S1 is my owner");
    // using borrows function
   borrows1(&mut s1);
    println!("s1= {}", s1);
// }
}
