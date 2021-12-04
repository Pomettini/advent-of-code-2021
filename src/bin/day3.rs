fn main() {
    let input = include_str!("../../inputs/day3.txt");

    let data: Vec<&str> = input.lines().collect();

    let data_half = data.len() / 2;

    let gamma = data
        .into_iter()
        .fold([0; 12], |mut acc, row| {
            row.chars().fold(0, |i, c| {
                if c == '0' {
                    acc[i] += 1;
                }
                i + 1
            });
            acc
        })
        .iter()
        .fold(String::new(), |mut a, b| {
            if b > &data_half {
                a.push('0')
            } else {
                a.push('1');
            }
            a
        });

    let epsilon = gamma.chars().fold(String::new(), |mut s, c| {
        if c == '0' {
            s.push('1');
        } else {
            s.push('0');
        }
        s
    });

    let first =
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();

    assert_eq!(first, 3549854);
}
