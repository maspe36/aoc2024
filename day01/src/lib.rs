use helpers;

pub fn solve01(left: &[u32], right: &[u32]) -> u32 {
    assert_eq!(left.len(), right.len());

    let mut sorted_left = left.to_vec();
    let mut sorted_right = right.to_vec();

    sorted_left.sort();
    sorted_right.sort();

    sorted_left
        .iter()
        .zip(sorted_right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

pub fn solve02(left: &[u32], right: &[u32]) -> u32 {
    left
        .iter()
        .map(|l| {
            l * right.iter().filter(|r| *r == l).count() as u32
        }).sum()
}

pub fn parse(s: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = helpers::read_lines(s).unwrap().flatten();

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    lines.into_iter().for_each(|line| {
        let mut it = line.split("   ");

        left.push(it.next().unwrap().parse::<u32>().unwrap());
        right.push(it.next().unwrap().parse::<u32>().unwrap());
    });

    (left, right)
}

pub fn part01() -> u32 {
    let (left, right ) = parse("day01/input.txt");

    solve01(&left, &right)
}

pub fn part02() -> u32 {
    let (left, right) = parse("day01/input.txt");

    solve02(&left, &right)
}

#[cfg(test)]
mod tests {
    use super::*;

    static LIST_1: [u32; 6] = [3, 4, 2, 1, 3, 3];

    static LIST_2: [u32; 6] = [4, 3, 5, 3, 9, 3];

    #[test]
    fn base_case() {
        let result = solve01(&LIST_1, &LIST_2);
        assert_eq!(result, 11);

        let result = solve02(&LIST_1, &LIST_2);
        println!("{}", result);
    }

    #[test]
    fn test_part01() {
        let result = part01();
        println!("{}", result);
    }

    #[test]
    fn test_part02() {
        let result = part02();
        println!("{}", result);
    }
}
