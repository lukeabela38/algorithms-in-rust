fn main() {
    
    let a = [6, 5, 4, 3, 2, 1];

    let mut b = a;
    selection_sort(&mut b);

    println!("Selection Sort");
    for i in 0..a.len() {
        let x = b[i];
        println!("{x}");
    }

    let mut c = a;
    bubble_sort(&mut c);

    println!("Bubble Sort");
    for i in 0..a.len() {
        let x = c[i];
        println!("{x}");
    }

    let d = merge_sort(a.to_vec());

    println!("Merge Sort");
    for i in 0..d.len() {
        let x = d[i];
        println!("{x}");
    }

    unsafe {
        println!("Inversion: {INVERSIONS}");
    }

}

fn selection_sort(arr: &mut [usize]) {

    let a_len = arr.len();

    for i in 0..a_len
    {

        let mut minimum: usize = usize::pow(2, 64 - 1);
        let mut min_pos: usize = 0;

        for j in i..a_len {

            if arr[j] < minimum {
                minimum = arr[j];
                min_pos = j;
            }
           
        }

        arr.swap(i, min_pos);

    }

}

fn bubble_sort(arr: &mut [usize]) {

    let a_len = arr.len();

    for i in 0..a_len{

        for j in 0..(a_len - i - 1) {

            if arr[j] > arr[j + 1] {

                arr.swap(j, j+1)
            
            }
        }
    }

}

fn merge_sort(vec: Vec<usize>) -> Vec<usize> {

    let a_vec = vec.len();

    if a_vec > 1 {

        let mut vector = Vec::new();

        let middle = a_vec/2;
        let left = &vec[..middle];
        let right = &vec[middle..];

        let left = merge_sort(left.to_vec());
        let right = merge_sort(right.to_vec());

        let mut i = 0;
        let mut j = 0;

        let left_len = left.len();
        let right_len = right.len();

        while i < left_len && j < right_len {
            if left[i] < right[j] {
                vector.push(left[i]);
                i += 1;
            } else {
                vector.push(right[j]);
                j += 1;
            }
        }

        while i < left_len {
            vector.push(left[i]);
            i += 1;

        }

        while j < right_len {
            vector.push(right[j]);
            j += 1;
        }

        return vector;
    }

    vec
}