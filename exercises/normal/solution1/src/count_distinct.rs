pub fn new_count_distinct(input_str: &str) -> usize {
    let mut str_vec: Vec<&str> = input_str
        .split(",")
        .collect();

    str_vec.sort();
    str_vec.dedup();

    str_vec.len()
}
