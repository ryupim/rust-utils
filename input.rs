fn input_number() -> usize {
    let mut number_str = String::new();
    io::stdin().read_line(&mut number_str).ok();
    let number = number_str.trim().parse().ok().unwrap();
    return number;
}

