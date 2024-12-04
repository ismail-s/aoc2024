use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn day1_part1(inp: &str) -> u64 {
    let parsed_input = inp
        .lines()
        .map(|line| {
            let (first_num, second_num) = line.split_once("   ").unwrap();
            (
                first_num.parse::<u64>().unwrap(),
                second_num.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut first_nums = parsed_input.iter().map(|x| x.0).collect::<Vec<_>>();
    first_nums.sort();
    let mut second_nums = parsed_input.iter().map(|x| x.1).collect::<Vec<_>>();
    second_nums.sort();
    first_nums
        .into_iter()
        .zip(second_nums)
        .map(|(first_num, second_num)| first_num.abs_diff(second_num))
        .sum()
}

pub fn day1_part2(inp: &str) -> u64 {
    let parsed_input = inp
        .lines()
        .map(|line| {
            let (first_num, second_num) = line.split_once("   ").unwrap();
            (
                first_num.parse::<u64>().unwrap(),
                second_num.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let first_nums = parsed_input.iter().map(|x| x.0).collect::<Vec<_>>();
    let second_nums = parsed_input.iter().map(|x| x.1).collect::<Vec<_>>();
    let mut second_nums_map = HashMap::new();
    for num in second_nums {
        second_nums_map
            .entry(num)
            .and_modify(|n| *n += 1)
            .or_insert(1_u64);
    }
    first_nums
        .into_iter()
        .map(|num| {
            let &occurrences = second_nums_map.get(&num).unwrap_or(&0);
            num * occurrences
        })
        .sum()
}

pub fn day2_part1(inp: &str) -> u64 {
    inp.lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|level| level.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| day2_report_is_safe(report))
        .count()
        .try_into()
        .unwrap()
}

pub fn day2_part2(inp: &str) -> u64 {
    inp.lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|level| level.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let first = day2_report_is_safe(report);
            if first {
                return first;
            }
            for n in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(n);
                if day2_report_is_safe(&modified_report) {
                    return true;
                }
            }
            false
        })
        .count()
        .try_into()
        .unwrap()
}

fn day2_report_is_safe(report: &[u64]) -> bool {
    let all_increasing = report
        .iter()
        .zip(&report[1..])
        .all(|(fst, snd)| fst < snd && (snd - fst <= 3));
    let all_decreasing = report
        .iter()
        .zip(&report[1..])
        .all(|(fst, snd)| fst > snd && (fst - snd <= 3));
    all_increasing || all_decreasing
}

pub fn day3_part1(inp: &str) -> u64 {
    let re = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();
    re.captures_iter(inp)
        .map(|captures| {
            let first_num = &captures[1].parse::<u64>().unwrap();
            let second_num = &captures[2].parse::<u64>().unwrap();
            first_num * second_num
        })
        .sum()
}

pub fn day3_part2(inp: &str) -> u64 {
    let re = Regex::new("do\\(\\)|don't\\(\\)|mul\\(([0-9]+),([0-9]+)\\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for captures in re.captures_iter(inp) {
        if &captures[0] == "do()" {
            enabled = true;
            continue;
        } else if captures[0].starts_with("don't") {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        let first_num = &captures[1].parse::<u64>().unwrap();
        let second_num = &captures[2].parse::<u64>().unwrap();
        sum += first_num * second_num;
    }
    sum
}

pub fn day4_part1(inp: &str) -> u64 {
    let grid: Vec<Vec<_>> = inp.lines().map(|line| line.chars().collect()).collect();
    // find the Xs in the grid
    let mut x_locs = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, &chr) in line.iter().enumerate() {
            if chr == 'X' {
                x_locs.push((x, y));
            }
        }
    }
    // for each X, count the number of words from it
    x_locs
        .into_iter()
        .map(|point| day4_count_words_from_point(&grid, point))
        .sum()
}

fn day4_count_words_from_point(grid: &[Vec<char>], (x, y): (usize, usize)) -> u64 {
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let (x, y): (i64, i64) = (x.try_into().unwrap(), y.try_into().unwrap());
    let mut word_coords = vec![
        [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
        [(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
        [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
        [(x, y), (x - 1, y), (x - 2, y), (x - 3, y)],
        [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
        [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
        [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
        [(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
    ];
    word_coords.retain(|coords| {
        !coords.iter().any(|&(x1, y1)| {
            x1 < 0
                || y1 < 0
                || x1 >= grid_width.try_into().unwrap()
                || y1 >= grid_height.try_into().unwrap()
        })
    });

    let mut count = 0;
    for coords in word_coords {
        let word = coords
            .iter()
            .map(|&(x1, y1)| grid[y1 as usize][x1 as usize])
            .collect::<String>();
        if word == "XMAS" {
            count += 1;
        }
    }
    count
}

pub fn day4_part2(inp: &str) -> usize {
    let grid: Vec<Vec<_>> = inp.lines().map(|line| line.chars().collect()).collect();
    // find the Ms in the grid
    let mut m_locs = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, &chr) in line.iter().enumerate() {
            if chr == 'M' {
                m_locs.push((x, y));
            }
        }
    }
    let word_centres = m_locs
        .into_iter()
        .flat_map(|point| day4_part_2_get_word_centres(&grid, point))
        .collect::<Vec<_>>();

    let mut freq_map = HashMap::new();
    for word_centre in word_centres {
        freq_map
            .entry(word_centre)
            .and_modify(|n| *n += 1)
            .or_insert(1_u64);
    }
    freq_map.retain(|_, count| *count > 1);
    freq_map.len()
}

fn day4_part_2_get_word_centres(grid: &[Vec<char>], (x, y): (usize, usize)) -> Vec<(i64, i64)> {
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let (x, y): (i64, i64) = (x.try_into().unwrap(), y.try_into().unwrap());
    let mut word_coords = vec![
        [(x, y), (x + 1, y + 1), (x + 2, y + 2)],
        [(x, y), (x + 1, y - 1), (x + 2, y - 2)],
        [(x, y), (x - 1, y + 1), (x - 2, y + 2)],
        [(x, y), (x - 1, y - 1), (x - 2, y - 2)],
    ];
    word_coords.retain(|coords| {
        !coords.iter().any(|&(x1, y1)| {
            x1 < 0
                || y1 < 0
                || x1 >= grid_width.try_into().unwrap()
                || y1 >= grid_height.try_into().unwrap()
        })
    });

    let mut word_centres = Vec::new();
    for coords in word_coords {
        let word = coords
            .iter()
            .map(|&(x1, y1)| grid[y1 as usize][x1 as usize])
            .collect::<String>();
        if word == "MAS" {
            word_centres.push(coords[1]);
        }
    }
    word_centres
}

pub fn day5_part1(inp: &str) -> u32 {
    let (first_part, second_part) = inp.split_once("\n\n").unwrap();
    let page_ordering_rules = first_part
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<HashSet<(u32, u32)>>();

    second_part
        .lines()
        .map(|line| {
            line.split(',')
                .map(|string| string.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|update| {
            for (n, &a) in update.iter().enumerate() {
                for &b in update.iter().skip(n + 1) {
                    if !page_ordering_rules.contains(&(a, b)) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn day5_part2(inp: &str) -> u32 {
    let (first_part, second_part) = inp.split_once("\n\n").unwrap();
    let page_ordering_rules = first_part
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<HashSet<(u32, u32)>>();

    second_part
        .lines()
        .map(|line| {
            line.split(',')
                .map(|string| string.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|update| {
            for (n, &a) in update.iter().enumerate() {
                for &b in update.iter().skip(n + 1) {
                    if !page_ordering_rules.contains(&(a, b)) {
                        return true;
                    }
                }
            }
            false
        })
        .map(|update| {
            let mut new_update = Vec::new();
            for page_num in update.into_iter() {
                let mut havent_inserted = true;
                for (n, &page_num1) in new_update.iter().enumerate() {
                    if page_ordering_rules.contains(&(page_num1, page_num)) {
                        continue;
                    } else {
                        new_update.insert(n, page_num);
                        havent_inserted = false;
                        break;
                    }
                }
                if havent_inserted {
                    new_update.push(page_num);
                }
            }
            println!("am here with {new_update:?}");
            new_update[new_update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn day1() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let input = fs::read_to_string("inputs/day1.txt").unwrap();

        assert_eq!(day1_part1(test_input), 11);
        assert_eq!(day1_part1(&input), 1530215);

        assert_eq!(day1_part2(&test_input), 31);
        assert_eq!(day1_part2(&input), 26800609);
    }

    #[test]
    fn day2() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let input = fs::read_to_string("inputs/day2.txt").unwrap();

        assert_eq!(day2_part1(test_input), 2);
        assert_eq!(day2_part1(&input), 269);

        assert_eq!(day2_part2(&test_input), 4);
        assert_eq!(day2_part2(&input), 337);
    }

    #[test]
    fn day3() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let test_input_2 =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = fs::read_to_string("inputs/day3.txt").unwrap();

        assert_eq!(day3_part1(test_input), 161);
        assert_eq!(day3_part1(&input), 174336360);

        assert_eq!(day3_part2(&test_input_2), 48);
        assert_eq!(day3_part2(&input), 88802350);
    }

    #[test]
    fn day4() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(day4_part1(test_input), 18);
        assert_eq!(day4_part2(&test_input), 9);
    }

    #[test]
    fn day5() {
        let test_input = "47|53
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
        assert_eq!(day5_part1(test_input), 143);
        assert_eq!(day5_part2(test_input), 123);
    }
}
