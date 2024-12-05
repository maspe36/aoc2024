pub fn parse(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

/// Okay, the word is only ever XMAS.
/// Check _each_ 'x' character, and scan in all 4 cardinal directions, _AND_ diagonals 3 characters out
pub fn solve01(map: &[Vec<char>]) -> usize {
    let mut xmas_count = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != 'X' { continue }

            // Okay, we have an X. Let's figure out all the directions we'll search in.
            let up = y >= 3;
            let down = y < map.len() - 3;
            let left = x >= 3;
            let right = x < row.len() - 3;

            let up_left = up && left;
            let up_right = up && right;
            let down_left = down && left;
            let down_right = down && right;

            if up {
                let search: String = [map[y][x], map[y - 1][x], map[y - 2][x], map[y - 3][x]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if down {
                let search: String = [map[y][x], map[y + 1][x], map[y + 2][x], map[y + 3][x]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if left {
                let search: String = [map[y][x], map[y][x - 1], map[y][x - 2], map[y][x - 3]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if right {
                let search: String = [map[y][x], map[y][x + 1], map[y][x + 2], map[y][x + 3]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if up_left {
                let search: String = [map[y][x], map[y - 1][x - 1], map[y - 2][x - 2], map[y - 3][x - 3]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if up_right {
                let search: String = [map[y][x], map[y - 1][x + 1], map[y - 2][x + 2], map[y - 3][x + 3]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if down_left {
                let search: String = [map[y][x], map[y + 1][x - 1], map[y + 2][x - 2], map[y + 3][x - 3]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }

            if down_right {
                let search: String = [map[y][x], map[y + 1][x + 1], map[y + 2][x + 2], map[y + 3][x + 3]].iter().collect();
                if search == "XMAS" {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count
}

/// Ok, now A is the centroid of our search. Only need to go out 1 character in all
/// cardinal directions, or in the diagonals.
pub fn solve02(map: &[Vec<char>]) -> usize {
    let mut xmas_count = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != 'A' { continue }

            let searchable = y >= 1 && x >= 1 && y < map.len() - 1 && x < row.len() - 1;

            // Okay, do we have room to search?
            if !searchable { continue }

            let diag_left: String = [map[y - 1][x - 1], map[y][x], map[y + 1][x + 1]].iter().collect();
            let diag_right: String = [map[y + 1][x - 1], map[y][x], map[y - 1][x + 1]].iter().collect();

            let diag_left_found = diag_left == "MAS" || diag_left == "SAM";
            let diag_right_found = diag_right == "MAS" || diag_right == "SAM";

            if diag_left_found && diag_right_found {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    use std::fs;
    use helpers::workspace_dir;
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn it_works() {
        let result = parse("xmas\nxmas");
        assert_eq!(result, [['x', 'm', 'a', 's'], ['x', 'm', 'a', 's']]);
    }

    #[test]
    fn test_solve1() {
        let result = solve01(&parse(&EXAMPLE));

        assert_eq!(result, 18);
    }

    #[test]
    fn test_solve2() {
        let result = solve02(&parse(&EXAMPLE));

        assert_eq!(result, 9);
    }

    #[test]
    fn test_part1() {
        let content = fs::read_to_string(workspace_dir().join("day04/input.txt")).unwrap();
        let result = solve01(&parse(&content));

        println!("{}", result);
    }

    #[test]
    fn test_part2() {
        let content = fs::read_to_string(workspace_dir().join("day04/input.txt")).unwrap();
        let result = solve02(&parse(&content));

        println!("{}", result);
    }
}
