use std::fs;

struct RowComponent {
    value: String,
    symbol_locations: Vec<usize>
}

struct Symbol {
    value: char,
    pos: Vec<usize>,
    parts: Vec<i32>
}

fn get_components_in_row(row: &str) -> Vec<RowComponent> {
    let mut components = Vec::new();
    let mut last = 0;
    for (index, separator) in row.match_indices(|c: char| !(c.is_alphanumeric())) {
        if last != index {
            let mut start_position = last;
            if start_position != 0 {
                start_position -= 1;
            }
            let mut end_position = index;
            if end_position != row.len() {
                end_position += 1;
            }
            let row_component = RowComponent {
                value: row[last..index].to_string(),
                symbol_locations: (start_position..end_position).collect()
            };
            components.push(row_component);
        }
        // Generate the row component for the separator
        components.push(RowComponent {
            value: separator.to_string(),
            symbol_locations: Vec::new()
        });
        last = index + separator.len();
    }

    if last < row.len() {
        components.push(RowComponent {
            value: row[last..].to_string(),
            symbol_locations: (last - 1 .. row.len()).collect()
        });
    }

    // Remove the last character as it is always a carriage return
    components.pop();

    // Calculate the positions of each of the components
    return components;
}

fn is_part_valid(row: &str, pos: usize) -> bool {
    if pos >= row.len() - 1 || pos < 0 {
        return false;
    }
    let pos_to_check = row.chars().nth(pos).unwrap();
    if !(pos_to_check.is_alphanumeric() || pos_to_check == '.') {
        return true;
    }
    return false;
}

fn main() {
    let input_string = fs::read_to_string("resources/part1/input.txt").expect("Unable to read file");
    let input_vec: Vec<&str> =input_string.split("\n").collect();

    let mut valid_parts: Vec<i32> = Vec::new();
    let mut validator_symbols: Vec<Symbol> = Vec::new();
    let mut valid_gears: Vec<i32> = Vec::new();
    // Iterate through the input vector, each iteration is a new line
    '_rows: for row in 0..input_vec.len() {
        println!("{}", input_vec[row]);
        // Split the row by it's separator to get the composite parts
        let separated_row = get_components_in_row(input_vec[row]);
        '_components: for component in separated_row {
            // We only produce symbol locations for valid parts
            if component.symbol_locations.len() > 0 {
                // Check the corresponding symbol locations in relative rows to search for valid parts
               '_inner: for loc in component.symbol_locations {
                    if row > 0 {
                        // Check the row above
                        if is_part_valid(input_vec[row - 1], loc) {
                            println!("Found valid part: {}", component.value);
                            // Check if we already have a validator symbol for this position
                            if validator_symbols.iter().find(|s| s.pos == [loc, row - 1]).is_some() {
                                // Get the position of the matched validator symbol and push to it's part
                                let symbol_pos = validator_symbols.iter().position(|s| s.pos == [loc, row - 1]).unwrap();
                                validator_symbols.get_mut(symbol_pos).unwrap().parts.push(component.value.parse::<i32>().expect("Expected a number"));
                            } else {
                                validator_symbols.push(Symbol {
                                    value: input_vec[row - 1].chars().nth(loc).unwrap(),
                                    pos: vec![loc, row - 1],
                                    parts: vec![component.value.parse::<i32>().expect("Expected a number").clone()]
                                });
                            }
                            valid_parts.push(component.value.parse::<i32>().expect("Expected a number").clone());
                        }
                    }
                    if row < input_vec.len() - 1  {
                        // Check the row below
                        if is_part_valid(input_vec[row + 1], loc) {
                            println!("Found valid part: {}", component.value);
                            // Check if we already have a validator symbol for this position
                            if validator_symbols.iter().find(|s| s.pos == [loc, row + 1]).is_some() {
                                // Get the position of the matched validator symbol and push to it's part
                                let symbol_pos = validator_symbols.iter().position(|s| s.pos == [loc, row + 1]).unwrap();
                                validator_symbols.get_mut(symbol_pos).unwrap().parts.push(component.value.parse::<i32>().expect("Expected a number"));
                            } else {
                                validator_symbols.push(Symbol {
                                    value: input_vec[row + 1].chars().nth(loc).unwrap(),
                                    pos: vec![loc, row + 1],
                                    parts: vec![component.value.parse::<i32>().expect("Expected a number").clone()]
                                });
                            }
                            valid_parts.push(component.value.parse::<i32>().expect("Expected a number").clone());
                        }
                    }
                   // Check the current row
                    if is_part_valid(input_vec[row], loc) {
                        println!("Found valid part: {}", component.value);
                        // Check if we already have a validator symbol for this position
                        if validator_symbols.iter().find(|s| s.pos == [loc, row]).is_some() {
                            // Get the position of the matched validator symbol and push to it's part
                            let symbol_pos = validator_symbols.iter().position(|s| s.pos == [loc, row]).unwrap();
                            validator_symbols.get_mut(symbol_pos).unwrap().parts.push(component.value.parse::<i32>().expect("Expected a number"));
                        } else {
                            validator_symbols.push(Symbol {
                                value: input_vec[row].chars().nth(loc).unwrap(),
                                pos: vec![loc, row],
                                parts: vec![component.value.parse::<i32>().expect("Expected a number").clone()]
                            });
                        }
                        valid_parts.push(component.value.parse::<i32>().expect("Expected a number").clone());
                    }
                }
            }
        }
    }

    // Sort out the gears
    for symbol in validator_symbols {
        if symbol.value == '*' {
            // This is a gear
            // A gear is only valid if there are exactly 2 parts associated with it
            if (symbol.parts.len() == 2) {
                valid_gears.push(symbol.parts[0] * symbol.parts[1]);
            }
        }

    }

    println!("Summed valid parts: {}", valid_parts.iter().sum::<i32>());
    println!("Summed valid gears: {}", valid_gears.iter().sum::<i32>());
}
