func findMissingRepeatingNumbers(a []int) [2]int {
    n := len(a)
    xr := 0
    for i := 0; i < n; i++ {
        xr ^= a[i]
        xr ^= i + 1
    }
    number := xr & ^(xr - 1)
    zero, one := 0, 0
    for i := 0; i < n; i++ {
        if a[i]&number != 0 {
            one ^= a[i]
        } else {
            zero ^= a[i]
        }
    }
    for i := 1; i <= n; i++ {
        if i&number != 0 {
            one ^= i
        } else {
            zero ^= i
        }
    }
    for i := 0; i < n; i++ {
        if a[i] == zero {
            return [2]int{zero, one}
        }
    }
    return [2]int{one, zero}
}

