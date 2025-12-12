const INPUT: &str = include_str!("input.txt");

fn main() {
    let part1_result: u32 = INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .map(get_max_for_bank)
        .sum();

    let part2_result: u64 = INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .map(get_max_12_for_bank)
        .sum();

    println!("PART 1: {part1_result}. PART 2: {part2_result}");
}

// get the max pairing for the bank
fn get_max_for_bank(bank: &str) -> u32 {
    let bytes = bank.as_bytes();
    let len = bytes.len();
    if len < 2 {
        0
    } else {
        let initial_pair = (bytes[len - 2], bytes[len - 1]);
        let initial_max = initial_pair.0.max(initial_pair.1);

        let ((max_first, max_second), _) = bytes[..len - 2].iter().rev().fold(
            (initial_pair, initial_max),
            |(first_and_second, max_digit), &current_digit| {
                // update pair if new pair has bigger value
                let (first, second) = if (current_digit, max_digit) > first_and_second {
                    (current_digit, max_digit)
                } else {
                    first_and_second
                };

                ((first, second), max_digit.max(current_digit))
            },
        );

        (u32::from(max_first - b'0') * 10) + u32::from(max_second - b'0')
    }
}
fn get_max_12_for_bank(bank: &str) -> u64 {
    get_max_n_for_bank::<12>(bank)
}

// get the max N for the bank, left to right
fn get_max_n_for_bank<const N: usize>(bank: &str) -> u64 {
    let bytes = bank.as_bytes();
    let len = bytes.len();

    if len < N {
        0
    } else {
        // pick the biggest digit from the available range
        let (result, _) = (0..N).fold(
            (Vec::with_capacity(N), 0),
            |(mut acc, start_pos), position| {
                let remaining_needed = N - position - 1; // how many more digits we need after this one
                let search_end = len - remaining_needed; // last index we can pick from

                // Find the maximum digit in the candidate range
                let (max_pos, max_digit) = (start_pos..search_end).fold(
                    (start_pos, bytes[start_pos]),
                    |(index, max_digit), current_postion| {
                        if bytes[current_postion] > max_digit {
                            (current_postion, bytes[current_postion])
                        } else {
                            (index, max_digit)
                        }
                    },
                );

                acc.push(max_digit);
                (acc, max_pos + 1)
            },
        );

        result
            .iter()
            .fold(0, |acc, digit| (acc * 10) + u64::from(digit - b'0'))
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_max_12_for_bank, get_max_for_bank};

    const EXAMPLE_INPUT: &str = include_str!("example.txt");
    const EXAMPLE_RESULT: u32 = 357;
    const EXAMPLE_RESULT_2: u64 = 3121910778619;

    #[test]
    fn part1_test() {
        assert_eq!(98, get_max_for_bank("987654321111111"));
        assert_eq!(89, get_max_for_bank("811111111111119"));
        assert_eq!(78, get_max_for_bank("234234234234278"));
        assert_eq!(92, get_max_for_bank("818181911112111"));
    }

    #[test]
    fn part1_test_2() {
        assert_eq!(98, get_max_for_bank("987654321111111"));
    }

    #[test]
    fn part1_full() {
        let result: u32 = EXAMPLE_INPUT
            .lines()
            .filter(|s| !s.is_empty())
            .map(get_max_for_bank)
            .sum();
        assert_eq!(EXAMPLE_RESULT, result);
    }

    #[test]
    fn part2_test_1() {
        assert_eq!(987654321111, get_max_12_for_bank("987654321111111"));
        assert_eq!(811111111119, get_max_12_for_bank("811111111111119"));
        assert_eq!(434234234278, get_max_12_for_bank("234234234234278"));
        assert_eq!(888911112111, get_max_12_for_bank("818181911112111"));
    }

    #[test]
    fn part2_full() {
        let result: u64 = EXAMPLE_INPUT
            .lines()
            .filter(|s| !s.is_empty())
            .map(get_max_12_for_bank)
            .sum();
        assert_eq!(EXAMPLE_RESULT_2, result);
    }
    #[test]
    fn test_exact_length() {
        assert_eq!(123456789012, get_max_12_for_bank("123456789012"));
        assert_eq!(999999999999, get_max_12_for_bank("999999999999"));
    }
}
