pub fn factors(n: u64) -> Vec<u64> {
    // We start our checks at 2 since 1 cannot be a divisor 
    let mut divisor = 2;
    // Re-define n as mutable (todo: can I do this in the function definition?)
    let mut n = n;
    // define our collection of divisors
    let mut collection: Vec<u64> = Vec::new();

    loop {
        if n % divisor == 0 {
            // cleanly divisible, set n and try again with new number
            n = n / divisor;
            collection.push(divisor);

            if n == 1 {
                // we've reached the end
                break
            }
        }
        else if divisor < n {
            // increase divisor if we have more to do
            divisor += 1;
        } else {
            // handles situation when we must break out of loop because we've reached our n.
            break;
        }

    }

    return collection;
}
