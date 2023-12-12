pub fn solve(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        // Game 1: 8 green, 6 blue, 20 red
        let game: Vec<_> = line.split(':').collect();

        // Split by ;
        let all_sets: Vec<_> = game[1].split(';').map(Cubes::from).collect();

        // Find max red, green, blue
        let max_red = all_sets.iter().map(|c| c.red).max().unwrap();
        let max_green = all_sets.iter().map(|c| c.green).max().unwrap();
        let max_blue = all_sets.iter().map(|c| c.blue).max().unwrap();

        let power = max_red * max_green * max_blue;

        sum += power;
    }
    sum
}

#[derive(PartialEq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    #[allow(dead_code)]
    fn new(red: u32, green: u32, blue: u32) -> Cubes {
        Cubes { red, green, blue }
    }
}

impl From<&str> for Cubes {
    fn from(s: &str) -> Cubes {
        // Parse: " 8 green, 6 blue, 20 red"
        // "1 green, 3 red, 6 blue"
        let vals: Vec<_> = s.split(',').map(|s| s.trim()).collect();

        // search for red
        let red = if let Some(red_pos) = vals.iter().position(|s| s.ends_with("red")) {
            vals[red_pos]
                .split(' ')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        } else {
            0
        };

        // search for green
        let green = if let Some(green_pos) = vals.iter().position(|s| s.ends_with("green")) {
            vals[green_pos]
                .split(' ')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        } else {
            0
        };

        // search for blue
        let blue = if let Some(blue_pos) = vals.iter().position(|s| s.ends_with("blue")) {
            vals[blue_pos]
                .split(' ')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        } else {
            0
        };

        Cubes { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = " 8 green, 6 blue, 20 red";
        let cubes = Cubes::from(input);
        assert_eq!(cubes.red, 20);
        assert_eq!(cubes.green, 8);
        assert_eq!(cubes.blue, 6);
    }

    #[test]
    fn example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(solve(input), 2286);
    }

    #[test]
    fn line_1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"#;
        assert_eq!(solve(input), 48);
    }

    #[test]
    fn line_2() {
        let input = r#"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"#;
        assert_eq!(solve(input), 12);
    }

    #[test]
    fn line_3() {
        let input = r#"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"#;
        assert_eq!(solve(input), 1560);
    }

    #[test]
    fn line_4() {
        let input = r#"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"#;
        assert_eq!(solve(input), 630);
    }

    #[test]
    fn line_5() {
        let input = r#"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(solve(input), 36);
    }
}
