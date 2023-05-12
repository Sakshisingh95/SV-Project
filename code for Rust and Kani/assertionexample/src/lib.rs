fn divide(a: i32, b: i32) -> i32 {
    assert!(b != 0, "Division by zero");
   assert!(a != std::i32::MIN);
   assert!(b != std::i32::MIN);
   assert!(a != std::i32::MAX);
   assert!(b != std::i32::MAX);
    a / b
}
#[cfg(kani)]
#[kani::proof]
fn verify_divide() {
    let a: i32 = kani::any();
    let b: i32 = kani::any();
    kani::assume(b != 0);
    kani::assume(a != std::i32::MIN);// Exclude the smallest i32 value
    kani::assume(b != std::i32::MIN);
    kani::assume(a != std::i32::MAX);
    kani::assume(b != std::i32::MAX);
   // let result: i32 = divide(a, b);
   divide(a,b);

    
}


