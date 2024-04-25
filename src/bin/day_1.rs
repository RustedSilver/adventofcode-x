use std::{fs, usize};

const NUMBERS: &'static [(&str, &str, char, usize)] = &[
    ("one", "eno", '1', 1),
    ("two", "owt", '2', 2),
    ("three", "eerht", '3', 3),
    ("four", "ruof", '4', 4),
    ("five", "evif", '5', 5),
    ("six", "xis", '6', 6),
    ("seven", "neves", '7', 7),
    ("eight", "thgie", '8', 8),
    ("nine", "enin", '9', 9),
];

fn get_first_find_index(data: &str) -> (usize, char) {
    let mut min_index = 10000;
    let mut value: char = '0';

    for number in NUMBERS {
        if let Some(num_index) = data.find(number.0) {
            if min_index > num_index {
                min_index = num_index;
                value = number.2;
            }
        }
    }

    (min_index, value)
}

fn get_lart_find_index(data: &str) -> (usize, char) {
    let mut min_index = 10000;
    let mut value: char = '0';

    for number in NUMBERS {
        if let Some(num_index) = data.find(number.1) {
            if min_index > num_index {
                min_index = num_index;
                value = number.2;
            }
        }
    }

    if value == '0' {
        return (0, value);
    }

    (min_index, value)
}

fn count_numbers() -> usize {
    let mut count = 0;

    fs::read_to_string("assets/input")
        .unwrap()
        .lines()
        .for_each(|line| {
            println!("Line: {}", line);
            let mut start: char = ' ';
            let mut last: char = '0';

            let first_num = line.char_indices().find(|x| x.1.is_numeric());
            let last_num = line
                .chars()
                .rev()
                .collect::<String>()
                .char_indices()
                .find(|x| x.1.is_numeric());

            if first_num.is_some() {
                let (firs_part, _) = line.split_at(first_num.unwrap().0);
                let found_first = get_first_find_index(firs_part);
                let rev_line: String = line.chars().rev().collect();
                // let (_, lp) = rev_line.split_at(last_num.unwrap().0);

                let found_sec = get_lart_find_index(&rev_line);
                if first_num.unwrap().0 < found_first.0 {
                    start = first_num.unwrap().1;
                } else {
                    start = found_first.1;
                }

                if found_sec.1 == '0' {
                    last = last_num.unwrap().1;
                } else {
                    if found_sec.0 < last_num.unwrap().0 {
                        last = found_sec.1;
                    } else {
                        last = last_num.unwrap().1;
                    }
                }
            } else {
                println!("No numeric chars found");
                let found_first = get_first_find_index(&line);
                let found_second = get_lart_find_index(&line.chars().rev().collect::<String>());

                if found_first.0 > 0 {
                    start = found_first.1;
                }

                if found_second.0 > 0 {
                    last = found_second.1;
                }
            }
            println!("Found Numbers are: {}{}", start, last);

            let total: usize = format!("{}{}", start, last).parse().unwrap();
            count += total;
        });
    count
}

fn main() {
    let count = count_numbers();
    println!("Count: {}", count);
}

#[cfg(test)]
mod test {
    use crate::{count_numbers, get_first_find_index, get_lart_find_index};

    #[test]
    fn find_substr_works() {
        let data = "tszjsxbsixtwoeight62ffjtdnxxtwofive";

        let found = get_first_find_index(data);
        assert_eq!('6', found.1);

        let last = get_lart_find_index(&data.chars().rev().collect::<String>());
        println!("IS::: {:?}", last);
        assert_eq!('5', last.1);
    }

    #[test]
    fn rev() {
        let data = "5onesixsevenphxtmlqhzfcjxrknpv"
            .chars()
            .rev()
            .collect::<String>();
        assert_eq!('7', get_lart_find_index(&data).1);
    }

    #[test]
    fn main_boom() {
        let count = count_numbers();

        assert_eq!(56324, count);
    }
}
