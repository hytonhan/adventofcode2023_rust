advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let all_ok = line
                .split_once(": ")
                .unwrap()
                .1
                .split(&[',', ';'][..])
                .map(|foo| {
                    let parts = foo.trim().split_once(' ').unwrap();
                    parts.0.parse::<u32>().unwrap()
                        <= match parts.1 {
                            "red" => max_red,
                            "green" => max_green,
                            "blue" => max_blue,
                            _ => unreachable!(),
                        }
                })
                .all(|item| item == true);
            if all_ok {
                let foo2 = line
                    .split_once(": ")
                    .unwrap()
                    .0
                    .split_once(' ')
                    .unwrap()
                    .1
                    .parse::<u32>()
                    .unwrap();
                return Some(foo2);
            }
            None
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for cube in line
                .split_once(": ")
                .unwrap()
                .1
                .split(&[',', ';'][..])
                .enumerate()
            {
                let parts = cube.1.trim().split_once(' ').unwrap();
                let number = parts.0.parse::<u32>().unwrap();
                match parts.1 {
                    "red" => {
                        if number > red {
                            red = number;
                        }
                    }
                    "green" => {
                        if number > green {
                            green = number;
                        }
                    }
                    "blue" => {
                        if number > blue {
                            blue = number;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            red * green * blue
        })
        .sum();
    Some(sum)
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
