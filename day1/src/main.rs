#![feature(array_windows)]

fn main() {
    let res1 = include_str!("../1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count();

    println!("part1: {:?}", res1);

    let res2 = include_str!("../1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
        .array_windows()
        .filter(|[a, _, _, d]| a < d)
        .count();

    println!("part2: {:?}", res2)
}
