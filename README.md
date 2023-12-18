#  Advent of Code 2023   <img src="./.github/christmas_ferris.png" align="right" width="300">
[![Build and test](https://github.com/Meisterlala/advent-of-code-2023/actions/workflows/rust.yml/badge.svg)](https://github.com/Meisterlala/advent-of-code-2023/actions/workflows/rust.yml) [![LoC](https://tokei.rs/b1/github/Meisterlala/advent-of-code-2023)](https://github.com/search?q=repo%3AMeisterlala%2Fadvent-of-code-2023++language%3ARust&type=code)  [![GitHub last commit (branch)](https://img.shields.io/github/last-commit/meisterlala/advent-of-code-2023/main)](https://github.com/Meisterlala/advent-of-code-2023/commits/main/)

## What is Advent of Code? 🎄

[Advent of Code](https://adventofcode.com/) is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. Each day there will be a new problem that is split into 2 parts. The second parts is a more difficult variation of the first part. This repository contains my solutions to the Advent of Code 2023 puzzles


### Some of the techniques used in the solutions

- [Parser combinator](https://en.wikipedia.org/wiki/Parser_combinator) to read the puzzle inputs
- Range math to track multiple mappings between values during [Day 05](src/day_05.rs)
- [Quadratic programming](https://en.wikipedia.org/wiki/Quadratic_programming) during [Day 06](src/day_06.rs) to find the minimum and maximum of a function. Which avoids having to use a brute force approach and instead solving it in constant time.
- [Cycle detection](https://en.wikipedia.org/wiki/Cycle_detection) during [Day 08](src/day_07.rs) to count the length of a sub cycle and then use that to calculate the length of the full cycle with the [least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple).
- [Pascal's triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle) during [Day 09](src/day_09.rs) to calculate the binomial coefficients and predict the next value in a sequence.
- [Point in Polygon](https://en.wikipedia.org/wiki/Point_in_polygon) during [Day 10](src/day_10.rs) to determine how many free spaces are enclosed by a line.
- [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming) during [Day 12](src/day_12.rs) to solve the recursiveness of the problem in a reasonable time.
- [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) during [Day 17](src/day_17.rs) to find the shortest path between two points, while following certain restrictions.
- [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) during [Day 18](src/day_18.rs) to calculate the area of a polygon. Here is a rendering of the [Day 18 Part 1](.github/day_18_a.svg) and [Day 18 Part 2](.github/day_18_b.svg) polygon.
  

## Solutions

| Day                                                                            | Stars         |         Code          | Execution Time |
| :----------------------------------------------------------------------------- | :------------ | :-------------------: | -------------: |
| [Day 01: Trebuchet?!](https://adventofcode.com/2023/day/1)                     | :star: :star: | [Code](src/day_01.rs) |       1.003 ms |
| [Day 02: Cube Conundrum](https://adventofcode.com/2023/day/2)                  | :star: :star: | [Code](src/day_02.rs) |       0.325 ms |
| [Day 03: Gear Ratios](https://adventofcode.com/2023/day/3)                     | :star: :star: | [Code](src/day_03.rs) |       4.476 ms |
| [Day 04: Scratchcards](https://adventofcode.com/2023/day/4)                    | :star: :star: | [Code](src/day_04.rs) |       0.867 ms |
| [Day 05: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5) | :star: :star: | [Code](src/day_05.rs) |       0.360 ms |
| [Day 06: Wait For It](https://adventofcode.com/2023/day/6)                     | :star: :star: | [Code](src/day_06.rs) |       0.068 ms |
| [Day 07: Camel Cards](https://adventofcode.com/2023/day/7)                     | :star: :star: | [Code](src/day_07.rs) |       2.450 ms |
| [Day 08: Haunted Wasteland](https://adventofcode.com/2023/day/8)               | :star: :star: | [Code](src/day_08.rs) |       3.930 ms |
| [Day 09: Mirage Maintenance](https://adventofcode.com/2023/day/9)              | :star: :star: | [Code](src/day_09.rs) |       0.929 ms |
| [Day 10: Pipe Maze](https://adventofcode.com/2023/day/10)                      | :star: :star: | [Code](src/day_10.rs) |       2.291 ms |
| [Day 11: Cosmic Expansion](https://adventofcode.com/2023/day/11)               | :star: :star: | [Code](src/day_11.rs) |       2.566 ms |
| [Day 12: Hot Springs](https://adventofcode.com/2023/day/12)                    | :star: :star: | [Code](src/day_12.rs) |       4.443 ms |
| [Day 13: Point of Incidence](https://adventofcode.com/2023/day/13)             | :star: :star: | [Code](src/day_13.rs) |       1.251 ms |
| [Day 14: Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)       | :star: :star: | [Code](src/day_14.rs) |      63.610 ms |
| [Day 15: Lens Library](https://adventofcode.com/2023/day/15)                   | :star: :star: | [Code](src/day_15.rs) |       1.274 ms |
| [Day 16: The Floor Will Be Lava](https://adventofcode.com/2023/day/16)         | :star: :star: | [Code](src/day_16.rs) |      19.653 ms |
| [Day 17: Clumsy Crucible](https://adventofcode.com/2023/day/17)                | :star: :star: | [Code](src/day_17.rs) |     197.790 ms |
| [Day 18: Lavaduct Lagoon](https://adventofcode.com/2023/day/18)                | :star: :star: | [Code](src/day_18.rs) |       0.312 ms |

## Try it out

### How to run the code?

To run the code, you'll first need to obtain the puzzle inputs from the [Advent of Code](https://adventofcode.com/) website and place them in the [input folder](inputs). Or save your session cookie in the environment variable `AOC_SESSION`.

You'll also need to have Rust installed on your system. If you haven't installed it yet, you can download it from [here](https://www.rust-lang.org/tools/install). Once you have Rust installed and the inputs in place, you can run all the puzzles using the following command:

```sh
# Run all the days
cargo run --release
# Run a specific days
cargo run --release -- day01
```


### How to benchmark the code?

You can benchmark the performance of the code using the following commands:

```sh
# Benchmark all the days
cargo bench
# Benchmark a specific days
cargo bench -- day01
```
