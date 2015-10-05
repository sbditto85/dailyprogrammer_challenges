use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn get_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let mut lines = vec![];
    
    let f = try!(File::open(filename));
    let f = BufReader::new(f);

    for line in f.lines().skip(1) {
        lines.push(line.unwrap());
    }
    
    Ok(lines)
}

fn main() {

    let filename =  env::args().nth(1).unwrap(); //Blow up if filename isn't present
    
    let lines = get_file_lines(&filename).unwrap(); //Blow up if reading the file fails

    for i in lines {
        let mut str = i.trim_left_matches('(');
        str = str.trim_right_matches(')');
        let parts: Vec<&str> = str.split(',').collect();
        let first_num = parts[0].parse::<i32>().unwrap(); //Blow up if its not a number
        let second_num = parts[1].parse::<i32>().unwrap(); //Blow up if its not a number

        if is_ruth_aaron(first_num, second_num) {
            println!("({},{}) VALID", first_num, second_num);
        } else {
            println!("({},{}) NOT VALID", first_num, second_num);
        }
    }
}

fn is_prime(i: i32) -> bool {
    for n in {2..i} {
        if i % n == 0 {
            return false
        }
    }
    true
}

fn prime_factors(i: i32) -> Vec<i32> {
    let mut v = vec![];
    for j in {2..i + 1} {
        if i % j == 0 && is_prime(j) {
            v.push(j);
        }
    }
    v
}

fn is_ruth_aaron(i: i32, j: i32) -> bool {
    // get vec of prime_factors for each number
    // sum each vec
    // return sums equal each other
    prime_factors(i).iter().fold(0, |acc, n| acc + n) == prime_factors(j).iter().fold(0, |acc, n| acc + n)
}

#[test]
fn should_be_ruth_aaron() {
    let result = is_ruth_aaron(5,6);
    assert!(result == true, "is_ruth_aaron(5,6)");
    let result = is_ruth_aaron(714,715);
    assert!(result == true, "is_ruth_aaron(714,715)");
}
