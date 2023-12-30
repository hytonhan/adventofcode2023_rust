advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines()
         .map(|line| {
            let parts = line.split_once(": ").unwrap().1.split_once(" | ");
            let winners = parts.unwrap().0.trim().split_whitespace().map(|number| number.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let mut value = 0;
            parts.unwrap()
                 .1
                 .trim()
                 .split_whitespace()
                 .for_each(|number| {
                    if winners.contains(&number.trim().parse::<u32>().unwrap()) {
                        if value == 0 {
                            value = 1;
                        }
                        else { 
                            value *= 2;
                        }
                    }
                 });
            value
         })
         .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let winners = input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap().1.split_once(" | ");
            let winners = parts
                .unwrap()
                .0
                .trim()
                .split_whitespace()
                .map(|number| number.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let mut value = 0;
            parts
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .for_each(|number| {
                    if winners.contains(&number.trim().parse::<u32>().unwrap()) {
                        value += 1;
                    }
                });
            value
        })
        .collect::<Vec<usize>>();

    let mut amounts: Vec<usize> = vec![1; 216];

    (0..amounts.len()).for_each(|i| {
        let wins = winners.get(i).unwrap();
        for j in 1..=*wins {
            if i + j == amounts.len() {
                break;
            }
            amounts[i + j] = amounts.get(i + j).unwrap() + amounts.get(i).unwrap();
        }
    });

    let mut sum = 0;
    (0..amounts.len()).for_each(|i| {
        sum += amounts[i];
    });

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
