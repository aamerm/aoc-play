
pub fn password_finder() {
    let input = "153517-630395";
    let mut result: u32 = 0;

    let mut split_input: Vec<&str> = input.split("-").collect();
    let start: u32 = split_input[0].parse::<u32>().unwrap();
    let end: u32 = split_input[1].parse::<u32>().unwrap();
    println!("{}", start);
    println!("{}", end);

    for value in start..end {
        if never_decrease(value) && has_double(value) && has_strict_double(value) {
            println!("Possible password is: {} ", value);
            result += 1;
        }
    }

//    println!("{}", never_decrease(123789));
//    println!("{}", has_double(123789));
//    println!("{}", has_strict_double(112233));
//    println!("{}", has_strict_double(123444));
//    println!("{}", has_strict_double(111122));
//    println!("{}", has_strict_double(588889));

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

fn has_strict_double(value: u32) -> bool {
    let vec_number: Vec<u32> = string_number_to_vec(value);

    let mut i = 0;
    let mut done = false;
    while !done {
        let mut curr_count = 1;
        for j in i+1..vec_number.len() {
            if vec_number[i] == vec_number[j] {
                curr_count += 1;
            } else if vec_number[i] != vec_number[j] {
                i = j;
                break;
            }

            if j == vec_number.len()-1 {
                i = j;
                break;
            }
        }
        if curr_count == 2 {
            return true;
        }
        if i >= vec_number.len()-1 {
            done = true;
        }
    }
    return false;
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
