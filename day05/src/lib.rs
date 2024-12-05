use std::collections::HashMap;

type OrderMap = HashMap<usize, Vec<usize>>;
type Updates = Vec<Vec<usize>>;

pub fn parse(s: &str) -> (OrderMap, Updates) {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let (top, bottom) = (parts[0], parts[1]);

    // In this map, the key needs to be "printed" before the value
    let order_map = top.lines().fold(
        HashMap::new(),
        |mut map: HashMap<usize, Vec<usize>>, line| {
            let parts: Vec<_> = line
                .split_once("|")
                .iter()
                .map(|(k, v)| (k.parse::<usize>().unwrap(), v.parse::<usize>().unwrap()))
                .collect();
            map.entry(parts[0].0).or_insert(Vec::new()).push(parts[0].1);
            map
        },
    );

    let updates = bottom
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (order_map, updates)
}

pub fn correct_order(map: &OrderMap, update: &Vec<usize>) -> bool {
    let mut seen_pages: Vec<usize> = vec![];

    for page in update {
        if let Some(entry) = map.get(page) {
            // If any of them have been seen, then this update is _BAD_
            if entry.iter().any(|item| seen_pages.contains(item)) {
                return false;
            }
        }

        seen_pages.push(*page);
    }

    true
}

pub fn fix_order(map: &OrderMap, update: &Vec<usize>) -> Vec<usize> {
    let mut temp = update.clone();
    temp.sort_by(|a, b| {
        if let Some(entry) = map.get(a) {
            if entry.contains(b) {
                // The first element should come before the second
                return std::cmp::Ordering::Less;
            }
        }

        // The order doesn't matter
        return std::cmp::Ordering::Equal;
    });

    temp
}

pub fn solve01(map: &OrderMap, updates: Updates) -> usize {
    updates
        .iter()
        .filter(|update| correct_order(map, update))
        .map(|update| update[update.len() / 2]) // I assume this is always odd?
        .sum()
}

pub fn solve02(map: &OrderMap, updates: Updates) -> usize {
    updates
        .iter()
        .filter(|update| !correct_order(map, update))
        .map(|update| fix_order(map, update)) // need to order the crap correctly here
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::workspace_dir;
    use std::fs;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_len() {
        let foo = vec![97, 61, 53, 29, 13];

        let result = foo[foo.len() / 2];

        println!("{:?}", result);
    }

    #[test]
    fn test_solve01() {
        let (map, updates) = parse(&EXAMPLE);
        assert_eq!(143, solve01(&map, updates));
    }

    #[test]
    fn test_solve02() {
        let (map, updates) = parse(&EXAMPLE);
        assert_eq!(123, solve02(&map, updates));
    }

    #[test]
    fn test_part01() {
        let content = fs::read_to_string(&workspace_dir().join("day05/input.txt")).unwrap();
        let (map, updates) = parse(&content);
        let result = solve01(&map, updates);
        println!("{}", result);
    }

    #[test]
    fn test_part02() {
        let content = fs::read_to_string(&workspace_dir().join("day05/input.txt")).unwrap();
        let (map, updates) = parse(&content);
        let result = solve02(&map, updates);
        println!("{}", result);
    }
}
