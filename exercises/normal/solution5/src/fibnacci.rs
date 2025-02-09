pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    /// get all fib num smaller than threshold, return sum of all odds in it
    let mut sum = 0;
    let (mut a, mut b) = (1, 1);

    while a < threshold {
        if a % 2 != 0 {
            sum += a;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}
