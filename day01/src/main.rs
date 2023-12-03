use std::{fs, collections::VecDeque, env};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let part_two = env::var("part").unwrap_or_default().eq("part2");
    let mut running_sum = 0;

    for line in input.lines() {

        let mut fixed_line = line.to_string();
        if part_two {
            fixed_line = line.replace("one", "o1e").replace("two", "t2.").replace("three", "e3e")
            .replace("four", "4.").replace("five", "5e").replace("six", "6.")
            .replace("seven", "7.").replace("eight", "e8.").replace("nine", "9e");
        }

        let mut numbers: VecDeque<char> = VecDeque::new();
        for c in fixed_line.chars() { 
            if c.is_numeric() {
                numbers.push_back(c);
            }
        }
        let mut number_str = numbers.pop_front().unwrap().to_string();
        let char_value_second = numbers.pop_back();
        if !char_value_second.is_none() {
            number_str += &char_value_second.unwrap().to_string();
        } else {
            number_str = number_str.repeat(2);
        }

        let int_value : i32 = number_str.parse().unwrap();

        running_sum += int_value;
    }
    println!("{}", running_sum);
}