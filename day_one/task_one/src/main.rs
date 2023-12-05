use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {

    let mut total_count = 0;

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let digits: String = ip.chars()
                    .filter(|c| c.is_digit(10))
                    .collect();
                if digits.len() == 1 {
                    // In this scenario, use it twice
                    let mut digit  = String::from(digits);
                    digit.push_str(digit.clone().as_str());
                    // Now store this total count
                    let num: i32 = digit.parse().expect("Not a valid integer");
                    total_count += num;
                }else {
                    // get first char
                    let mut first = digits.chars().next().unwrap().to_string();
                    // get last char
                    let last = digits.chars().last().unwrap().to_string();

                    first.push_str(&last);
                    let num: i32 = first.parse().expect("Not a valid integer");
                    total_count += num;
                }
            }
        }
    }
    println!("{}", total_count);
    // loop through each line and find first and last integer and combine to two digits

}
