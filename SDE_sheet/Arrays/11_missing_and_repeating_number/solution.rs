fn find_missing_repeating_numbers(a: Vec<i32>) -> (i32, i32) {
    let n = a.len();
    let mut xr = 0;
    for i in 0..n {
        xr ^= a[i];
        xr ^= (i + 1) as i32;
    }
    let number = xr & !(xr - 1);
    let mut zero = 0;
    let mut one = 0;
    for i in 0..n {
        if a[i] & number != 0 {
            one ^= a[i];
        } else {
            zero ^= a[i];
        }
    }
    for i in 1..=n {
        if (i as i32) & number != 0 {
            one ^= i as i32;
        } else {
            zero ^= i as i32;
        }
    }
    for i in 0..n {
        if a[i] == zero {
            return (zero, one);
        }
    }
    (one, zero)
}

