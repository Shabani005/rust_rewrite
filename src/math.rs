pub fn abs_i32(number: i32) -> i32{
    return number.abs()
}

pub fn abs_i64(number: i64) -> i64{
    return number.abs()
}

pub fn abs_f32(number: f32) -> f32{
    return number.abs()
}

pub fn abs_f64(number: f64) -> f64{
    return number.abs()
}


fn factorial(n: u64) -> f64 {
    if n == 0 {
        1.0
    } else {
        (n as f64) * factorial(n - 1)
    }
}

fn power(base: f64, exponent: u64) -> f64 {
    if exponent == 0 {
        1.0
    } else {
        base * power(base, exponent - 1)
    }
}

pub fn custom_exp(x: f64) -> f64 {
    let mut sum = 0.0;
    let mut n = 0;
    let tolerance = 1e-15; // Define a small tolerance for convergence

    loop {
        let term = power(x, n) / factorial(n);
        sum += term;

        // Stop when the absolute value of the current term is very small
        if term.abs() < tolerance {
            break;
        }

        n += 1;
    }

    sum
}
