#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Kind {
    Increment,
    Decrement,
    Unsafe,
}

pub fn to_kinds(level: &[u64]) -> Vec<Kind> {
    level
        .windows(2)
        .map(|w| {
            let (i, j) = (w[0], w[1]);

            if i > j && (i - j) <= 3 {
                Kind::Decrement
            } else if i < j && (j - i) <= 3 {
                Kind::Increment
            } else {
                Kind::Unsafe
            }
        })
        .collect::<Vec<Kind>>()
}

pub fn safe02(report: &[u64]) -> bool {
    // BRUTE FORCE IT BABY
    let mut report_permutations: Vec<Vec<u64>> = vec![Vec::from(report), report[0..(report.len() - 1)].to_vec()];

    for i in 0..report.len() - 1 {
        let subset = report[0..i].iter().chain(report[i + 1..].iter()).cloned().collect();
        report_permutations.push(subset);
    }

    report_permutations.iter().any(|r| safe01(r))
}

pub fn safe01(level: &[u64]) -> bool {
    let kinds = to_kinds(level);

    if kinds.iter().any(|k| *k == Kind::Unsafe) {
        false
    } else if kinds.iter().all(|k| *k == Kind::Increment) {
        true
    } else if kinds.iter().all(|k| *k == Kind::Decrement) {
        true
    } else {
        false
    }
}

pub fn parse(s: &str) -> Vec<Vec<u64>> {
    let lines = helpers::read_lines(s).unwrap().flatten();

    let mut levels: Vec<Vec<u64>> = vec![];

    lines.into_iter().for_each(|line| {
        let level: Vec<u64> = line
            .split(" ")
            .into_iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        levels.push(level);
    });

    levels
}

pub fn part01() -> u64 {
    let input = parse("day02/input.txt");

    input.iter().filter(|level| safe01(&level)).count() as u64
}

pub fn part02() -> u64 {
    let input = parse("day02/input.txt");

    input.iter().filter(|level| safe02(&level)).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAFE_DEC_LEVEL: [u64; 5] = [7, 6, 4, 2, 1];
    const UNSAFE_LAGE_INC: [u64; 5] = [1, 2, 7, 8, 9];
    const UNSAFE_LAGE_DEC: [u64; 5] = [9, 7, 6, 2, 1];
    const UNSAFE_FLIP_FLOP: [u64; 5] = [1, 3, 2, 4, 5];
    const UNSAFE_STAGNATE: [u64; 5] = [8, 6, 4, 4, 1];
    const SAFE_INC_LEVEL: [u64; 5] = [1, 3, 6, 7, 9];

    #[test]
    fn base_cases01() {
        assert_eq!(safe01(&SAFE_DEC_LEVEL), true);
        assert_eq!(safe01(&UNSAFE_LAGE_INC), false);
        assert_eq!(safe01(&UNSAFE_LAGE_DEC), false);
        assert_eq!(safe01(&UNSAFE_FLIP_FLOP), false);
        assert_eq!(safe01(&UNSAFE_STAGNATE), false);
        assert_eq!(safe01(&SAFE_INC_LEVEL), true);
    }

    #[test]
    fn base_cases02() {
        assert_eq!(safe02(&SAFE_DEC_LEVEL), true);
        assert_eq!(safe02(&UNSAFE_LAGE_INC), false);
        assert_eq!(safe02(&UNSAFE_LAGE_DEC), false);
        assert_eq!(safe02(&UNSAFE_FLIP_FLOP), true);
        assert_eq!(safe02(&UNSAFE_STAGNATE), true);
        assert_eq!(safe02(&SAFE_INC_LEVEL), true);
    }

    #[test]
    fn edge_cases02() {
        assert_eq!(safe02(&[3, 3, 4, 5, 6]), true);
        assert_eq!(safe02(&[8, 3, 4, 5, 6]), true);
        assert_eq!(safe02(&[1, 3, 4, 5, 2]), true);
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
