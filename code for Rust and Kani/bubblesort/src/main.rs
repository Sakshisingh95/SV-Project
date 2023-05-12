fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
    let mut n = arr.len();

    while swapped {
        swapped = false;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                let temp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = temp;
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn main() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array is: {:?}", arr);

    bubble_sort(&mut arr);
    println!("Sorted array is: {:?}", arr);
}

