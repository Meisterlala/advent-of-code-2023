use nom::{
    bytes::complete::{tag, take_until, take_while_m_n},
    character::{
        self,
        complete::{line_ending, multispace0, not_line_ending, one_of, space1},
    },
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated},
    Parser,
};

crate::solution!(18, solve_a, solve_b);

/// Formula:
/// 1/2 * (sum_{i=0}^{n-1} (x_i * y_{i+1} - x_{i+1} * y_i))

pub fn solve_a(input: &str) -> u64 {
    let (rest, plan) = parse_a(input).expect("failed to parse input");
    debug_assert!(rest.is_empty());

    let path_area = plan
        .iter()
        .map(|d| match d {
            Direction::Up { distance } => distance,
            Direction::Down { distance } => distance,
            Direction::Left { distance } => distance,
            Direction::Right { distance } => distance,
        })
        .sum::<u64>();
    let points = create_points(&plan);

    _draw_area(&points, "a");
    area(&points) + path_area / 2 + 1
}

pub fn solve_b(input: &str) -> u64 {
    let (rest, plan) = parse_b(input).expect("failed to parse input");
    debug_assert!(rest.is_empty());
    let path_area = plan
        .iter()
        .map(|d| match d {
            Direction::Up { distance } => distance,
            Direction::Down { distance } => distance,
            Direction::Left { distance } => distance,
            Direction::Right { distance } => distance,
        })
        .sum::<u64>();
    let points = create_points(&plan);

    _draw_area(&points, "b");
    area(&points) + path_area / 2 + 1
}

#[derive(Debug)]
enum Direction {
    Up { distance: u64 },
    Down { distance: u64 },
    Left { distance: u64 },
    Right { distance: u64 },
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn create_points(directions: &[Direction]) -> Vec<Point> {
    // Start at (0, 0)
    let mut current = Point { x: 0, y: 0 };

    directions
        .iter()
        .map(|direction| match direction {
            Direction::Up { distance } => {
                current.y += *distance as i64;
                current
            }
            Direction::Down { distance } => {
                current.y -= *distance as i64;
                current
            }
            Direction::Left { distance } => {
                current.x -= *distance as i64;
                current
            }
            Direction::Right { distance } => {
                current.x += *distance as i64;
                current
            }
        })
        // Add the starting point
        .fold(vec![Point { x: 0, y: 0 }], |mut acc, p| {
            acc.push(p);
            acc
        })
}

fn area(points: &[Point]) -> u64 {
    let mut res = 0;

    for i in 0..points.len() {
        let p = if i > 0 {
            &points[i - 1]
        } else {
            &points[points.len() - 1]
        };
        let q = &points[i];
        res += (p.x - q.x) * (p.y + q.y);
    }
    res.unsigned_abs() / 2
}

fn parse_a(input: &str) -> nom::IResult<&str, Vec<Direction>> {
    let direction =
        pair(terminated(one_of("UDLR"), space1), character::complete::u64).map(|(c, d)| match c {
            'U' => Direction::Up { distance: d },
            'D' => Direction::Down { distance: d },
            'L' => Direction::Left { distance: d },
            'R' => Direction::Right { distance: d },
            _ => unreachable!(),
        });

    terminated(
        separated_list1(line_ending, terminated(direction, not_line_ending)),
        multispace0,
    )
    .parse(input)
}

fn parse_b(input: &str) -> nom::IResult<&str, Vec<Direction>> {
    let direction = delimited(
        tag("(#"),
        pair(
            take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit()),
            one_of("0123"),
        )
        .map(|(color, direction)| {
            let distance = u64::from_str_radix(color, 16).unwrap();
            match direction {
                '0' => Direction::Right { distance },
                '1' => Direction::Down { distance },
                '2' => Direction::Left { distance },
                '3' => Direction::Up { distance },
                _ => unreachable!(),
            }
        }),
        tag(")"),
    );

    terminated(
        separated_list1(line_ending, preceded(take_until("("), direction)),
        multispace0,
    )
    .parse(input)
}

#[cfg(not(debug_assertions))]
fn _draw_area(_points: &[Point], _name: &str) {}

#[cfg(debug_assertions)]
fn _draw_area(points: &[Point], name: &str) {
    use plotters::prelude::*;

    // find min/max of x and y
    let max_x = points.iter().map(|p| p.x).max().unwrap() as f64;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as f64;

    // Scale all points to be between 0 and 1
    let points = points
        .iter()
        .map(|p| (p.x as f64 / max_x.max(max_y), p.y as f64 / max_x.max(max_y)))
        .collect::<Vec<_>>();

    // Create Dir
    std::fs::create_dir_all("target/graph").unwrap();
    let file = format!("target/graph/day_18_{}.svg", name);
    let root = SVGBackend::new(&file, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(-1.0..1.0, -1.0..1.0)
        .unwrap();

    // Draw the area
    chart
        .draw_series(AreaSeries::new(points, 0., BLUE))
        .unwrap();

    // Draw the starting point with label
    chart
        .draw_series(PointSeries::of_element(
            vec![(0., 0.)],
            5.0,
            &RED,
            &|c, s, st| {
                EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(
                        "Starting Point",
                        (10, 0),
                        ("sans-serif", 15).into_font().color(&RED),
                    )
            },
        ))
        .unwrap();

    root.present().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 62);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 952408144115);
    }

    #[test]
    fn parse() {
        let (rest, plan) = super::parse_a(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert_eq!(plan.len(), 14);
    }

    #[test]
    fn area() {
        let square = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
        ];
        _draw_area(&square, "test_square");
        assert_eq!(super::area(&square), 1);

        let sqaure_4 = vec![
            Point { x: 0, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 4, y: 4 },
            Point { x: 0, y: 4 },
        ];
        _draw_area(&sqaure_4, "test_square_4");
        assert_eq!(super::area(&sqaure_4), 16);

        let square_around_origin = vec![
            Point { x: -2, y: -2 },
            Point { x: 2, y: -2 },
            Point { x: 2, y: 2 },
            Point { x: -2, y: 2 },
        ];
        _draw_area(&square_around_origin, "test_square_around_origin");
        assert_eq!(super::area(&square_around_origin), 16);
    }
}
