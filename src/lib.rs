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
            new_update[new_update.len() / 2]
        })
        .sum()
}

enum Day6Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn day6_part1(inp: &str) -> usize {
    let grid = inp
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let rows = grid.len();
    let cols = grid[0].len();
    let transp_grid = (0..cols)
        .map(|col| (0..rows).map(|row| grid[row][col]).collect())
        .collect::<Vec<Vec<_>>>();
    let mut guard_loc = (0, 0);
    'outer: for (row_num, row) in grid.iter().enumerate() {
        for (col_num, &col) in row.iter().enumerate() {
            if col == '^' {
                guard_loc = (col_num, row_num);
                break 'outer;
            }
        }
    }
    let mut guard_direction = Day6Direction::Up;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(guard_loc);
    loop {
        // Below code may have some +-1 errors, but it worked well enough to get the right answer for the inputs I had.
        match guard_direction {
            Day6Direction::Up => {
                let col_part = &transp_grid[guard_loc.0][0..guard_loc.1];
                match col_part.iter().rposition(|&chr| chr == '#') {
                    Some(wall_col_num) => {
                        ((wall_col_num + 1)..col_part.len()).for_each(|col_num| {
                            visited_positions.insert((guard_loc.0, col_num));
                        });
                        guard_direction = Day6Direction::Right;
                        guard_loc = (guard_loc.0, wall_col_num + 1);
                    }
                    None => {
                        (0..col_part.len()).for_each(|col_num| {
                            visited_positions.insert((guard_loc.0, col_num));
                        });
                        return visited_positions.len();
                    }
                }
            }
            Day6Direction::Right => {
                let row_part = &grid[guard_loc.1][(guard_loc.0 + 1)..];
                match row_part.iter().position(|&chr| chr == '#') {
                    Some(wall_col_num) => {
                        ((guard_loc.0 + 1)..(guard_loc.0 + wall_col_num + 1)).for_each(|row_num| {
                            visited_positions.insert((row_num, guard_loc.1));
                        });
                        guard_direction = Day6Direction::Down;
                        guard_loc = (guard_loc.0 + wall_col_num, guard_loc.1);
                    }
                    None => {
                        ((guard_loc.0 + 1)..(guard_loc.0 + row_part.len())).for_each(|row_num| {
                            visited_positions.insert((row_num, guard_loc.1));
                        });
                        return visited_positions.len();
                    }
                }
            }
            Day6Direction::Down => {
                let col_part = &transp_grid[guard_loc.0][(guard_loc.1 + 1)..];
                match col_part.iter().position(|&chr| chr == '#') {
                    Some(wall_col_num) => {
                        ((guard_loc.1 + 1)..(guard_loc.1 + wall_col_num + 1)).for_each(|col_num| {
                            visited_positions.insert((guard_loc.0, col_num));
                        });
                        guard_direction = Day6Direction::Left;
                        guard_loc = (guard_loc.0, (guard_loc.1 + wall_col_num));
                    }
                    None => {
                        ((guard_loc.1 + 1)..(guard_loc.1 + col_part.len() + 1)).for_each(
                            |col_num| {
                                visited_positions.insert((guard_loc.0, col_num));
                            },
                        );
                        return visited_positions.len();
                    }
                }
            }
            Day6Direction::Left => {
                let row_part = &grid[guard_loc.1][0..guard_loc.0];
                match row_part.iter().rposition(|&chr| chr == '#') {
                    Some(wall_col_num) => {
                        ((wall_col_num + 1)..row_part.len()).for_each(|row_num| {
                            visited_positions.insert((row_num, guard_loc.1));
                        });
                        guard_direction = Day6Direction::Up;
                        guard_loc = (wall_col_num + 1, guard_loc.1);
                    }
                    None => {
                        (0..row_part.len()).for_each(|row_num| {
                            visited_positions.insert((row_num, guard_loc.1));
                        });
                        return visited_positions.len();
                    }
                }
            }
        }
    }
}

