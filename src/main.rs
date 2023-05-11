fn main() {
    let arr = vec![10, 99, 23, 88];
    println!("{:?}", bubblesort(arr));
}

fn bubblesort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}
