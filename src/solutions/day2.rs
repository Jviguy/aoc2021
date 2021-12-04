pub fn part1(data: String) -> i64 {
    let mut depth = 0;
    let mut x = 0;
    let lines = data.split("\n")
        .filter(|line| line.len()>0)
        .map(|line| line.strip_suffix("\r").unwrap_or(line));
    for line in lines {
        let mut words = line.split_whitespace();
        let instr = words.next().unwrap();
        let value = words.next().unwrap().parse::<i64>().unwrap();
        match instr {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" => x += value,
            _ => panic!("Unknown instruction: {}", instr),
        }
    }
    return depth*x;
}

pub fn part2(data: String) -> i64 {
    let mut depth = 0;
    let mut aim = 0;
    let mut x = 0;
    let lines = data.split("\n")
        .filter(|line| line.len()>0)
        .map(|line| line.strip_suffix("\r").unwrap_or(line));
    for line in lines {
        let mut words = line.split_whitespace();
        let instr = words.next().unwrap();
        let value = words.next().unwrap().parse::<i64>().unwrap();
        match instr {
            "up" => aim += value,
            "down" => aim -= value,
            "forward" => {x += value; depth -= aim*value;},
            _ => panic!("Unknown instruction: {}", instr),
        }
    }
    return depth*x;
}