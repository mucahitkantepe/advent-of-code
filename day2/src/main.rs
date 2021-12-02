fn main() {
    //PART-1
    let (p1, d1) = include_str!("../input.txt")
        .lines()
        .fold((0, 0), |acc: (i32, i32), command: &str| {
            let s = command.split_whitespace().collect::<Vec<&str>>();
            let x = s[1].parse::<i32>().unwrap();
            match (*s)[0] {
                "up" => (acc.0, acc.1 - x),
                "down" => (acc.0, acc.1 + x),
                "forward" => (acc.0 + x, acc.1),
                _ => panic!("unexpected input")
            }
        },
        );
    println!("Part1: {:?}", p1 * d1);
    //PART-2
    let (p2, d2, _) = include_str!("../input.txt")
        .lines()
        .fold((0, 0, 0), |acc: (i32, i32, i32), command: &str| {
            let s = command.split_whitespace().collect::<Vec<&str>>();
            let x = s[1].parse::<i32>().unwrap();
            match (*s)[0] {
                "up" => (acc.0, acc.1, acc.2 - x),
                "down" => (acc.0, acc.1, acc.2 + x),
                "forward" => (acc.0 + x, acc.1 + (acc.2 * x), acc.2),
                _ => panic!("unexpected input")
            }
        },
        );
    println!("Part2: {:?}", p2 * d2)
}
