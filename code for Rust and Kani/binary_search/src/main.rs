// Define a function that takes a sorted array and a value to search for
fn binary_search(arr: &[i32], x: i32) -> Option<usize> {
    // Define start index
    let mut start = 0;                           
    let mut end = arr.len() - 1;                   
    while start <= end {                         
        let mid = (start + end) / 2;                
        if arr[mid] == x {                     
            return Some(mid);                
        } else if arr[mid] < x {                 
            start = mid + 1;                        
        } else {                                   
            end = mid - 1;                          
        }
    }

    None                                           
}

fn main() {
    let arr = [1, 6, 5, 7, 9, 11, 24, 56,11]; 
                     // Define a sorted array to search
    let x = 5;                                      // Define a value to search for

    match binary_search(&arr, x) {
        Some(i) => println!("{} found at index {}", x, i),
        None => println!("{} not found in array", x),
    }
}
