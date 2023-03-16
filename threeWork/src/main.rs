fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr = vec![5, 2, 7, 1, 9, 4];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut arr2 = vec!["apple", "banana", "pear", "orange"];
    bubble_sort(&mut arr2);
    println!("Sorted array: {:?}", arr2);
}
