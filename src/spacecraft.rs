use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

//fn read_input_commandline() -> io::Result<()> {
//    let mut args = std::env::args();
//    args.next();
//    for arg in args {
//        let lines = file_to_vec(arg)?;
//        println!("{:?}", lines);
//    }
//
//    Ok(())
//}

fn read_from_file(filename: String) -> io::Result<Vec<i64>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let input_vector: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let result = input_vector.iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    return Ok(result);
}

pub fn spacecraft() {
    //mass of each module - inputlist
    //fuel for each module - for each item in list, divide by 3 round down, subtract 2 - inputlist.map and return newlist
    //totalfuel - newlist.reduce

    let file_path = String::from("src/resources/input_day1.txt");
    let module_mass = match read_from_file(file_path) {
        Ok(data) => data,
        Err(err) => Vec::new()
//        Err(err) => println!("err {}", err)
    };

//    let module_mass: Vec<i64> = vec![12, 14, 1969, 100756];
    fuel_counter_upper(module_mass);
}

fn fuel_counter_upper(module_mass: Vec<i64>) {
//    let v = u.iter().map(|&x| x + 1).collect::<Vec<_>>();
    let module_fuel = module_mass.into_iter().map(calculate_fuel).collect::<Vec<_>>();
    let total_fuel: i64 = module_fuel.iter().sum();

//    print_vector(module_fuel);
    println!("{}", total_fuel);
}

fn calculate_fuel(mut mass: i64) -> i64 {
    let mut done = false;
    let mut result: i64 = 0;
    while !done {
        mass = (mass / 3) - 2;
        if mass > 0 {
            result += mass;
        } else {
            done = true;
        }
    }
    return result;
}

//fn print_vector(v: Vec<i64>) {
//    for x in v.iter() {
//        println!("vec contained {}", x);
//    }
//}
