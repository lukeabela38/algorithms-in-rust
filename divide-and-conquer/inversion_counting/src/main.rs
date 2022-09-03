fn main() {
    let a = [5, 4, 2, 3, 1, 0];
    let count = count_inversion(&a);
    println!("Inversions: {count}");
}

fn count_inversion(arr: &[usize]) -> usize {

    let a_len = arr.len();
    let mut count = 0;

    for i in 0..a_len {
        for j in i..a_len {
            if arr[j] < arr[i] {
                count += 1;
            }
        }
    }

    count
}