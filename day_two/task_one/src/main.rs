use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct Game {
    number: i32,
    blue: i32,
    red: i32,
    green: i32,
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn delimiter_test(c: char) -> bool {
    c == ';' || c == ','
}

fn main() {

    let mut game_results: Vec<Game> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        // Map the lines to an object
        for line in lines {
            let mut game_red = 0;
            let mut game_blue = 0;
            let mut game_green = 0;
            let line_copy = line.unwrap().clone();
            let game_raw: Vec<&str> = line_copy.split(":").collect();
            // keep a reference to how long the split string is
            let game_raw_len = game_raw.first().unwrap().len();
            let game_num_raw: Vec<&str> = game_raw.first().unwrap().split(" ").collect();
            // game number
            let game_num: i32 = game_num_raw.last().unwrap().to_string().parse().expect("unable to parse int");
            //println!("{:?}", game_num);

            // remove game_raw_len from the beginning of line_copy
            let (line_first,line_last) = &line_copy.split_at(game_raw_len +1);

            let split_lint = line_last.split(delimiter_test);
            for v in split_lint {

                let trimmed = v.trim();
                let numerical_raw:Vec<&str> = trimmed.split(" ").collect();
                let number: i32 = numerical_raw.first().unwrap().parse().expect("unable to parse int");

                // find the colour
                match numerical_raw.last().unwrap() {
                    &"green" => {
                        if number > game_green {
                            game_green = number;
                        }
                    },
                    &"red" => {
                        if number > game_red {
                            game_red = number;
                        }
                    },
                    &"blue" => {
                        if number > game_blue {
                            game_blue = number;
                        }
                    },
                    _ => {}
                }
            }
            let game = Game{
                number: game_num,
                green: game_green,
                blue: game_blue,
                red: game_red,
            };
            game_results.push(game);
        }
    }
    /*
    Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
     13 green cubes, and 14 blue cubes.
    What is the sum of the IDs of those games?
     */
    let test_red = 12;
    let test_blue = 14;
    let test_green = 13;
    let mut possible_games = vec![];
    for game in game_results {
        if game.blue <= test_blue && game.green <= test_green && game.red <= test_red
        {
            possible_games.push(game);
        }else {
            println!("Game {} not possible r{}b{}g{}", game.number,game.red,game.blue,game.green);
        }
    }
    // total count
    let mut tcount = 0;
    for pgame in possible_games {
        tcount += pgame.number;
    }
    println!("{}", tcount);
}
