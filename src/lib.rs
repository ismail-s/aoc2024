use std::collections::HashMap;

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
}
