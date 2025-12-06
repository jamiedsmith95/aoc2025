use fancy_regex::Regex;
use std::fs;

fn d2p1(file_name: String) {
    let file_contents = fs::read_to_string(file_name).expect("Should be able to read file");
    let instructions = file_contents.lines();
    let mut total = 0;
    instructions.for_each(|instruct| {
        instruct.split(",").for_each(|range| {
            eprintln!("Range: {range}");
            let boundstr = range
                .split_once("-")
                .expect("Should be a range e.g. 223-365");
            let lower: u64 = boundstr
                .0
                .parse()
                .expect("range should be integer to integer;");
            let upper: u64 = boundstr
                .1
                .parse()
                .expect("range should be integer to integer;");
            let seq = lower..=upper;
            seq.for_each(|num| {
                let numstr = num.to_string();
                if numstr.len() % 2 == 0 {
                    let halfs = numstr.split_at(numstr.len() / 2);
                    if halfs.0 == halfs.1 {
                        total += num
                    };
                }
            });
        });
    });
    eprintln!("Total: {total}");
}

fn check_for_repeating_even(numstr: &str) -> bool {
    if numstr.len() % 2 == 0 {
        let halfs = numstr.split_at(numstr.len() / 2);
        if halfs.0 == halfs.1 {
            return true;
        } else {
            return check_for_repeating_even(halfs.0) || check_for_repeating_even(halfs.1);
        }
    } else {
        return false;
    };
}

fn check_for_repeating_odd(numstr: &str, re: &Regex) -> bool {
    //262728262728262728

    let result = re.is_match(numstr).expect("Regex should be valid");
    return result;
}

fn d2p2(file_name: String) {
    let re = Regex::new(r"^([0-9]+)(\1)+$").unwrap();
    let file_contents = fs::read_to_string(file_name).expect("Should be able to read file");
    let instructions = file_contents.lines();
    let mut total = 0;
    instructions.for_each(|instruct| {
        instruct.split(",").for_each(|range| {
            eprintln!("Range: {range}");
            let boundstr = range
                .split_once("-")
                .expect("Should be a range e.g. 223-365");
            let lower: u64 = boundstr
                .0
                .parse()
                .expect("range should be integer to integer;");
            let upper: u64 = boundstr
                .1
                .parse()
                .expect("range should be integer to integer;");
            let seq = lower..=upper;
            seq.for_each(|num| {
                let numstr = num.to_string();
                if check_for_repeating_odd(&numstr, &re) {
                    total += num
                };
            });
        });
    });
    eprintln!("Total: {total}");
}

fn d1p1(file_name: String) {
    let file_contents = fs::read_to_string(file_name).expect("Should be able to read file");
    let mut current = 50;
    let mut count = 0;
    let instructions = file_contents.lines();
    instructions.for_each(|instruct| {
        let direction = instruct
            .chars()
            .nth(0)
            .expect("Line should have length greater than 0.");
        let mut jump: u16 = instruct
            .split_at(1)
            .1
            .parse()
            .expect("Instruct should be a valid integer.");
        let rem = jump % 100;
        let skips = (jump - rem) / 100;
        jump = rem;
        count += skips;
        eprintln!("SKIPS {skips}");
        eprintln!("Direction: {direction} Length: {jump}");
        if direction == 'L' {
            if jump > current {
                if current != 0 {
                    count += 1
                };
                jump -= (current) + 1;
                current = 99;
            }
            current -= jump;
        } else if direction == 'R' {
            if jump > (99 - current) {
                jump = jump - (99 - current) - 1;
                current = 0;
                count += 1;
            }
            current += jump;
        }
        if current == 0 && jump != 0 {
            count += 1;
        };
        eprintln!("CURRENT: {current}");
    });
    eprintln!("0 hit {count} times");
}

fn d3p1(file_name: String) {
    let file_contents = fs::read_to_string(file_name).expect("Should be able to read file");
    let instructions = file_contents.lines();
    let mut total = 0;
    instructions.for_each(|line| {
        // let last: u32 = line.chars().last().expect("All should be chars.").to_digit(10).expect("Should be a digit");
        let max = line
            .split_at(line.len() - 1)
            .0
            .chars()
            .max()
            .expect("Should have a max")
            .to_digit(10)
            .expect("Should be digit");
        let max_idx = line
            .find(&max.to_string())
            .expect("Line should contain its maximum");
        let second: u32 = line
            .split_at(max_idx + 1)
            .1
            .chars()
            .max()
            .expect("Should have a max")
            .to_digit(10)
            .expect("Should be a digit");
        eprintln!("first {max} second {second}");
        total += (max * 10) + second;
    });
    eprintln!("Total: {total}");
}

fn d3p2(file_name: String) {
    let file_contents = fs::read_to_string(file_name).expect("Should be able to read file");
    let instructions = file_contents.lines();
    let mut total: u64 = 0;
    instructions.for_each(|line| {
        // let last: u32 = line.chars().last().expect("All should be chars.").to_digit(10).expect("Should be a digit");
        let mut max_idx: i16 = -1;
        let mut line_max = 0;
        for i in 0..12 {
            let split = line.split_at((max_idx + 1 as i16) as usize);
            let max = split
                .1
                .split_at(split.1.len() -(11-i))
                .0
                .chars()
                .max()
                .expect("Should have a max")
                .to_digit(10)
                .expect("Should be digit") as u64;
            max_idx += split.1
                .find(&max.to_string())
                .expect("Line should contain its maximum") as i16 + 1;
            line_max += max * 10_u64.pow(11 - i as u32) as u64;
        }
        total += line_max;
    });
    eprintln!("Total: {total}");
}



fn d4p1(file_name: String) {
    let file_contents = fs::read_to_string(file_name).expect("Should be able to read file");
    let end_line = file_contents.find("\n").expect("Should have end line");
    let empty_line = ".".repeat(end_line + 1) + "\n";
    let modified = empty_line.clone() + &file_contents + &empty_line;
    eprint!("file: \n{modified}");
    let mut instructions = modified.lines();
    let new_instructions: Vec<String> = instructions.map(|line| {
        let new_line = ".".to_owned() + line + &".";
        eprintln!("{new_line}");
        new_line
    }).collect();
    let mut line_idx = 0;
    let mut total = 0;
    new_instructions.iter().for_each(|line| {
        for idx in 1..line.len() {
            eprintln!("idx {idx}");
            if line.split_at(idx).1.starts_with("@") {
                eprintln!("Line {line}");
                let block = new_instructions.split_at(line_idx - 1).1.split_at(3).0;
                eprintln!("block {:?}",block);
                let mut count = 0;
                block.iter().for_each(|chunk| {
                    chunk.split_at(idx-1).1.split_at(3).0.chars().for_each(|ch| {
                        if ch.to_string().as_str() == "@"  {
                            count += 1;
                        }
                    });
                    eprintln!("LEN SHOULD BE 3 {count}");
                });
                if count <= 4 {total += 1};
            }
        }
        line_idx += 1;
    });
    eprintln!("TOTAL: {total}");
}

fn main() {
    d4p1("data/d4p1".to_owned());
}
