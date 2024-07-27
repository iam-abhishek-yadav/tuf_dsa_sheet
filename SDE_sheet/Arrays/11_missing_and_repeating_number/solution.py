def findMissingRepeatingNumbers(a):
    n = len(a)
    xr = 0
    for i in range(n):
        xr ^= a[i]
        xr ^= (i + 1)
    number = xr & ~(xr - 1)
    zero, one = 0, 0
    for i in range(n):
        if (a[i] & number) != 0:
            one ^= a[i]
        else:
            zero ^= a[i]
    for i in range(1, n + 1):
        if (i & number) != 0:
            one ^= i
        else:
            zero ^= i
    for i in range(n):
        if a[i] == zero:
            return [zero, one]
    return [one, zero]

