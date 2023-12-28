use std::collections::HashMap;

advent_of_code::solution!(3);

struct PartNumber {
    number: u32,
    index: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    // let mut lines = input.lines();
    // let i = 1;
    // let map = include_bytes!("../../data/inputs/03.txt");

    let map = input.chars().collect::<Vec<char>>();
    let first_line = input.lines().next().unwrap();
    let width: isize = (first_line.len() + 1) as isize;

    let mut total_sum: u32 = 0;
    for i in 0..map.len() {
        // for i in 0..width*5 {
        if map[i] == '.' {
            continue;
        }
        if map[i].is_ascii_digit() == false {
            continue;
        }
        if map[i].is_ascii_digit() && map[i - 1].is_ascii_digit() {
            continue;
        }
        let mut length: isize = 0;
        for j in 1..10 {
            if map[i + j].is_ascii_digit() == false {
                length = j as isize;
                break;
            }
        }
        let foo = (i..i + (length as usize))
            .map(|x| map[x])
            .collect::<String>();

        let check: bool = (-width..-width + length)
            .chain([-1, length])
            .chain(width..width + length)
            .any(|x| {
                // Check if parts here
                print!("x: {}", x as usize);
                map.get(x as usize).unwrap_or(&'0').is_ascii_punctuation() && map.get(x as usize).unwrap_or(&'0') != &'.'
            });
            println!("{}", foo);
        if check {
            total_sum += foo.parse::<u32>().unwrap();
            println!("{}", foo);
        }
    }

    // for i in 0..width*10 {
    //     print!("{}", map[i]);
    // }
    // println!("");
    // println!("");
    // println!("");

    // let map2 = input.as_bytes();

    // let total_length = map2.len();
    // println!("{}", total_length);

    // for i in 0..total_length {
    //     if map[i].is_ascii_digit() {
    //         println!("")
    //     }
    // }
    // map2.iter()
    //     .filter(|x| {
    //         true
    //     })
    //     .map(|x| {
    //         println!("{}", x);
    //         1
    //     })
    //     .collect::<Vec<u32>>()
    //     .sum();
    // let first_line = input.lines().next().unwrap();
    // let width = first_line.len();

    // let foo: HashMap<u32, &PartNumber> = HashMap::new();

    // let width = input.lines().next().unwrap().len();
    // let mut prev: &str;
    // let mut current: &str = input.lines().next().unwrap();
    // let mut next: &str = input.lines().next().unwrap();
    // let line_count = input.lines().count();

    // let mut temp_number: String = String::from("");
    // let mut temp_start = 0;
    // for i in 0..line_count {
    //     let mut last_was_numeric = false;
    //     for j in 0..current.chars().count() {
    //         let char = current.chars().nth(j);
    //         if char.unwrap().is_ascii_digit() {
    //             temp_number.push(char.unwrap());
    //             if last_was_numeric == false {
    //                 temp_start = i * width + j;
    //             }
    //             last_was_numeric = true;
    //         }
    //         else if last_was_numeric {
    //             let foo = PartNumber{ number: temp_number.parse::<u32>(), index: temp_start}
    //         }
    //     }

    //     prev = current;
    //     current = next;
    //     if i == line_count- 1 {
    //         next = input.lines().next().unwrap();
    //     }

    // }
    Some(total_sum)
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
