

fn main() {
    //                 1    2    3    4    5    6         7    8    9             10
    let throws = vec!['X', 'X', 'X', 'X', 'X', 'X', '8', '/', 'X', 'X', 'X', '7', '/'];
    let score = score_game(throws);
    println!("Scored: {}", score);
}

fn parse_points(c: char) -> Option<i32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),

        '-' => Some(0),
        '/' => Some(10),
        'X' => Some(10),
        _   => None
    }
}

fn score_game(throws: Vec<char>) -> i32 {
    let mut idx = 0;
    let mut score = 0;
    for _ in {0..10} {
        match throws[idx] {
            'X'  => {
                score += 10;
                match throws[idx + 2] {
                    '/' => score += 10,
                    second   => {
                        match (parse_points(throws[idx + 1]), parse_points(second)) {
                            (Some(i), Some(j)) => score += i + j,
                            _                  => panic!("Coudln't parse the number")
                        }
                    }
                }
                idx += 1;
            }, // 'X'
            first  => { // cant be a / as its first throw
                match throws[idx + 1] {
                    '/' => match parse_points(throws[idx + 2]) {
                        Some(i) => score += 10 + i,
                        None    => panic!("Couldn't parse the number"),
                    },
                    second   => {
                        match (parse_points(first), parse_points(second)) {
                            (Some(i), Some(j)) => score += i + j,
                            _                  => panic!("Coudln't parse the number")
                        }
                    }
                }
                idx += 2;
            }
        }
        // println!("frame: {}, score: {}", frame, score);
    }
    score
}


#[test]
fn perfect_game() {
    //                 1    2    3    4    5    6    7    8    9   10   10   10
    let throws = vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'];

    let score = score_game(throws);

    assert_eq!(score, 300);
}


#[test]
fn first_game() {
    //                 1    2    2    3    4    4    5    5    6    6    7    8    8    9    9   10   10   10
    //                 20        20   15        5        19         9   19         9         1             20
    //                 20        40   55       60        79        88  107       116       117            137
    let throws = vec!['X', '-', '/', 'X', '5', '-', '8', '/', '9', '-', 'X', '8', '1', '1', '-', '4', '/', 'X'];

    let score = score_game(throws);

    assert_eq!(score, 137);
}

#[test]
fn second_game() {
    //                 1    1    2    2    3    4    4    5    5    6    7    8    8    9    9   10   10   10
    //                      8         8   19         9        20   23   18         8         9             18
    //                      8        16   35        44        64   87  105       113       122            140
    let throws = vec!['6', '2', '7', '1', 'X', '9', '-', '8', '/', 'X', 'X', '3', '5', '7', '2', '5', '/', '8'];
    let score = score_game(throws);

    assert_eq!(score, 140);
}
