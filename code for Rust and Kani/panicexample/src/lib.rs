
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Div by zero");
        }
        a / b
    }
    #[cfg(kani)]
#[kani::proof]
fn verify_divide() {
    let a: i32 = kani::any();
    let b: i32 = kani::any();
    divide(a,b);
   
    }
