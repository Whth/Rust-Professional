const CACHES: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];

pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut remaining_amount = amount;
    let mut count = 0;

    for &cache in &CACHES {
        if remaining_amount >= cache {
            count += remaining_amount / cache;
            remaining_amount %= cache;
        }
    }

    if remaining_amount == 0 {
        count
    } else {
        0 // or some indication that it's not possible, depending on requirements
    }
}
