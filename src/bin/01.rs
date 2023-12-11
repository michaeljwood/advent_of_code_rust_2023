advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n")
        .filter( |s| !s.is_empty())
        .map(|s| s.chars().filter(|c| c.is_digit(10)))
        .map(|s| s.collect::<String>())
        .map(|s| format!("{}{}", s.chars().nth(0).unwrap_or('0'), s.chars().nth(s.len() - 1).unwrap_or('0')))
        .map(|s| s.parse::<u32>().unwrap())
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_strings: Vec<&str> = vec!("one", "two", "three", "four", "five", "six", "seven", "eight", "nine");

    Some(input.split("\n")
        .filter( |s| !s.is_empty())
        .map(|s| {
            let mut first_digit = usize::MAX;
            let mut first_num = '0';
            let mut last_digit = 0;
            let mut last_num = '0';
            let owned_string = s.to_owned();
            for i in 0..s.len() {
                if s.chars().nth(i).unwrap_or(' ').is_digit(10) {
                    if i < first_digit {
                        first_digit = i;
                        first_num = s.chars().nth(i).unwrap_or('0');
                    }

                    if i >= last_digit {
                        last_digit = i;
                        last_num = s.chars().nth(i).unwrap_or('0');
                    }
                }

                for (pos, num_string) in num_strings.iter().enumerate() {
                    if owned_string[i..].starts_with(num_string) {
                        if i < first_digit {
                            first_digit = i;
                            first_num = (pos + 1).to_string().chars().nth(0).unwrap();
                        }

                        if i >= last_digit {
                            last_digit = i;
                            last_num = (pos + 1).to_string().chars().nth(0).unwrap();
                        }
                    }
                };
            }
            format!("{}{}", first_num, last_num)
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
