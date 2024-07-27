public int[] findMissingRepeatingNumbers(int[] a) {
    int n = a.length;
    int xr = 0;
    for (int i = 0; i < n; i++) {
        xr ^= a[i];
        xr ^= (i + 1);
    }
    int number = (xr & ~(xr - 1));
    int zero = 0, one = 0;
    for (int i = 0; i < n; i++) {
        if ((a[i] & number) != 0) {
            one ^= a[i];
        } else {
            zero ^= a[i];
        }
    }
    for (int i = 1; i <= n; i++) {
        if ((i & number) != 0) {
            one ^= i;
        } else {
            zero ^= i;
        }
    }
    for (int i = 0; i < n; i++) {
        if (a[i] == zero) {
            return new int[] {zero, one};
        }
    }
    return new int[] {one, zero};
}

