use std::fmt::Display;

crate::solution!(16, solve_a, solve_b);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    East = 4,
    West = 8,
}

#[derive(PartialEq, Eq, Debug)]
enum Mirror {
    Rightward,
    Leftward,
    HorizontalSplitter,
    VerticalSplitter,
    Empty,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Energized {
    data: u8,
}

pub fn solve_a(input: &str) -> u64 {
    let (rest, map) = parse(input).unwrap();
    debug_assert!(rest.is_empty());

    let mut energized = vec![vec![Energized { data: 0 }; map[0].len()]; map.len()];
    traverse(&map, (0, 0), Direction::East, &mut energized);

    // _print_energy(&energized);
    energized
        .into_iter()
        .map(|row| row.into_iter().filter(|e| e.data > 0).count() as u64)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let (rest, map) = parse(input).unwrap();
    debug_assert!(rest.is_empty());

    let north = (0..map[0].len()).map(|y| ((0, y), Direction::South));
    let east = (0..map.len()).map(|x| ((x, map[0].len() - 1), Direction::West));
    let south = (0..map[0].len()).map(|y| ((map.len() - 1, y), Direction::North));
    let west = (0..map.len()).map(|x| ((x, 0), Direction::East));

    // _print_energy(&energized);
    north
        .chain(east)
        .chain(south)
        .chain(west)
        .map(|((x, y), direction)| {
            let mut energized = vec![vec![Energized { data: 0 }; map[0].len()]; map.len()];
            traverse(&map, (x as i32, y as i32), direction, &mut energized);
            energized
                .iter()
                .map(|row| row.iter().filter(|&&e| e.data > 0).count() as u64)
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

/*
   -------------> y
  |  .|...\....
  |  |.-.\.....
  |  .....|-...
  |  ........|.
  |  ..........
  |  .........\
  |  ..../.\\..
  |  .-.-/..|..
 x|  .|....-|.\
 \/  ..//.|....
*/

fn traverse(
    map: &[Vec<Mirror>],
    start: (i32, i32),
    direction: Direction,
    energized: &mut [Vec<Energized>],
) {
    let (x, y) = start;
    // Range check
    if x < 0 || y < 0 || x >= map.len() as i32 || y >= map[0].len() as i32 {
        return;
    }
    let (mut x, mut y) = (x as usize, y as usize);

    // Check if already energized
    if energized[x][y].direction_contains(direction) {
        return;
    }

    //println!("Traversing {:?} from {:?}", &direction, &start);
    //_print_energy(energized);
    //println!();

    match direction {
        Direction::North => loop {
            energized[x][y].add(Direction::North);
            match &map[x][y] {
                Mirror::Empty | Mirror::VerticalSplitter => {
                    if x > 0 {
                        x -= 1;
                    } else {
                        return;
                    }
                }
                Mirror::HorizontalSplitter => {
                    traverse(map, (x as i32, y as i32 + 1), Direction::East, energized);
                    traverse(map, (x as i32, y as i32 - 1), Direction::West, energized);
                    return;
                }
                Mirror::Rightward => {
                    traverse(map, (x as i32, y as i32 + 1), Direction::East, energized);
                    return;
                }
                Mirror::Leftward => {
                    traverse(map, (x as i32, y as i32 - 1), Direction::West, energized);
                    return;
                }
            }
        },
        Direction::South => loop {
            energized[x][y].add(Direction::South);
            match &map[x][y] {
                Mirror::Empty | Mirror::VerticalSplitter => {
                    if x < map.len() - 1 {
                        x += 1;
                    } else {
                        return;
                    }
                }
                Mirror::HorizontalSplitter => {
                    traverse(map, (x as i32, y as i32 + 1), Direction::East, energized);
                    traverse(map, (x as i32, y as i32 - 1), Direction::West, energized);
                    return;
                }
                Mirror::Rightward => {
                    traverse(map, (x as i32, y as i32 - 1), Direction::West, energized);
                    return;
                }
                Mirror::Leftward => {
                    traverse(map, (x as i32, y as i32 + 1), Direction::East, energized);
                    return;
                }
            }
        },
        Direction::East => loop {
            energized[x][y].add(Direction::East);
            match &map[x][y] {
                Mirror::Empty | Mirror::HorizontalSplitter => {
                    if y < map[0].len() - 1 {
                        y += 1;
                    } else {
                        return;
                    }
                }
                Mirror::VerticalSplitter => {
                    traverse(map, (x as i32 - 1, y as i32), Direction::North, energized);
                    traverse(map, (x as i32 + 1, y as i32), Direction::South, energized);
                    return;
                }
                Mirror::Rightward => {
                    traverse(map, (x as i32 - 1, y as i32), Direction::North, energized);

                    return;
                }
                Mirror::Leftward => {
                    traverse(map, (x as i32 + 1, y as i32), Direction::South, energized);

                    return;
                }
            }
        },
        Direction::West => loop {
            energized[x][y].add(Direction::West);
            match &map[x][y] {
                Mirror::Empty | Mirror::HorizontalSplitter => {
                    if y > 0 {
                        y -= 1;
                    } else {
                        return;
                    }
                }
                Mirror::VerticalSplitter => {
                    traverse(map, (x as i32 - 1, y as i32), Direction::North, energized);
                    traverse(map, (x as i32 + 1, y as i32), Direction::South, energized);
                    return;
                }
                Mirror::Rightward => {
                    traverse(map, (x as i32 + 1, y as i32), Direction::South, energized);
                    return;
                }
                Mirror::Leftward => {
                    traverse(map, (x as i32 - 1, y as i32), Direction::North, energized);
                    return;
                }
            }
        },
    }
}

fn _print_energy(energized: &[Vec<Energized>]) {
    for row in energized {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<Mirror>>> {
    use nom::{
        character::complete::line_ending, character::complete::multispace0,
        character::complete::one_of, multi::many1, multi::separated_list1, sequence::terminated,
        Parser,
    };

    terminated(
        separated_list1(
            line_ending,
            many1(one_of("|-/\\.").map(|c| match c {
                '|' => Mirror::VerticalSplitter,
                '-' => Mirror::HorizontalSplitter,
                '/' => Mirror::Rightward,
                '\\' => Mirror::Leftward,
                '.' => Mirror::Empty,
                _ => unreachable!(),
            })),
        ),
        multispace0,
    )(input)
}

impl Energized {
    fn count(&self) -> u8 {
        let mut c = 0;
        if self.north() {
            c += 1;
        }
        if self.south() {
            c += 1;
        }
        if self.east() {
            c += 1;
        }
        if self.west() {
            c += 1;
        }
        c
    }

    fn add(&mut self, direction: Direction) {
        self.data |= direction as u8;
    }

    fn north(&self) -> bool {
        self.data & 0b0000_0001 != 0
    }

    fn south(&self) -> bool {
        self.data & 0b0000_0010 != 0
    }

    fn east(&self) -> bool {
        self.data & 0b0000_0100 != 0
    }

    fn west(&self) -> bool {
        self.data & 0b0000_1000 != 0
    }

    fn direction_contains(&self, direction: Direction) -> bool {
        self.data & (direction as u8) != 0
    }
}

impl Display for Energized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.count() {
            0 => write!(f, "."),
            2 => write!(f, "2"),
            3 => write!(f, "3"),
            4 => write!(f, "4"),
            1 => {
                if self.north() {
                    write!(f, "↑")
                } else if self.south() {
                    write!(f, "↓")
                } else if self.east() {
                    write!(f, "→")
                } else if self.west() {
                    write!(f, "←")
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

impl std::ops::AddAssign<Direction> for Energized {
    fn add_assign(&mut self, rhs: Direction) {
        self.data += rhs as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 46);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 51);
    }

    #[test]
    fn parse() {
        let (inp, mirrors) = super::parse(EXAMPLE).unwrap();
        assert_eq!(inp, "");
        assert_eq!(mirrors.len(), 10);
    }

    #[test]
    fn bit_manip() {
        let mut e = Energized { data: 0 };
        assert_eq!(e.count(), 0);
        assert!(!e.north() && !e.south() && !e.east() && !e.west());

        e.add(Direction::North);
        assert_eq!(e.count(), 1);
        assert!(e.north() && !e.south() && !e.east() && !e.west());

        e.add(Direction::North);
        assert_eq!(e.count(), 1);

        e.add(Direction::South);
        assert_eq!(e.count(), 2);
        e.add(Direction::East);
        assert_eq!(e.count(), 3);
        e.add(Direction::West);
        assert_eq!(e.count(), 4);

        assert_eq!(e.data, 0b0000_1111);
        assert!(e.north() && e.south() && e.east() && e.west());
    }
}
