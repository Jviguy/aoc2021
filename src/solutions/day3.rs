pub fn part1(data: String) -> i64 {
    let mut gamma_bin=String::new();
    let lines = data.split("\n")
        .filter(|line| line.len()>0)
        .map(|line| line.strip_suffix("\r").unwrap_or(line));
    for i in 0..lines.clone().next().unwrap().len() {
        let ones=lines.clone().map(|line| line.chars().nth(i).unwrap()).filter(|c| *c=='1').count();
        let zeros=lines.clone().count()-ones;
        gamma_bin.push_str(&(if ones<zeros {'1'} else {'0'}).to_string());
    }
    let gamma=i64::from_str_radix(&*gamma_bin, 2).unwrap();
    let epsilon=invert_bits(gamma);
    gamma*epsilon
}

pub fn part2(data: String) -> i64 {
    let data_lines = data.split("\n")
        .filter(|line| line.len()>0)
        .map(|line| line.strip_suffix("\r").unwrap_or(line))
        .collect::<Vec<&str>>();
    let gen_rating = iter_helper(data_lines.clone(), true);
    let scrubber_rating = iter_helper(data_lines, false);
    gen_rating*scrubber_rating
}

fn iter_helper(lines: Vec<&str>, most_mode: bool) -> i64{
    let mut i = 0;
    let mut lines = lines.clone();
    while i<12&&lines.len()!=1 {
        let ones=lines.iter().map(|line| line.chars().nth(i).unwrap()).filter(|c| *c=='1').count();
        let zeros=lines.len()-ones;
        let most_common = if ones<zeros {'1'} else {'0'};
        let least_common = if ones>=zeros {'1'} else {'0'};
        lines=lines.into_iter().filter(|line| if most_mode {line.chars().nth(i).unwrap()==most_common} else {line.chars().nth(i).unwrap()==least_common}).collect();
        i += 1;
    }
    i64::from_str_radix(&*lines.join(""), 2).unwrap()
}

fn invert_bits(num: i64) -> i64 {
    let mut x = num;
    let mut i = 1;
    while i<=16 {
        x|=x>>i;
        i*=2;
    };
    num ^ x
}