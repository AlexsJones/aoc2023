use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn word_to_number(word: &str) -> Option<i32> {
    match word.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
fn main() {
    let pattern = r"one|two|three|four|five|six|seven|eight|nine|[1-9]";
    let re = Regex::new(pattern).unwrap();
    let mut total_count = 0;

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                let ip_str = ip.as_str();
                let mut findings: Vec<String> = vec![];
                for mat in re.find_iter(ip_str) {
                    findings.push(ip_str[mat.start()..mat.end()].to_string());
                }
                // Add the start and end together
                if findings.len() == 1 {
                    let mut digit  = String::from(findings.first().unwrap());
                    digit.push_str(digit.clone().as_str());
                    let num: i32 = digit.parse().expect("Not a valid integer");
                    total_count += num;
                }else {
                    let first = findings.first().unwrap();
                    let mut first_num = 0;
                    let last = findings.last().unwrap();
                    let mut last_num = 0;
                    match word_to_number(first) {
                        Some(number) => {
                            first_num = number;
                        },
                        None => {
                            let num: i32 = first.parse().expect("Not a valid integer");
                            first_num = num;
                        },
                    }
                    match word_to_number(last) {
                        Some(number) => {
                            last_num = number;
                        },
                        None => {
                            let num: i32 = last.parse().expect("Not a valid integer");
                            last_num = num;
                        },
                    }

                    // Now for the dumb bit
                    let res = format!("{}{}",first_num,last_num);
                    let num: i32 = res.parse().expect("Not a valid integer");
                    total_count += num;
                    println!("From {} found first {} and last {} which combine to make {}", &ip_str, first_num, last_num, num);

                }
            }else {
                panic!()
            }

        }
    }
    println!("{}", total_count);
    // loop through each line and find first and last integer and combine to two digits

}

