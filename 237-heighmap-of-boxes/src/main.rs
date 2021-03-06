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

    let box_string = "+-----------------------------------------------------------------------+
|     +--------------------------------------------------------------+  |
|     |      +-----------------------------------------------------+ |  |
|     |      |         +-----------------------------------------+ | |  |
|     |      |         |           +--------------+ +----------+ | | |  |
|     |      |         |           |              | |          | | | |  |
|     |      |         |           |              | |          | | | |  |
|     |      |         |           |              | |          | | | |  |
|     |      |         |           +--------------+ +----------+ | | |  |
|     |      |         |                                         | | |  |
|     |      |         +-----------------------------------------+ | |  |
|     |      |                                                     | |  |
|     |      |                                                     | |  |
|     |      +-----------------------------------------------------+ |  |
|     |                                                              |  |
|     +--------------------------------------------------------------+  |
|                                                                       |
|                                                                       |
|                                                                       |
+-----------------------------------------------------------------------+";

    // let box_string = "+------+
// |  +-+ |
// |  | | |
// |  +-+ |
// +------+";

    let mut boxes = vec![];
    for line in box_string.lines() {
        let string = line.chars().collect::<Vec<char>>();
        boxes.push(string);
    }

    fill_box(&mut boxes, 0, 0, '#');

    for y in {0..boxes.len()} {
        for x in {0..boxes[y].len()} {
            print!("{}", boxes[y][x]);
        }
        println!("");
    }
}

fn get_next_lvl_char(c:char) -> char {
    match c {
        '#' => '=',
        '=' => '-',
        '-' => '.',
        _   => ' ', //Currently the algo makes it so any blank space will get filled ... doesn't work when a lower level goes through that box :(
    }
}


fn fill_box(boxes: &mut Vec<Vec<char>>, startx: usize, starty: usize, c: char) {
    let mut x = startx;
    let mut y = starty;
    //find the max x and y
    let mut max_x = 0;
    let mut max_y = 0;
    
    if y < boxes.len() && x < boxes[y].len() && boxes[starty][startx] == '+' {
        y += 1;
        x += 1;
        //find max x
        for tmpx in {x..boxes[starty].len()} {
            if boxes[starty][tmpx] == '+' {
                max_x = tmpx;
                break;
            }
        }
        //find max y
        for tmpy in {y..boxes.len()} {
            if  boxes[tmpy][startx] == '+' {
                max_y = tmpy;
                break;
            }
        }
    } else {
        return
    }
    // Find and fill all boxes in side the current one
    let mut cur_y = y;
    while cur_y < max_y {
        let mut cur_x = x;
        while cur_x < max_x {
            match boxes[cur_y][cur_x] {
                '+' => if cur_x + 1 < max_x
                        && cur_y + 1 < max_y
                        && boxes[cur_y][cur_x+1] == '-'
                        && boxes[cur_y+1][cur_x] == '|' {
                            // 
                            fill_box(boxes, cur_x, cur_y, get_next_lvl_char(c))
                       },
                ' ' => boxes[cur_y][cur_x] = c,
                '|' => {
                    cur_x += 1;
                    while cur_x < max_x && boxes[cur_y][cur_x] != '|' {
                        cur_x +=1;
                    }
                }
                _   => ()
            }
            cur_x += 1;
        }
        cur_y += 1;
    }
}
