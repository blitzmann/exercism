const MAX_SIZE: usize = 1_000_005;

fn sieve(vec: &mut Vec<u32>) {
    let mut is_prime = [true; MAX_SIZE];

    let mut p = 2;
    while p * p < MAX_SIZE {
        if is_prime[p] {
            let mut i = p * p;
            while i < MAX_SIZE {
                is_prime[i] = false;
                i += p
            }
        }

        p += 1;
    }

    // Store all prime numbers
    let mut p = 2;
    while p < MAX_SIZE {
        if is_prime[p] {
            vec.push(p as u32);
        }
        p += 1;
    }
}

pub fn nth(n: u32) -> u32 {
    let mut vec: Vec<u32> = Vec::new();

    sieve(&mut vec);

    vec[n as usize]
}
