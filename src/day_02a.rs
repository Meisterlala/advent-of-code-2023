crate::solution!(2, solve, crate::day_02b::solve);

pub fn solve(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        // Game 1: 8 green, 6 blue, 20 red
        let game: Vec<_> = line.split(':').collect();
        let game_index: i32 = game[0].split(' ').last().unwrap().parse().unwrap();

        // Split by ;
        let valid = game[1].split(';').all(|set| {
            let cubes = Cubes::from(set);
            // Check if possible set
            cubes.red <= MAX.red && cubes.green <= MAX.green && cubes.blue <= MAX.blue
        });

        if valid {
            sum += game_index as u32;
        }
    }
    sum
}

#[derive(PartialEq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

static MAX: &Cubes = &Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

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
        assert_eq!(solve(input), 1 + 2 + 5);
    }
}