pub fn day6_part2(inp: &str) -> usize {
    let grid = inp
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut guard_loc_initial = (0, 0);
    'outer: for (row_num, row) in grid.iter().enumerate() {
        for (col_num, &col) in row.iter().enumerate() {
            if col == '^' {
                guard_loc_initial = (col_num, row_num);
                break 'outer;
            }
        }
    }
    let mut num_of_positions = 0;

    for obstruction @ (obstruction_col, obstruction_row) in
        (0..rows).flat_map(|r| (0..cols).map(|c| (c, r)).collect::<Vec<_>>())
    {
        if obstruction == guard_loc_initial || grid[obstruction_row][obstruction_col] == '#' {
            continue;
        }
        let mut guard_loc = guard_loc_initial;
        let mut grid = inp
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let mut transp_grid = (0..cols)
            .map(|col| (0..rows).map(|row| grid[row][col]).collect())
            .collect::<Vec<Vec<_>>>();
        grid[obstruction_row][obstruction_col] = '#';
        transp_grid[obstruction_col][obstruction_row] = '#';

        let mut guard_direction = Day6Direction::Up;
        let mut visited_positions = HashSet::new();
        visited_positions.insert(guard_loc);

        let mut further_loops_counter = 0;
        loop {
            let start_num_of_visited_positions = visited_positions.len();
            match guard_direction {
                Day6Direction::Up => {
                    let col_part = &transp_grid[guard_loc.0][0..guard_loc.1];
                    match col_part.iter().rposition(|&chr| chr == '#') {
                        Some(wall_col_num) => {
                            ((wall_col_num + 1)..col_part.len()).for_each(|col_num| {
                                visited_positions.insert((guard_loc.0, col_num));
                            });
                            guard_direction = Day6Direction::Right;
                            guard_loc = (guard_loc.0, wall_col_num + 1);
                        }
                        None => break,
                    }
                }
                Day6Direction::Right => {
                    let row_part = &grid[guard_loc.1][(guard_loc.0 + 1)..];
                    match row_part.iter().position(|&chr| chr == '#') {
                        Some(wall_col_num) => {
                            ((guard_loc.0 + 1)..(guard_loc.0 + wall_col_num + 1)).for_each(
                                |row_num| {
                                    visited_positions.insert((row_num, guard_loc.1));
                                },
                            );
                            guard_direction = Day6Direction::Down;
                            guard_loc = (guard_loc.0 + wall_col_num, guard_loc.1);
                        }
                        None => break,
                    }
                }
                Day6Direction::Down => {
                    let col_part = &transp_grid[guard_loc.0][(guard_loc.1 + 1)..];
                    match col_part.iter().position(|&chr| chr == '#') {
                        Some(wall_col_num) => {
                            ((guard_loc.1 + 1)..(guard_loc.1 + wall_col_num + 1)).for_each(
                                |col_num| {
                                    visited_positions.insert((guard_loc.0, col_num));
                                },
                            );
                            guard_direction = Day6Direction::Left;
                            guard_loc = (guard_loc.0, (guard_loc.1 + wall_col_num));
                        }
                        None => break,
                    }
                }
                Day6Direction::Left => {
                    let row_part = &grid[guard_loc.1][0..guard_loc.0];
                    match row_part.iter().rposition(|&chr| chr == '#') {
                        Some(wall_col_num) => {
                            ((wall_col_num + 1)..row_part.len()).for_each(|row_num| {
                                visited_positions.insert((row_num, guard_loc.1));
                            });
                            guard_direction = Day6Direction::Up;
                            guard_loc = (wall_col_num + 1, guard_loc.1);
                        }
                        None => break,
                    }
                }
            }
            if start_num_of_visited_positions == visited_positions.len() {
                if further_loops_counter < 8 {
                    further_loops_counter += 1;
                    continue;
                }
                num_of_positions += 1;
                break;
            }
        }
    }
    num_of_positions
}

