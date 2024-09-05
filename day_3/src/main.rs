use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::str::Split;

const ADJACENT_COORDS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[derive(Clone, Debug, PartialEq)]
struct Part {
    value: u32,
    digits: Vec<[i32; 2]>,
}

impl Part {
    fn new(value: u32, digits: Vec<[i32; 2]>) -> Part {
        Part { value, digits }
    }
}

fn main() {
    let resources_path: &str = "resources";
    println!("Calculating part 1");
    let part1_path = resources_path.to_string() + "/input.txt";
    let part1_schematic = fs::read_to_string(part1_path).expect("Could not read file");

    let part1_res = calc_part1(part1_schematic.as_str());
    println!("Part 1: {}", part1_res);

    println!("Calculating part 2");
    let part2_res = calc_part2(part1_schematic.as_str());
    println!("Part 2: {}", part2_res);
}

fn test_adjacent_cells(symbols: BTreeMap<[i32; 2], bool>, parts: Vec<[i32; 2]>) -> bool {
    for [x, y] in parts.iter() {
        for [i, j] in ADJACENT_COORDS.iter() {
            if symbols
                .keys()
                .any(|&key| key[0] == x + i && key[1] == y + j)
            {
                return true;
            }
        }
    }

    return false;
}

fn get_symbols_from_schematic(
    schematic: Split<'_, &str>,
    symbol_re: Regex,
) -> BTreeMap<[i32; 2], bool> {
    let mut symbols: BTreeMap<[i32; 2], bool> = BTreeMap::new();

    // Find all symbols in the schematic and store their position
    for (i, line) in schematic.to_owned().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if symbol_re.is_match(c.to_string().as_str()) {
                // This is a symbol so store it's position
                symbols.insert([j as i32, i as i32], true);
            }
        }
    }
    return symbols;
}

fn calc_part1(schematic: &str) -> u32 {
    // Setup control vars
    let mut sum: u32 = 0;
    let symbol_re = Regex::new(r"[^.0-9]").unwrap();

    let schematic_lines = schematic.split("\n");
    let symbol_map = get_symbols_from_schematic(schematic_lines.to_owned(), symbol_re.to_owned());

    // Track the current part value
    let mut part: String = "".to_owned();
    let mut part_pos: Vec<[i32; 2]> = Vec::new();
    for (i, line) in schematic_lines.to_owned().enumerate() {
        // If we have a part value when we start a line, we had a part
        // at the end of the previous line.
        if part != "" {
            if test_adjacent_cells(symbol_map.clone(), part_pos.clone()) {
                sum += part.parse::<u32>().unwrap();
            }
            part = "".to_owned();
            part_pos.clear();
        }
        for (j, c) in line.chars().enumerate() {
            if symbol_re.is_match(c.to_string().as_str()) || c == '.' {
                if part != "" {
                    if test_adjacent_cells(symbol_map.clone(), part_pos.clone()) {
                        sum += part.parse::<u32>().unwrap();
                    }
                }
                // This isn't a part so reset the part
                part = "".to_owned();
                part_pos.clear();
            }

            if c.is_numeric() {
                part += c.to_string().as_str();
                part_pos.push([j as i32, i as i32]);
            }
        }
    }

    return sum;
}

fn calc_part2(schematic: &str) -> u32 {
    let mut sum = 0;

    // We only want to get the gears (denoted with '*')
    let gear_re = Regex::new(r"\*").unwrap();

    let schematic_lines = schematic.split("\n");
    let gear_map = get_symbols_from_schematic(schematic_lines.to_owned(), gear_re.to_owned());

    // Get the positions of all the parts.
    let mut parts: Vec<Part> = Vec::new();
    let mut part_value: String = "".to_owned();
    let mut part_pos: Vec<[i32; 2]> = Vec::new();
    for (i, line) in schematic_lines.to_owned().enumerate() {
        if part_value != "" {
            parts.push(Part::new(
                part_value.parse::<u32>().unwrap(),
                part_pos.to_owned(),
            ));
            part_value = "".to_owned();
            part_pos.clear();
        }

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                part_value += c.to_string().as_str();
                part_pos.push([j as i32, i as i32]);
            } else {
                // This is a non-numeric character so if we have a part value then collect the part
                if part_value != "" {
                    parts.push(Part::new(
                        part_value.parse::<u32>().unwrap(),
                        part_pos.to_owned(),
                    ));
                    part_value = "".to_owned();
                    part_pos.clear();
                }
            }
        }
    }

    // Now we have all the location of gears and part numbers. Compare the gears to part numbers in associated positions.
    for (_, gear) in gear_map.keys().enumerate() {
        let mut adjacent_parts: Vec<Part> = Vec::new();
        for [i, j] in ADJACENT_COORDS.iter() {
            let found_parts: Vec<&Part> = parts
                .iter()
                .filter(|part| {
                    part.digits
                        .iter()
                        .any(|&digit| digit[0] == gear[0] + i && digit[1] == gear[1] + j)
                })
                .collect();

            for found_part in found_parts.iter().copied() {
                if !adjacent_parts.contains(found_part) {
                    adjacent_parts.push(found_part.clone());
                }
            }
        }
        if adjacent_parts.len() == 2 {
            sum += adjacent_parts[0].value * adjacent_parts[1].value;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn example_returns_correct_value_part1() {
        let part1 = calc_part1(EXAMPLE);
        assert_eq!(part1, 4361);
    }

    #[test]
    fn example_returns_correct_value_part2() {
        let part2 = calc_part2(EXAMPLE);
        assert_eq!(part2, 467835);
    }
}
