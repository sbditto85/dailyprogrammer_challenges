//http://rustbyexample.com/trait/iter.html found something that does what i was thinking
struct Fib {
    curr: u64,
    next: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let curr = self.curr; //So that we start with 0 otherwise we start with 'n'
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(curr)
    }
}

fn fib_base_n(n: u64) -> Fib {
    Fib{ curr: 0, next: n}
}

fn find_smallest_base(n: u64) -> u64 {
    let mut base: u64 = 1;

    'outer: loop {        
        if n % base == 0 {
            for i in fib_base_n(base) {
                if i == n {
                    break 'outer;
                } else if i > n {
                    break;
                }
            }
        }
        base += 1;
    }

    base
}

fn main() {
    // println!("Hello, world!");
    // for u in fib_base_n(1).take(10) {
    //     println!("{}", u);
    // }

    // // turns out base 3 is just base 1 times 3
    // for u in fib_base_n(3).take(10) {
    //     println!("{} {}", u, u as f64 / 3 as f64);
    // }

    // let n = 0;
    // let n = 578;
    // let n = 123456789;
    let n = 38695577906193299;
    let base = find_smallest_base(n);
    for i in fib_base_n(base) {
        if i > n {
            break;
        }
        print!("{} ", i);
    }
    println!("");
}

#[test]
fn fib_first_ten_base_one() {
    let itr = fib_base_n(1).take(10).collect::<Vec<u64>>();
    assert_eq!(itr, vec![0,1,1,2,3,5,8,13,21,34]);
}

#[test]
fn fib_first_ten_base_three() {
    let itr = fib_base_n(3).take(10).collect::<Vec<u64>>();
    assert_eq!(itr, vec![0,3,3,6,9,15,24,39,63,102]);
}

#[test]
fn get_fib_small_base() {
    assert_eq!(1, find_smallest_base(0));
    assert_eq!(17, find_smallest_base(578));
    assert_eq!(41152263, find_smallest_base(123456789));
    assert_eq!(7, find_smallest_base(38695577906193299));
}
