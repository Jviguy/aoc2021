pub fn part1(data: String) -> i64 {
    let mut previous=0;
    data.split("\n")
        .filter(|line| line.len()>0)
        .map(|line| line.strip_suffix("\r").unwrap_or(line).parse::<i64>().expect(format!("{} is not a number", line).as_str()))
        .fold(0,|acc, current| {let result;if current>=previous{result=acc+1}else{result=acc};previous=current;result})-1
}

pub fn part2(data: String) -> i64 {
    let mut previous=0;
    let vec = data.split("\n")
        .filter(|line| line.len()>0)
        .map(|line| line.strip_suffix("\r").unwrap_or(line).parse::<i64>().expect(format!("{} is not a number", line).as_str()))
        .collect::<Vec<i64>>();
    vec.iter().enumerate()
        .fold(0,|acc, current| {
            if current.0+1 >= vec.len() || current.0+2 >= vec.len() {return acc}
            let mut result=acc;
            let current=vec.get(current.0).unwrap()+vec.get(current.0+1).unwrap()+vec.get(current.0+2).unwrap();
            if current>previous{result=acc+1}
            previous=current;
            result
        })-1
}