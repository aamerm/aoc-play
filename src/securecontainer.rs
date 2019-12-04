
pub fn password_finder() {
    let input = "153517-630395";
    let mut result: u32 = 0;

    let mut split_input: Vec<&str> = input.split("-").collect();
    let start: u32 = split_input[0].parse::<u32>().unwrap();
    let end: u32 = split_input[1].parse::<u32>().unwrap();
    println!("{}", start);
    println!("{}", end);

    for value in start..end {
        if never_decrease(value) && has_double(value) {
            println!("Possible password is: {} ", value);
            result += 1;
        }
    }

//    println!("{}", never_decrease(123789));
//    println!("{}", has_double(123789));

    println!("Count of possible passwords: {}", result);
}

fn never_decrease(value: u32) -> bool {
    let vec_number: Vec<u32> = string_number_to_vec(value);
    for i in 0..vec_number.len()-1 {
        if vec_number[i] > vec_number[i+1] {
            return false;
        }
    }
    true
}

fn has_double(value: u32) -> bool {
    let vec_number: Vec<u32> = string_number_to_vec(value);
    for i in 0..vec_number.len()-1 {
        if vec_number[i] == vec_number[i+1] {
            return true;
        }
    }
    false

}

//fn number_to_vec(n: u32) -> Vec<u32> {
//    let mut digits = Vec::new();
//    let mut n = n;
//    while n > 9 {
//        digits.push(n % 10);
//        n = n / 10;
//    }
//    digits.push(n);
//    digits.reverse();
//    digits
//}

fn string_number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
