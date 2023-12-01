use regex::Regex;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);

    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let numbers = extract_numbers(line);
        let num_str = numbers.first().unwrap().to_string() + numbers.last().unwrap().to_string().as_str();
        sum += num_str.parse::<i64>().unwrap();
    }

    return sum;
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let numbers = extract_string_numbers(line);
        let num_str = numbers.first().unwrap().to_string() + numbers.last().unwrap().to_string().as_str();
        sum += num_str.parse::<i64>().unwrap();
    }

    return sum;
}

fn extract_numbers(line: &str) -> Vec<i32> {
    let re = Regex::new(r"\d").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn extract_string_numbers(line: &str) -> Vec<i32> {
    let re = Regex::new(r"(oneight|threeight|fiveight|nineight|eightwo|eighthree|sevenine|twone|\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    re.find_iter(line)
        .flat_map(|m| match m.as_str() {
            "one" => vec![1],
            "two" => vec![2],
            "three" => vec![3],
            "four" => vec![4],
            "five" => vec![5],
            "six" => vec![6],
            "seven" => vec![7],
            "eight" => vec![8],
            "nine" => vec![9],
            "oneight" => vec![1, 8],
            "threeight" => vec![3, 8],
            "fiveight" => vec![5, 8],
            "nineight" => vec![9, 8],
            "eightwo" => vec![8, 2],
            "eighthree" => vec![8, 3],
            "sevenine" => vec![7, 9],
            "twone" => vec![2, 1],
            _ => vec![m.as_str().parse().unwrap()],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
}
