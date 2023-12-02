use std::fs;
// use fancy_regex::Regex;
use aho_corasick::{AhoCorasick};
const ASCII_ZERO: u8 = 48;

// Helper functions to convert word to digits and vice versa
fn word_to_digit(word_digit: &str) -> Option<&str> {
    // Match against the word with it's equivalent digit
    // If we don't find a match then assume the digit is already converted
    match word_digit {
        "zero" => Some("0"),
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => Some(word_digit)
    }
}

fn digit_to_word(digit: i32) -> Option<String> {
    match digit {
        0 => Some("zero".to_string()),
        1 => Some("one".to_string()),
        2 => Some("two".to_string()),
        3 => Some("three".to_string()),
        4 => Some("four".to_string()),
        5 => Some("five".to_string()),
        6 => Some("six".to_string()),
        7 => Some("seven".to_string()),
        8 => Some("eight".to_string()),
        9 => Some("nine".to_string()),
        _ => Some(digit.to_string())
    }
}

fn main() {
    // Part 1

    println!("\n\n========\nPart 1\n");
    // Read the test_input.txt file for this project
    let input_bytes = fs::read("resources/part1/input.txt").expect("Unable to read file");
    let mut calibration_values: Vec<i32> = Vec::new();
    let mut found_digit: u8 = 0;
    let mut first_digit: u8 = 0;
    for x in input_bytes {
        if (x == 10) {
            // This is the start of a newline
            let calibration_value: String = format!("{}{}", first_digit as char, found_digit as char);
            calibration_values.push(calibration_value.parse().expect("Expected a number"));
            found_digit = 0;
            continue;
        }

        if (x > ASCII_ZERO) {
            let byte_diff = x - ASCII_ZERO;
            // If the byte difference is less than 10 this is a digit
            if (byte_diff <= 10) {
                if (found_digit == 0) {
                    first_digit = x;
                }
                found_digit = x;
            }
        }
    }

    // There should be one digit left to add
    calibration_values.push(format!("{}{}", first_digit as char, found_digit as char).parse().expect("Expected a number"));

    // Sum all the calibration values
    println!("--------------\nSummed Values:");
    let sum: i32 = calibration_values.iter().sum();
    println!("{}", sum);

    // --------------------------------------------------------------------------------
    // Part 2
    // Brilliant, now I can't use bytestreams to evaluate ASCII characters. Regex it is.
    // Turns out regex::Regex doesn't support positional lookaround functionality so we can't even use regex.
    // Using an implementation of the Aho-Corasick algorithm instead

    println!("\n\n========\nPart 2\n");

    // Read the input file as a string
    let input_string = fs::read_to_string("resources/part2/input.txt").expect("Unable to read file");
    println!("{}", input_string);
    // Split the string into a vector for iterating
    let input_vec: Vec<&str> = input_string.split("\n").collect();

    // Use regex to match for the digit strings that we care about
    let mut valid_digits: Vec<String> = Vec::with_capacity(20);
    // Fill the valid digits array
    let mut index = 0;
    let mut current_digit = 0;
    while current_digit < 10 {
        valid_digits.push(current_digit.to_string());
        valid_digits.push(digit_to_word(current_digit).unwrap());
        current_digit += 1;
    }

    // Initialise the pattern matcher
    let ac = AhoCorasick::new(valid_digits).unwrap();

    let mut calibration_values: Vec<i32> = Vec::new();
    for test_string in input_vec {
        let mut matches = vec![];
        for digit_match in ac.find_overlapping_iter(test_string) {
            // Find the match string itself by capturing the substring between the match start and end position
            let digit = &test_string[digit_match.start()..digit_match.end()];
            matches.push((digit_match.pattern(), digit_match.start(), digit_match.end(), digit));
        }

        let first_digit = word_to_digit(matches[0].3).unwrap();
        let last_digit = word_to_digit(matches.last().unwrap().3).unwrap();
        calibration_values.push(format!("{}{}", first_digit, last_digit).parse().expect("Expected a number"));
    }

    // Sum all the calibration values
    println!("--------------\nSummed Values:");
    let sum: i32 = calibration_values.iter().sum();
    println!("{}", sum);

}
