extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let testslice = ['c','v','c','V','v','V','c','C'];

    if verify_cv(&testslice) {
        println!("{}", replace_cv(&testslice));
    } else {
        println!("String is invalid");
    }
}

fn verify_cv(s: &[char]) -> bool {
    s.iter().filter(|&x| *x != 'c' && *x != 'v' && *x != 'C' && *x != 'V').count() == 0
}

fn replace_cv(s: &[char]) -> String {
    let constonants = "bcdfghjklmnpqrstvwxz";
    let vowels      = "aeiouy";
    let mut rng = thread_rng();
    let mut return_string = "".to_string();
    
    for c in s.iter() {
        let next_char = match *c {
            'c' => rng.choose(&(constonants.chars().collect::<Vec<char>>()[..])).unwrap().clone(),
            'C' => rng.choose(&(constonants.chars().collect::<Vec<char>>()[..])).unwrap().clone().to_uppercase().next().unwrap(),
            'v' => rng.choose(&(vowels.chars().collect::<Vec<char>>()[..])).unwrap().clone(),
            'V' => rng.choose(&(vowels.chars().collect::<Vec<char>>()[..])).unwrap().clone().to_uppercase().next().unwrap(),
            _         => ' '
        };
        return_string.push(next_char);
    }
    return_string
}

