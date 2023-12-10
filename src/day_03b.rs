pub struct Day03b;

impl crate::Solution for Day03b {
    fn solve(&self) -> String {
        format!("{}", solve(include_str!("../inputs/day03")))
    }
}

fn solve(input: &str) -> u32 {
    // Sum of all products between two numbers that are connected by a gear ('*')
    let mut sum = 0;

    // Find all numbers adjacent to the gear
    let mut adjacent_numbers = Vec::with_capacity(2);

    // Iterate over the lines in the input
    for (current_line_index, current_line_content) in input.lines().enumerate() {
        // Determine the index of the lines before and after the gear
        let line_above_index = current_line_index.checked_sub(1);
        let line_below_index = current_line_index + 1;

        // Iterate over the characters in the line and find the gears
        let char_indices_peekable = current_line_content.char_indices().peekable();
        for (index, c) in char_indices_peekable {
            // Skip non-gear characters
            if c != '*' {
                continue;
            }

            // Search for numbers to the left of the gear
            if let Some(number) = current_line_content[..index]
                .char_indices()
                .rev()
                .take_while(|(_, c)| c.is_ascii_digit())
                .last()
                .and_then(|(index, _)| {
                    current_line_content[index..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                })
            {
                adjacent_numbers.push(number)
            }

            // Search for numbers to the right of the gear
            if let Ok(number) = current_line_content[(index + 1)..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
            {
                adjacent_numbers.push(number)
            }

            // Search for numbers above the gear
            if let Some(line_above_index) = line_above_index {
                // Find the left-most digit above the gear
                let numbers_start_index = input
                    .lines()
                    .nth(line_above_index)
                    .and_then(|line| {
                        line[..=index.saturating_sub(1)]
                            .char_indices()
                            .rev()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| i)
                    })
                    .unwrap_or(index.saturating_sub(1));
                // Find the right-most digit above the gear
                let numbers_end_index = input
                    .lines()
                    .nth(line_above_index)
                    .and_then(|line| {
                        line[index.saturating_add(1)..]
                            .char_indices()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| index.saturating_add(2) + i)
                    })
                    .unwrap_or(index.saturating_add(1));

                // Parse all numbers above the gear
                if let Some(line) = input.lines().nth(line_above_index) {
                    line[numbers_start_index..numbers_end_index]
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|number| number.parse::<u32>().ok())
                        .for_each(|number| adjacent_numbers.push(number))
                }
            };

            // Search for numbers below the gear
            {
                // Find the left-most digit below the gear
                let numbers_start_index = input
                    .lines()
                    .nth(line_below_index)
                    .and_then(|line| {
                        line[..=index.saturating_sub(1)]
                            .char_indices()
                            .rev()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| i)
                    })
                    .unwrap_or(index.saturating_sub(1));
                // Find the right-most digit below the gear
                let numbers_end_index = input
                    .lines()
                    .nth(line_below_index)
                    .and_then(|line| {
                        line[index.saturating_add(1)..]
                            .char_indices()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| index.saturating_add(2) + i)
                    })
                    .unwrap_or(index.saturating_add(1));

                // Parse all numbers below the gear
                if let Some(line) = input.lines().nth(line_below_index) {
                    line[numbers_start_index..numbers_end_index]
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|number| number.parse::<u32>().ok())
                        .for_each(|number| adjacent_numbers.push(number))
                }
            }

            // Add the product of the adjacent numbers to the sum if there are exactly two adjacent numbers
            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
            adjacent_numbers.clear();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE), 467835);
    }
}
