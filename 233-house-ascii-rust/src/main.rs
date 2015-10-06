#![feature(slice_splits)]

use std::env;
use std::io::{self, BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;

#[cfg(test)]
mod test;

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>, io::Error> {
    let f = try!(File::open(filename));
    let f = BufReader::new(f);
    
    Ok(f.lines())
}

fn main() {
    /////////////////////////////
    // Begin imperative dangerous wrapper the interfaces with outside
    /////////////////////////////

    let mut args = env::args();
    let lines = args.nth(1).and_then(|f| read_lines(&f).ok()).unwrap();

    let mut l = vec![];
    for line in lines {
        l.push(line.unwrap());
    }

    //get rid of line count for now
    let ( _ /*num_lines*/, lines) = l.split_first()
        .and_then(
            |(first, rest)| {
                //create vecutor of chars for each line
                let mut lines: Vec<Vec<char>> = vec![];
                for line in rest {
                    lines.push(line.chars().collect());
                }
                Some((first.parse::<i32>().unwrap(),lines))
            }).unwrap();


    /////////////////////////////
    // Begin safe funcational core
    /////////////////////////////
    let (roof_size, mut output) = generate_output_grid(&lines);
    generate_roof(&lines, roof_size, &mut output);
}

//////////////////////////////////////////////////////////////////
// create an output grid to populate later with according to lines
//////////////////////////////////////////////////////////////////
fn generate_output_grid<'a>(lines: &Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let mut max_width = 0;
    let max_height = lines.len() * 2 + 1;
    for l in lines {
        if l.len() > max_width {
            max_width = l.len();
        }
    }

    // let mut max: usize = if max_height > max_width { max_height } else { max_width };
    
    let mut v = vec![];
    
    let mut first = true;
    let mut level = vec![];
    for _ in 0..max_width {
        if first {
            level.push(' ');
            first = false;
        }
        level.push(' ');
        level.push(' ');
        level.push(' ');
        level.push(' ');
    }
    
    let max_roof_lines = level.len() / 2;

    let max = max_height + max_roof_lines;

    for _ in 0..max {
        v.push(level.clone());
    }

    (max_roof_lines, v)
}

//////////////////////////////////
// Lets take care of the roof baby
//////////////////////////////////
fn generate_roof(lines: &Vec<Vec<char>>, roof_size: usize, output: &mut Vec<Vec<char>>) {
    let mut start_x = 0;
    let mut start_y = 0;
    let max_y = lines.len();
    let max_x = lines[0].len();
    let mut end_x;
    let mut processed_x = 0;
    // println!("{} {}", max_x, max_y);

    for i in 0..max_x {
        //  println!("{} -> {}, {}", i, processed_x, if processed_x > 4 { (processed_x / 4) } else { 0 });
        if processed_x > 4 && i < (processed_x / 4) {
            continue;
        }
        // Find the start of a roof
        for j in 0..max_y {
            if lines[j][i] == '*' {
                start_x = i * 4;
                start_y = j;
                break;
            }
        }

        // Find how long the roof is
        end_x = i;
        for (k, l) in lines[start_y].iter().enumerate().skip(i) {
            if *l == '*' {
                if start_y > 0 && lines[start_y-1][k] == '*' {
                    break;
                }
                end_x = k;
            } else {
                break;
            }
        }
        end_x = (end_x + 1) * 4;
        processed_x = end_x;

        let mut current_y = roof_size + start_y * 2;
        // println!("rs: {} sx:{} sy:{} ex:{} cy: {}", roof_size, start_x, start_y, end_x, current_y);

        // Get in position for the first part of roof
        start_x += 1;
        end_x -= 1;
        current_y -= 1;
        while start_x < end_x {
            // up one right one from left put /
            output[current_y][start_x] = '/';
            // up one left one from right put \
            output[current_y][end_x] = '\\';

            // println!("{} -> {} {}", current_y, start_x, end_x);
            
            //adjust pos for the next part of roof
            start_x += 1;
            end_x -= 1;
            current_y -= 1;
        }
        // when left and right are same put A
        output[current_y][start_x] = 'A';
        //repeat until found all roofs
    }
}

//////////////////////////////////
// Lets take care of the base now
//////////////////////////////////
fn generate_base(lines: &Vec<Vec<char>>, output: &mut Vec<Vec<char>>) {
    let max_x = lines[0].len();
    let max_y = lines.len();
    // Start at base and work up
    let mut output_y = output.len() - 1;
    let mut output_x = 0;

    for j in {0..max_y}.rev() {
        for i in {0..max_x} {
            // if its a *
            if lines[j][i] == '*' {
                //Draw the box! will smooth out later
                output[output_y][output_x] = '+';
                output[output_y - 1][output_x] = '|';
                output[output_y - 2][output_x] = '+';
                output_x += 1;
                output[output_y][output_x] = '-';
                output[output_y - 2][output_x] = '-';
                output_x += 1;
                output[output_y][output_x] = '-';
                output[output_y - 2][output_x] = '-';
                output_x += 1;
                output[output_y][output_x] = '-';
                output[output_y - 2][output_x] = '-';
                output_x += 1;
                output[output_y][output_x] = '+';
                output[output_y - 1][output_x] = '|';
                output[output_y - 2][output_x] = '+';
                //leave it at bottom right of current box
            } else {
                output_x += 4;
            }
        }
        //put it at the top left of previous row
        output_y -= 2;
        output_x = 0;
    }
}

//////////////////////////////////
// Lets smooth out the base now
//////////////////////////////////
fn smooth_base(output: &mut Vec<Vec<char>>) {
    // Start at base and work up
    let max_y = output.len();
    let max_x = output[0].len();
    
    //left to right, bottom to top
    for j in {0..max_y}.rev() {
        for i in {0..max_x} {
            // if a - and up 2 is a '-' then ' '
            if output[j][i] == '-' && j > 1 && j + 1 < max_y {
                let mut temp_j = j - 2;
                let mut has_above = false;
                while temp_j > 1 {
                    if output[temp_j][i] == '-' {
                        has_above = true;
                    }
                    temp_j -= 2;
                }
                if has_above {
                    output[j][i] = ' ';
                }
            }
            // if a + and before and after a - then convert to -
            if output[j][i] == '+' 
                && i + 1 < max_x && i > 0 
                && output[j][i + 1] == '-'
                && output[j][i - 1] == '-' {
                    output[j][i] = '-';
            }
            // if a + and above and below are | then |
            else if output[j][i] == '+' 
                && j + 1 < max_y && j > 0 
                && output[j + 1][i] == '|'
                && output[j - 1][i] == '|' {
                    output[j][i] = '|';
            }
            // if a | and before and after, above and below have - then ' '
            else if output[j][i] == '|' 
                && j + 1 < max_y && i + 1 < max_x
                && j > 0 && i > 0 
                && output[j - 1][i - 1] == '-' 
                && output[j - 1][i + 1] == '-' 
                && output[j + 1][i - 1] == '-' 
                && output[j + 1][i + 1] == '-' {
                    output[j][i] = ' ';
            }
            // otherwise leave it alone
        }
    }

}

// left: `[
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '-', '-', '-', '+'], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['+', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '+', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', '-', '-', '-', '-', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', '+', ' ', ' ', ' ', '+', ' ', ' ', ' ', '+', ' ', ' ', ' ', '+', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'], 
// ['+', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '+']
// ]`, right: `[
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '-', '-', '-', '+'], 
// [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['+', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '+', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '-', '-', '-', '+', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'], 
// ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'], 
// ['+', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '+']]
