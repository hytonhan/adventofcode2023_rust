advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = {
        input.lines()
             .map(|line| line.chars()
                             .filter_map(|character| character.to_digit(10))
                             .collect::<Vec<u32>>())
             .map(|foo| foo.first().unwrap() * 10 + foo.last().unwrap())
             .sum()
    };
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum2: u32 = {
        input.lines()
             .map(|line| line.replace("one", "one1one")
                                   .replace("two", "two2two")
                                   .replace("three", "three3three")
                                   .replace("four", "four4four")
                                   .replace("five", "five5five")
                                   .replace("six", "six6six")
                                   .replace("seven", "seven7seven")
                                   .replace("eight", "eight8eight")
                                   .replace("nine", "nine9nine")
                                   .chars()
                                   .filter_map(|character| character.to_digit(10))
                                   .collect::<Vec<u32>>())
             .map(|foo| foo.first().unwrap() * 10 + foo.last().unwrap())
             .sum()
    };
    Some(sum2)
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
