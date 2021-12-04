use std::collections::HashMap;

use crate::problem::Problem;

pub struct BingoBoard {
    actual_board: HashMap<i64, (usize, usize)>,
    marked_board: HashMap<(usize, usize), bool>,
    board_size: usize,
}

impl BingoBoard {
    pub fn new(board: &Vec<Vec<i64>>) -> BingoBoard {
        let mut actual_board: HashMap<i64, (usize, usize)> = HashMap::new();
        let mut marked_board: HashMap<(usize, usize), bool> = HashMap::new();

        board.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, num)| {
                actual_board.insert(num.clone(), (row_index.clone(), col_index.clone()));
                marked_board.insert((row_index.clone(), col_index.clone()), false);
            })
        });

        BingoBoard {
            actual_board,
            marked_board,
            board_size: board.len(),
        }
    }

    pub fn mark_value(&mut self, num: &i64) {
        if self.actual_board.contains_key(&num) {
            (*self.marked_board.get_mut(&self.actual_board[&num]).unwrap()) = true;
        }
    }

    fn check_row_solved(&self, row_index: usize) -> bool {
        let mut col_index: usize = 0;
        while col_index < self.board_size {
            if self.marked_board[&(row_index, col_index)] != true {
                return false;
            }
            col_index += 1;
        }
        return true;
    }

    fn check_col_solved(&self, col_index: usize) -> bool {
        let mut row_index: usize = 0;
        while row_index < self.board_size {
            if self.marked_board[&(row_index, col_index)] != true {
                return false;
            }
            row_index += 1;
        }
        return true;
    }

    pub fn solved(&self) -> bool {
        let mut index: usize = 0;
        while index < self.board_size {
            if self.check_row_solved(index) || self.check_col_solved(index) {
                return true;
            }
            index += 1;
        }
        return false;
    }

    pub fn unmarked_total(&self) -> i64 {
        let mut total = 0;
        self.actual_board.iter().for_each(|(value, position)| {
            if self.marked_board[position] == false {
                total += value;
            }
        });
        return total;
    }
}

pub struct Problem04 {}

impl Problem04 {
    pub fn new() -> Problem04 {
        Problem04 {}
    }

    fn parse(&self, input: String) -> (Vec<i64>, Vec<BingoBoard>) {
        let mut lines = input.lines();
        let numbers_to_be_called: Vec<i64> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        let mut boards: Vec<BingoBoard> = Vec::new();
        while !lines.next().is_none() {
            let mut board_raw: Vec<&str> = vec![
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
            ];
            let board: Vec<Vec<i64>> = board_raw
                .iter_mut()
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|val| val.parse::<i64>().unwrap())
                        .collect()
                })
                .collect();
            boards.push(BingoBoard::new(&board));
        }
        return (numbers_to_be_called, boards);
    }

    fn solve_actual(&self, bingo_numbers: &Vec<i64>, bingo_boards: &mut Vec<BingoBoard>) -> i64 {
        for number in bingo_numbers {
            for board in &mut *bingo_boards {
                board.mark_value(number);
                if board.solved() {
                    return number * board.unmarked_total();
                }
            }
        }
        return 0;
    }
}

impl Problem for Problem04 {
    fn solve(&self) {
        let input = get_input!("./inputs/problem_04.txt");

        let (bingo_numbers, mut bingo_boards) = self.parse(input);

        let result = self.solve_actual(&bingo_numbers, &mut bingo_boards);
        println!("Day 4 Answer:");
        println!(" - Part 1: {:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem04::new();
        let input = get_input!("./inputs/problem_04_part1_example.txt");
        let (bingo_numbers, mut bingo_boards) = problem.parse(input);
        assert_eq!(
            problem.solve_actual(&bingo_numbers, &mut bingo_boards),
            4512
        );
    }
}
