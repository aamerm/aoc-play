//use std::fs::File;
//use std::io::{self, BufRead};
//use std::path::Path;
//
//fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//    where
//        P: AsRef<Path>,
//{
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file).lines())
//}

pub fn spacecraft() {
    //mass of each module - inputlist
    //fuel for each module - for each item in list, divide by 3 round down, subtract 2 - inputlist.map and return newlist
    //totalfuel - newlist.reduce

//   lines_from_file("input_day1.data");
    let module_mass: Vec<i64> = vec![12, 14, 1969, 100756];

    fuel_counter_upper(module_mass);
}

fn fuel_counter_upper(module_mass: Vec<i64>) {

//    let v = u.iter().map(|&x| x + 1).collect::<Vec<_>>();
    let module_fuel = module_mass.into_iter().map(calculate_fuel).collect::<Vec<_>>();
    let total_fuel: i64 = module_fuel.iter().sum();

    print_vector(module_fuel);
    println!("{}", total_fuel);
}

fn calculate_fuel(mass: i64) -> i64 {
    (mass/3)-2
}

fn print_vector(v: Vec<i64>) {
    for x in v.iter() {
        println!("vec contained {}", x);
    }
}
