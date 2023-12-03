use std::{fs, collections::{VecDeque, HashMap}, env};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let part_two = env::var("part").unwrap_or_default().eq("part2");
    let mut running_sum = 0;

    for line in input.lines() {
        let id_end = line.find(":").unwrap();
        let id = &line[5..id_end].parse::<i32>().unwrap();

        let line = &line[id_end + 2..];
        let parts = line.split("; ");

        let mut possible = true;
        for part in parts {
            println!("{}", part);
            let mut counts = HashMap::from(
                [("red", 0), ("green", 0), ("blue", 0)]);
            let parts = part.split(", ");
            for part in parts {
                let mut parts: std::str::Split<'_, &str> = part.split(" ");
                let count = parts.next().unwrap().parse::<i32>().unwrap();
                let color = parts.next().unwrap();

                *counts.get_mut(color).unwrap() += count;

                if counts.get("red").unwrap() > &12 || counts.get("green").unwrap() > &13 || counts.get("blue").unwrap() > &14 {
                    println!("{} impossible", id);
                    possible = false;
                    break;
                }
            }
        }
        if possible {
            running_sum += id;
        }
    }
    println!("{}", running_sum);
}