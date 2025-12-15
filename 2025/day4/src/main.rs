const INPUT: &str = include_str!("input.txt");

fn main() {
    let result_1 = get_rolls_count(INPUT);
    println!("PART 1: {result_1}");
}

enum Field {
    PaperRoll,
    Empty,
}

struct Lines {
    lines: Vec<Vec<Field>>,
}
impl Lines {
    fn get(&self, x: usize, y: usize) -> Option<&Field> {
        self.lines.get(y).and_then(|line| line.get(x))
    }

    fn is_accessible(&self, x: usize, y: usize) -> bool {
        let mut number_of_adjacent_rolls = 0;
        for i in -1_i64..=1_i64 {
            for j in -1_i64..=1_i64 {
                let new_x = (x as i64 + i) as usize;
                let new_y = (y as i64 + j) as usize;
                if let Some(Field::PaperRoll) = self.get(new_x, new_y)
                    && (i != 0 || j != 0)
                {
                    number_of_adjacent_rolls += 1;
                    if number_of_adjacent_rolls > 3 {
                        // we can exit early
                        return false;
                    }
                }
            }
        }
        true
    }

    // TODO define a function to remove paper roll
}

fn get_rolls_count(input: &str) -> u64 {
    let fields: Vec<Vec<Field>> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.bytes()
                .map(|b| match b {
                    b'@' => Field::PaperRoll,
                    b'.' => Field::Empty,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let num_lines = fields.len();
    let num_columns = fields.first().map(|s| s.len()).unwrap();

    let lines = Lines { lines: fields };
    let mut num_rolls = 0;
    for y in 0..num_lines {
        for x in 0..num_columns {
            if let Some(Field::PaperRoll) = lines.get(x, y)
                && (lines.is_accessible(x, y))
            {
                num_rolls += 1;
            }
        }
    }

    num_rolls
}

#[cfg(test)]
mod tests {
    use crate::get_rolls_count;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");
    const EXAMPLE_RESULT_1: u64 = 13;
    // const EXAMPLE_RESULT_2: u64 = 3121910778619;
    //

    #[test]
    fn example_test() {
        assert_eq!(EXAMPLE_RESULT_1, get_rolls_count(EXAMPLE_INPUT))
    }
}
