fn factorial(num: u32) -> u32 {
    if num == 0 || num == 1 {
        return 1;
    }

    let mut result = num;

    for i in (1..num).rev() {
        result = result * i;
    }

    return result;
}
