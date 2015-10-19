use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::borrow::Cow;
use std::convert::Into;
use std::cmp::Ordering::{Less, Greater};

fn main() {
    println!("{}", get_longest_word(&['b','n','i','k']));
}

fn get_possible_words<'a>() -> Vec<Cow<'a, str>> {
    let mut v = vec![];
    let f = match File::open(r"/usr/share/dict/words") {
        Ok(f_handle) => f_handle,
        Err(_)       => return vec![]
    };
    let f = BufReader::new(f);
    for line in f.lines() {
        v.push(line.unwrap().into());
    }
    v
}

fn get_longest_word<'a>(letters: &[char]) -> String {
    let possible_words = get_possible_words();
    let mut v: Vec<&Cow<'a, str>> = possible_words.iter()
        // Only if it had to use all the letters ... woops
        // .filter(|&s| {
        //     for l in letters.iter() {
        //         if !s.contains(*l) {
        //             return false;
        //         }
        //     }
        //     true
        // })
        .filter(|&s| {
            for l in s.chars() {
                if !letters.contains(&l) {
                    return false;
                }
            }
            true
        })
        .collect();
    v.sort_by(|a, b| if a.chars().count() > b.chars().count() { Less } else { Greater });
    
    if v.len() > 0 {
        v[0].clone().into_owned()
    } else {
        "".to_string()
    }
}

#[test]
fn test_edcf() { //deedeed
    assert_eq!(get_longest_word(&['e','d','c','f']), "deedeed");
}

#[test]
fn test_bnik() { //bikini
    assert_eq!(get_longest_word(&['b','n','i','k']), "bikini");
}

#[test]
fn test_poil() { //pililloo or lollipop
    assert_eq!(get_longest_word(&['p','o','i','l']), "lollipop");
}

#[test]
fn test_vybu() { //bubby
    assert_eq!(get_longest_word(&['v','y','b','u']), "bubby");
}
