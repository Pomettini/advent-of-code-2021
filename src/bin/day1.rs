fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let numbers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let first = numbers
        .windows(2)
        .fold(0, |acc, val| if val[0] < val[1] { acc + 1 } else { acc });

    assert_eq!(first, 1624);

    let second = numbers
        .windows(3)
        .map(|val| val[0] + val[1] + val[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |acc, val| if val[0] < val[1] { acc + 1 } else { acc });

    assert_eq!(second, 1653);
}
