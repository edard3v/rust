fn is_cousin(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limit: u32 = (num as f64).sqrt().round() as u32;

    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}
