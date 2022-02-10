
use std::{sync::Mutex};
#[derive(Debug)]
struct Board {
    pub nodes: Vec<Mutex<Vec<Node>>>,
}
#[derive(Debug)]
struct Node {
    pub value: usize,
    pub marked: bool,
}

impl Board {
    pub fn new(lines: &[&str]) -> Board {
        let mut nodes: Vec<Mutex<Vec<Node>>> = Vec::new();
        for i in 0..5 {
            let nested = Mutex::new(Vec::new());
            for (idx, j) in lines[i].split(" ").enumerate() {
                let value = j.parse::<usize>().unwrap();
                nested.lock().unwrap().insert(idx, Node { value: value, marked: false });
            }
            nodes.insert(i, nested);
        }
        Board { nodes }
    }
    
    pub fn print(&self) {
        for j in self.nodes.iter() {
            for l in j.lock().unwrap().iter() {
                if !l.marked {
                    print!("{} ", l.value);
                } else {
                    print!("{} ", "X");
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    // returns a bool indicating if this is a winner
    pub fn mark(&mut self, value: usize) -> bool {
        for j in self.nodes.iter_mut() {
            for k in j.lock().unwrap().iter_mut() {
                if k.value == value {
                    k.marked = true;
                    break; 
                }
            }
        }
        self.winner()
    }

    pub fn winner(&self) -> bool {
        for row in self.nodes.iter() {
            let mut row_total = 0;
            for i in row.lock().unwrap().iter() {
                row_total += i.marked as i32;
            }
            if row_total >= 5 {return true;}
        }
        for col in 0..5 {
            let mut col_total = 0;
            for row in 0..5 {
                col_total += self.nodes.get(row).unwrap().lock().unwrap().get(col).unwrap().marked as i32;
            }
            if col_total >= 5 {return true;}
        }
        false
    }
}


pub fn part1(data: String) -> usize {
    //replace all "  " with " " and split on "\n" then filter all empty strings and strip "\r" or unwrap line
    let data = data.replace("\n ", "\n")
        .replace("  ", " ");
    let mut lines = data
        .split("\n")
        .map(|line| line.strip_prefix(" ").unwrap_or(line).strip_suffix("\r").unwrap_or(line))
        .filter(|line| line.len()>0);
    let guesses = lines.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let lines = lines.collect::<Vec<&str>>();
    let chunks = lines.chunks(5);
    let mut boards = chunks.map(|chunk| Board::new(chunk)).collect::<Vec<Board>>();
    for guess in guesses.clone() {
        for board in boards.iter_mut() {
            if board.mark(guess) {
                println!("Winner");
                return board.nodes
                .iter()
                .map(|j| j.lock().unwrap().iter().filter(|k| !k.marked)
                .map(|k| k.value).sum::<usize>())
                .sum::<usize>()*guess
            }
        }
    }
    0
}