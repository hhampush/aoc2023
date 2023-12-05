use std::{fs, collections::HashMap, env};

const SIZE_X : usize = 141;
const SIZE_Y : usize = 141;

fn matches(two_dim_array : [[char; SIZE_X]; SIZE_Y], x : usize, y: usize) -> bool {
    if !two_dim_array[x][y].is_alphanumeric() && two_dim_array[x][y] != '.' && two_dim_array[x][y] != ' ' {
        return true;
    }
    return false;
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let part_two = env::var("part").unwrap_or_default().eq("part2");
    let mut running_sum = 0;

    let mut two_dim_array: [[char; SIZE_X]; SIZE_Y] = [[' '; SIZE_X]; SIZE_Y];

    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for char in line.chars() {
            two_dim_array[x][y] = char;
            x += 1;
        }
        y += 1;
        x = 0;
    }

    x = 0;
    y = 0;

    while y < SIZE_Y {
        let mut current_number = "".to_string();
        let mut matching = false;
        while x < SIZE_X {
            let c = two_dim_array[x][y];
            if c.is_digit(10) {
                current_number.push(c);
                
                if !matching {
                    if x > 0 && matches(two_dim_array, x-1, y) { matching = true; }
                    else if matches(two_dim_array, x+1, y) { matching = true; }
                    else if y > 0 && matches(two_dim_array, x, y-1) { matching = true; }
                    else if matches(two_dim_array, x, y+1) { matching = true; }
                    else if y > 0 &&  matches(two_dim_array, x+1, y-1) { matching = true; }
                    else if x > 0 && y > 0 &&  matches(two_dim_array, x-1, y-1) { matching = true; }
                    else if matches(two_dim_array, x+1, y+1) { matching = true; }
                    else if x > 0 && matches(two_dim_array, x-1, y+1) { matching = true; }
                }
            } else {
                if !current_number.is_empty() {
                    println!("{} {} {}", current_number, matching, running_sum);

                }

                if matching {
                    running_sum += current_number.parse::<i32>().unwrap_or_default();
                }

                current_number = "".to_string();
                matching = false;
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    println!("{}", running_sum);
}