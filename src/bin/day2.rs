fn main() {
    let input = include_str!("../../inputs/day2.txt");

    let commands: Vec<(&str, u32)> =
        input
            .lines()
            .map(|line| line.split(' '))
            .fold(Vec::new(), |mut line, mut values| {
                line.push((
                    values.next().unwrap(),
                    values.next().unwrap().parse().unwrap(),
                ));
                line
            });

    let first = commands
        .iter()
        .fold([0; 2], |acc, val| match val.0 {
            "forward" => [acc[0] + val.1, acc[1]],
            "down" => [acc[0], acc[1] + val.1],
            "up" => [acc[0], acc[1] - val.1],
            _ => [acc[0], acc[1]],
        })
        .windows(2)
        .fold(0, |_acc, val| val[0] * val[1]);

    assert_eq!(first, 1714950);

    let second = commands
        .iter()
        .fold([0; 3], |acc, val| match val.0 {
            "forward" => [acc[0] + val.1, acc[1] + (val.1 * acc[2]), acc[2]],
            "down" => [acc[0], acc[1], acc[2] + val.1],
            "up" => [acc[0], acc[1], acc[2] - val.1],
            _ => [acc[0], acc[1], acc[2]],
        })
        .windows(3)
        .fold(0, |_acc, val| val[0] * val[1]);

    assert_eq!(second, 1281977850);
}
