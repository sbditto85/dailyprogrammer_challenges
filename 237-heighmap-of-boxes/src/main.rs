fn main() {
    let box_string = "+--------------------------------------------------------------+
|                                                              |
|   +-------------------------------+          +-------+       |
|   |                               |          |       |       |
|   |                               |          |       |       |
|   |     +----------------+        |          |       |       |
|   |     |                |        |          +-------+       |
|   |     |                |        |                          |
|   |     |                |        |          +-------+       |
|   |     +----------------+        |          |       |       |
|   |                               |          |       |       |
|   |                               |          |       |       |
|   +-------------------------------+          +-------+       |
|                                                              |
+--------------------------------------------------------------+";

    // let box_string = "+------+
// |  +-+ |
// |  | | |
// |  +-+ |
// +------+";

    let mut boxes = vec![];
    for line in box_string.lines() {
        let mut string = line.chars().collect::<Vec<char>>();
        boxes.push(string);
    }

    fill_box(&mut boxes, 0, 0, '#');

    for y in {0..boxes.len()} {
        for x in {0..boxes[y].len()} {
            print!("{}", boxes[y][x]);
        }
        println!("");
    }
    println!("Hello, world!");
}

fn get_next_lvl_char(c:char) -> char {
    match c {
        '#' => '=',
        '=' => '-',
        '-' => '.',
        _   => ' ',
    }
}


fn fill_box(boxes: &mut Vec<Vec<char>>, startx: usize, starty: usize, c: char) {
    let mut x = startx;
    let mut y = starty;
    if y < boxes.len() && x < boxes[y].len() && boxes[y][x] == '+' {
        y += 1;
        x += 1;
    }
    'outer: for cury in {y..boxes.len()} {
        'inner: for curx in {x..boxes[y].len()} {
            println!("{} {} -> {}", curx, cury, boxes[cury][curx]);
            match boxes[cury][curx] {
                '+' => if curx != startx && curx + 1 < boxes[y].len() && boxes[y][curx+1] == '-' { fill_box(boxes, curx, cury, get_next_lvl_char(c)) },
                ' ' => boxes[cury][curx] = c,
                '|' => if curx != startx { break 'inner },
                // '|' => break 'inner,
                _ => ()
            }
        }
    }
}
