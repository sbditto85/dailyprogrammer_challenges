extern crate rand;

use std::fmt;
use rand::thread_rng;
use rand::Rng;


enum BagItems {
    O,I,S,Z,L,J,T
}
use BagItems::*;

impl fmt::Display for BagItems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_rep = match self {
            &O => 'O',
            &I => 'I',
            &S => 'S',
            &Z => 'Z',
            &L => 'L',
            &J => 'J',
            &T => 'T'
        };
        write!(f, "{}", char_rep)
    }
}

fn shuffle<T>(slice: &mut [T]) {
    thread_rng().shuffle(slice);
}

fn main() {
    let poss_values = vec![O,I,S,Z,L,J,T];
    
    let mut idxs = [0,1,2,3,4,5,6];
    shuffle(&mut idxs);

    let mut counter = 0;
    //until were done
    for _ in {0..50} {
        //foreach index and print out the BagItem for the index
        print!("{}", poss_values[idxs[counter]]);
        counter += 1;
        if counter > 6 {
            //shuffle index
            shuffle(&mut idxs);
            counter = 0;
            print!(" ");
        }
    }
    println!("");
}
