use std::fs;

fn main() {
    // Constants for the maximum colour cubes
    const MAX_RED: i32 = 12;
    const MAX_BLUE: i32 = 14;
    const MAX_GREEN: i32 = 13;

    let input_string = fs::read_to_string("resources/part1/input.txt").expect("Unable to read file");
    let input_vec: Vec<&str> = input_string.split("\n").collect();

    let mut possible_games: Vec<i32> = Vec::new();
    let mut power_sum: i32 = 0;

    for line_num in 0..input_vec.len() {
        // Extract the red items from the string
        let game: Vec<&str> = input_vec[line_num].trim().split(":").collect();
        let game_id: i32 = game[0].trim().split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let bags: Vec<&str> = game[1].split(";").collect();
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green = 0;
        let mut power: i32 = 0;
        for bag in bags {
            let cubes: Vec<&str> = bag.trim().split(",").collect();
            for cube in cubes {
                let count: i32 = cube.trim().split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
                let colour = cube.trim().split(" ").collect::<Vec<&str>>()[1];
                if (colour == "red" && count > num_red) {
                    num_red = count;
                }
                if (colour == "blue" && count > num_blue) {
                    num_blue = count;
                }
                if (colour == "green" && count > num_green) {
                    num_green = count;
                }
            }
        }
        if (num_green <= MAX_GREEN && num_blue <= MAX_BLUE && num_red <= MAX_RED) {
            possible_games.push(game_id);
        }
        power = num_red * num_blue * num_green;
        power_sum += power;
    }

    println!("Possible games: {:?}", possible_games);
    let summed_games: i32 = possible_games.iter().sum();
    println!("Summed games: {} {}", possible_games.iter().sum::<i32>(), summed_games);
    println!("Summed power: {}", power_sum);
}
