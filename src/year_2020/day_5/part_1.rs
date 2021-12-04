use std::fs;

pub fn main() {
    let input =
        fs::read_to_string("src/year_2020/day_5/input").expect("IOError: unable to read input");
    let seats = input.lines().map(parse_line);
    let max_id = seats.fold(
        0,
        |max_id, seat| {
            if max_id < seat.id {
                seat.id
            } else {
                max_id
            }
        },
    );
    println!("{}", max_id);
}

fn parse_line(line: &str) -> Seat {
    let (raw_row, raw_col) = line.split_at(7);
    let row = parse_row(raw_row);
    let col = parse_col(raw_col);
    let id = row * 8 + col;
    Seat { id, row, col }
}

fn parse_row(raw_row: &str) -> i32 {
    raw_row
        .chars()
        .fold(BinarySearcher { low: 0, high: 127 }, |bs, c| match c {
            'F' => bs.left(),
            'B' => bs.right(),
            _ => panic!("ParseError: row characters must be 'F' or 'B'"),
        })
        .middle()
}

fn parse_col(raw_col: &str) -> i32 {
    raw_col
        .chars()
        .fold(BinarySearcher { low: 0, high: 7 }, |bs, c| match c {
            'L' => bs.left(),
            'R' => bs.right(),
            _ => panic!("ParseError: column characters must be 'R' or 'L'"),
        })
        .middle()
}

#[derive(Debug)]
struct Seat {
    id: i32,
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct BinarySearcher {
    low: i32,
    high: i32,
}

impl BinarySearcher {
    fn left(mut self) -> Self {
        self.high = self.middle() - 1;
        self
    }

    fn right(mut self) -> Self {
        self.low = self.middle();
        self
    }

    fn middle(&self) -> i32 {
        self.high - (self.distance() / 2)
    }

    fn distance(&self) -> i32 {
        self.high - self.low
    }
}