pub fn day7_part1(inp: &str) -> u64 {
    inp.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(test_val, equation)| {
            (
                test_val.parse::<u64>().unwrap(),
                equation
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .filter(|(test_val, equation)| {
            if !equation.contains(&1)
                && (equation.iter().sum::<u64>() > *test_val
                    || equation.iter().product::<u64>() < *test_val)
            {
                return false;
            }
            for mut operator_iteration in 0..(2_u64.pow((equation.len() as u32) - 1)) {
                let mut actual_val = equation[0];
                for n in &equation[1..] {
                    if operator_iteration % 2 == 0 {
                        actual_val += n;
                    } else {
                        actual_val *= n;
                    }
                    operator_iteration /= 2;
                }
                if actual_val == *test_val {
                    return true;
                }
            }
            false
        })
        .map(|(test_val, _)| test_val)
        .sum()
}

pub fn day7_part2(inp: &str) -> u64 {
    inp.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(test_val, equation)| {
            (
                test_val.parse::<u64>().unwrap(),
                equation
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .filter(|(test_val, equation)| {
            for mut operator_iteration in 0..(3_u64.pow((equation.len() as u32) - 1)) {
                let mut actual_val = equation[0];
                for n in &equation[1..] {
                    match operator_iteration % 3 {
                        0 => {
                            actual_val += n;
                        }
                        1 => {
                            actual_val *= n;
                        }
                        _ => {
                            // Concatenate the 2 numbers
                            let num_of_digits_to_shift_by = n.ilog10() + 1;
                            actual_val = (actual_val * 10_u64.pow(num_of_digits_to_shift_by)) + n;
                        }
                    }
                    operator_iteration /= 3;
                }
                if actual_val == *test_val {
                    return true;
                }
            }
            false
        })
        .map(|(test_val, _)| test_val)
        .sum()
}

pub fn day8_part1(inp: &str) -> usize {
    let grid = inp.lines().collect::<Vec<_>>();
    let num_of_rows = grid.len();
    let num_of_cols = grid[0].chars().count();

    // find all antennas and group by frequency
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    grid.iter().enumerate().for_each(|(row_num, &row)| {
        row.chars().enumerate().for_each(|(col_num, chr)| {
            if chr != '.' {
                antennas
                    .entry(chr)
                    .and_modify(|antenna_list| antenna_list.push((col_num, row_num)))
                    .or_insert(vec![(col_num, row_num)]);
            }
        });
    });
    // for each frequency, find all the antinodes
    let mut unique_antinodes = HashSet::new();
    for (_, antennas_vec) in antennas.iter() {
        for (n, antenna1) in antennas_vec.iter().enumerate() {
            for antenna2 in antennas_vec.iter().skip(n + 1) {
                let diff_col = (antenna2.0 as i32) - (antenna1.0 as i32);
                let diff_row = (antenna2.1 as i32) - (antenna1.1 as i32);
                let antinode1 = (
                    ((antenna1.0 as i32) - diff_col),
                    ((antenna1.1 as i32) - diff_row),
                );
                let antinode2 = (
                    ((antenna2.0 as i32) + diff_col),
                    ((antenna2.1 as i32) + diff_row),
                );
                for antinode in vec![antinode1, antinode2].into_iter() {
                    if antinode.0 >= 0
                        && antinode.0 < (num_of_cols as i32)
                        && antinode.1 >= 0
                        && antinode.1 < (num_of_rows as i32)
                    {
                        unique_antinodes.insert(antinode);
                    }
                }
            }
        }
    }
    // count up all the unique antinodes
    unique_antinodes.len()
}

pub fn day8_part2(inp: &str) -> usize {
    let grid = inp.lines().collect::<Vec<_>>();
    let num_of_rows = grid.len();
    let num_of_cols = grid[0].chars().count();

    // find all antennas and group by frequency
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    grid.iter().enumerate().for_each(|(row_num, &row)| {
        row.chars().enumerate().for_each(|(col_num, chr)| {
            if chr != '.' {
                antennas
                    .entry(chr)
                    .and_modify(|antenna_list| antenna_list.push((col_num, row_num)))
                    .or_insert(vec![(col_num, row_num)]);
            }
        });
    });
    // for each frequency, find all the antinodes
    let mut unique_antinodes = HashSet::new();
    for (_, antennas_vec) in antennas.iter() {
        // skip any antennas that are on their own
        if antennas_vec.len() < 2 {
            continue;
        }
        antennas_vec.iter().for_each(|a| {
            unique_antinodes.insert((a.0 as i32, a.1 as i32));
        });
        for (n, antenna1) in antennas_vec.iter().enumerate() {
            for antenna2 in antennas_vec.iter().skip(n + 1) {
                let diff_col = (antenna2.0 as i32) - (antenna1.0 as i32);
                let diff_row = (antenna2.1 as i32) - (antenna1.1 as i32);

                let mut antinode1 = (
                    ((antenna1.0 as i32) - diff_col),
                    ((antenna1.1 as i32) - diff_row),
                );
                while antinode1.0 >= 0
                    && antinode1.0 < (num_of_cols as i32)
                    && antinode1.1 >= 0
                    && antinode1.1 < (num_of_rows as i32)
                {
                    unique_antinodes.insert(antinode1);
                    antinode1 = ((antinode1.0 - diff_col), (antinode1.1 - diff_row));
                }

                let mut antinode2 = (
                    ((antenna2.0 as i32) + diff_col),
                    ((antenna2.1 as i32) + diff_row),
                );
                while antinode2.0 >= 0
                    && antinode2.0 < (num_of_cols as i32)
                    && antinode2.1 >= 0
                    && antinode2.1 < (num_of_rows as i32)
                {
                    unique_antinodes.insert(antinode2);
                    antinode2 = ((antinode2.0 + diff_col), (antinode2.1 + diff_row));
                }
            }
        }
    }
    // count up all the unique antinodes
    unique_antinodes.len()
}

pub fn day9_part1(inp: &str) -> usize {
    let mut num_of_free_spaces = 0;
    let mut disk: Vec<i32> = Vec::new();
    for (n, digit) in inp
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if n % 2 == 0 {
            disk.append(&mut vec![(n / 2) as i32; digit as usize]);
        } else {
            disk.append(&mut vec![-1; digit as usize]);
            num_of_free_spaces += digit;
        }
    }
    let target_disk_len = disk.len() - num_of_free_spaces as usize;
    while disk.len() > target_disk_len {
        let file_block = disk.pop().unwrap();
        if file_block == -1 {
            continue;
        }
        // could optimise by skipping ahead and not always starting at the beginning.
        // But this implementation is fast enough in release mode.
        let index = disk.iter().position(|f| *f == -1).unwrap();
        disk[index] = file_block;
    }
    disk.iter()
        .enumerate()
        .map(|(n, &file_id)| n * file_id as usize)
        .sum()
}

pub fn day9_part2(inp: &str) -> usize {
    let mut disk: Vec<i32> = Vec::new();
    // vec of (start_pos_in_disk, length_of_file). Position in this vec is the file ID.
    let mut files = Vec::new();
    for (n, digit) in inp
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if n % 2 == 0 {
            files.push((disk.len(), digit));
            disk.append(&mut vec![(n / 2) as i32; digit as usize]);
        } else {
            disk.append(&mut vec![-1; digit as usize]);
        }
    }
    for (file_id, &(file_start_pos, file_length)) in files.iter().enumerate().skip(1).rev() {
        for (n, window) in disk.windows(file_length as usize).enumerate() {
            if n >= file_start_pos {
                break;
            }
            if window.iter().all(|&f| f == -1) {
                for item in disk.iter_mut().skip(n).take(file_length as usize) {
                    *item = file_id as i32;
                }
                for item in disk
                    .iter_mut()
                    .skip(file_start_pos)
                    .take(file_length as usize)
                {
                    *item = -1;
                }
                while disk[disk.len() - 1] == -1 {
                    disk.pop();
                }
                break;
            }
        }
    }
    disk.iter()
        .enumerate()
        .map(|(n, &file_id)| {
            if file_id == -1 {
                0
            } else {
                n * file_id as usize
            }
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

    #[test]
    fn day6() {
        let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(day6_part1(test_input), 41);
        assert_eq!(day6_part2(test_input), 6);
    }

    #[test]
    fn day7() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(day7_part1(test_input), 3749);
        assert_eq!(day7_part2(test_input), 11387);
    }

    #[test]
    fn day8() {
        let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(day8_part1(test_input), 14);
        assert_eq!(day8_part2(test_input), 34);
    }

    #[test]
    fn day9() {
        let test_input = "2333133121414131402";

        assert_eq!(day9_part1(test_input), 1928);
        assert_eq!(day9_part2(test_input), 2858);
    }
}
