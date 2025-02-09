pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // Split the input string to get the number and its base
    let parts: Vec<&str> = num_str.strip_suffix(")").unwrap().split('(').collect();
    if parts.len() != 2 {
        panic!("Invalid input format");
    }
    let number_str = parts[0];
    let from_base_str = parts[1].trim_end_matches(')');
    let from_base: u32 = from_base_str.parse().expect("Invalid base");

    // Convert the number from its original base to decimal
    let decimal_number = u32::from_str_radix(number_str, from_base).expect("Invalid number");

    // Convert the decimal number to the target base
    if !(2..=36).contains(&to_base) {
        panic!("Base must be between 2 and 36");
    }
    let mut result = String::new();
    let mut n = decimal_number;
    while n > 0 {
        let remainder = n % to_base;
        let digit = if remainder < 10 {
            (b'0' + remainder as u8) as char
        } else {
            (b'a' + (remainder - 10) as u8) as char
        };
        result.push(digit);
        n /= to_base;
    }
    if result.is_empty() {
        result.push('0');
    }
    result.chars().rev().collect()
}
