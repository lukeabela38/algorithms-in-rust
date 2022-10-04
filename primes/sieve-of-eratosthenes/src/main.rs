fn main() {
    let n = 1000;
    let primes = sieve_of_eratosthenes(n);

    for i in 0..n {
        let value = i*primes[i];
        println!("{value}");
    }
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // Declare an array of size n
    let mut prime = vec![1; n + 1];
    let mut p = 2;

    // loop while p squared is not larger than n
    while p * p <= n {
        // if we have assumed a prime value
        if prime[p] == 1 {

            // from p squared to the maximum value
            for i in ((p * p)..(n + 1)).step_by(p) {
                prime[i] = 0;
            }
        }

        p += 1;
    }

    prime
}
