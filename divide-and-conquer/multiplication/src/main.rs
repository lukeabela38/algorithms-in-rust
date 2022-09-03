fn main() {
    let result = karatsuba(12345, 6789);
    println!("{}", result);
}

// find number of characters in a number
fn size_base10(x: usize) -> usize {
    let x_string: String = x.to_string();
    x_string.chars().count()
}

// divide string into upper and lower
fn split_string(x: usize, m: usize) -> (usize, usize) {
    let x_string: String = x.to_string();
    let x_string_len: usize = x_string.chars().count();

    let x0: usize = x_string[0..(x_string_len - m)].parse().unwrap();
    let x1: usize = x_string[(x_string_len - m)..x_string_len].parse().unwrap();

    (x0, x1)
}

fn karatsuba(x: usize, y: usize) -> u32 {
    if x < 10 || y < 10 {
        (x * y) as u32
    } else {
        let m: usize = std::cmp::min(size_base10(x), size_base10(y)) / 2;

        let tuple = split_string(x, m);
        let high1 = tuple.0;
        let low1 = tuple.1;

        let tuple = split_string(y, m);
        let high2 = tuple.0;
        let low2 = tuple.1;

        let z0 = karatsuba(low1, low2);
        let z1 = karatsuba(low1 + high1, low2 + high2);
        let z2 = karatsuba(high1, high2);

        let m: u32 = m.try_into().unwrap();

        z2 * 10_u32.pow(m * 2) + (z1 - z2 - z0) * 10_u32.pow(m) + z0

    }
}
