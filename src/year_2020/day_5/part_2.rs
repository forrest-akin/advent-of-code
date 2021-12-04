use std::fs;

pub fn main() {
    let input =
        fs::read_to_string("src/year_2020/day_5/input").expect("IOError: unable to read input");
    let mut seats: Vec<Seat> = input.lines().map(parse_line).collect();

    seats.sort_by_key(|seat| seat.id);

    println!("{:?}", find_id_by_window(seats));
}

fn find_id_by_diff(seats: Vec<Seat>) -> i32 {
    let sum = match (seats.first(), seats.last()) {
        (Some(first), Some(last)) => (first.id..=last.id).sum(),
        _ => 0,
    };
    let partial: i32 = seats.iter().map(|seat| seat.id).sum();

    sum - partial
}

fn find_id_by_window(seats: Vec<Seat>) -> i32 {
    seats
        .windows(2)
        .find_map(|window| match window {
            [prev, next] => {
                let seat = prev.next();
                if seat.eq(next) {
                    None
                } else {
                    Some(seat)
                }
            }
            _ => None,
        })
        .expect("Your seat was not found!")
        .id
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

#[derive(Debug, Eq)]
struct Seat {
    id: i32,
    row: i32,
    col: i32,
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Seat {
    fn next(&self) -> Self {
        if self.col == 7 {
            let row = self.row + 1;
            Seat {
                id: row * 8,
                row,
                col: 0,
            }
        } else {
            let col = self.col + 1;
            Seat {
                id: self.row * 8 + col,
                row: self.row,
                col,
            }
        }
    }
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
