use std::collections::HashMap;

advent_of_code::solution!(3);

struct PartNumber {
    number: u32,
    index: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input.chars().collect::<Vec<char>>();
    let first_line = input.lines().next().unwrap();
    let width: isize = (first_line.len() + 1) as isize;

    let mut total_sum: u32 = 0;
    for i in 0..map.len() {
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

        let check: bool = (-width-2..-width + length)
            .chain([-1, length])
            .chain(width..width + length+2)
            .any(|x| {
                map.get(i.wrapping_add(x as usize)).unwrap_or(&'0').is_ascii_punctuation()
                    && map.get(i.wrapping_add(x as usize)).unwrap_or(&'0') != &'.'
            });
        if check {
            total_sum += foo.parse::<u32>().unwrap();
        }
    }
    Some(total_sum)
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
