extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn train<'a, 'b>(words: &'a Vec<&'b [u8]>) -> HashMap<&'b [u8], i64> {
    let mut model = HashMap::new();
    for word in words.iter() {
        let count = model.entry(*word).or_insert(1);
        *count += 1;
    }
    return model;
}

fn edits1(word: Vec<u8>) -> Vec<Vec<u8>> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut result = Vec::<Vec<u8>>::new();

    deletes(&word, &mut result);
    replaces(&word, &mut result, alphabet);
    inserts(&word, &mut result, alphabet);
    transposes(&word, &mut result);

    remove_duplicates(&mut result);

    return result;
}

fn replaces(word: &Vec<u8>, result: &mut Vec<Vec<u8>>, alphabet: &str) {
    for i in 0 .. word.len() {
        for alph in alphabet.chars() {
            let mut rep = Vec::<u8>::new();
            for (itr, c) in word.iter().enumerate() {
                rep.push(if itr != i { c.to_owned() } else { alph as u8 });
            }
            result.push(rep);
        }
    }
}

fn deletes(word: &Vec<u8>, result: &mut Vec<Vec<u8>>) {
    for i in 0 .. word.len() {
        let mut v = Vec::<u8>::new();
        for (itr, c) in word.iter().enumerate() {
            if itr != i {
                let ch = c.to_owned();
                v.push(ch);
            }
        }
        result.push(v);
    }
}

fn inserts(word: &Vec<u8>, result: &mut Vec<Vec<u8>>, alphabet: &str) {
    // for insertions before the last char of 'word'
    for i in 0 .. word.len() {
        for alph in alphabet.chars() {
            let mut ins = Vec::<u8>::new();
            for (itr, c) in word.iter().enumerate() {
                if itr == i {
                    ins.push(alph as u8);
                }
                ins.push(c.to_owned());
            }
            result.push(ins);
        }
    }
    // for insertions at the end of 'word'
    for alph in alphabet.chars() {
        let mut ins = Vec::<u8>::new();
        for c in word.iter() {
            ins.push(c.to_owned());
        }
        ins.push(alph as u8);
        result.push(ins);
    }
}

fn transposes(word: &Vec<u8>, result: &mut Vec<Vec<u8>>) {
    for i in 0 .. (word.len() - 1) {
        let mut trsp = word.to_owned();
        let tmp = trsp[i];
        trsp[i] = trsp[i+1];
        trsp[i+1] = tmp;
        result.push(trsp);
    }
}

fn remove_duplicates(words: &mut Vec<Vec<u8>>) {
    words.sort();
    words.dedup();
}

fn known_edits2(word: Vec<u8>, model: &HashMap<&[u8], i64>) -> Vec<Vec<u8>> {
    let mut result = Vec::<Vec<u8>>::new();
    let e1 = edits1(word);
    for edit_vec1 in e1.iter() {
        let e2 = edits1(edit_vec1.to_owned());
        for edit_vec2 in e2.iter() {
            if model.contains_key(edit_vec2) {
                result.push(edit_vec2.to_owned());
            }
        }
    }
    return result;
}

// fn correct(word: &str) -> String {
//     "temp".to_string()
// }

fn print_bytevector(bv: &Vec<u8>) {
    for by in bv.iter() { print!("{}", by.to_owned() as char); }
    println!("");
}

fn to_bytevector(word: &str) -> Vec<u8> {
    let mut result = vec![];
    for c in word.chars() { result.push(c as u8); }
    return result;
}

fn main() {
    let path = Path::new("../../big.txt");
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // println!("done reading file");

    // let re = Regex::new(r"([a-z]+)").unwrap();
    // let mut words = Vec::new();
    // for capture in re.captures_iter(&contents) {
    //     let word = capture.at(1).unwrap();
    //     words.push(word);
    // }
    // println!("done finding words");

    // let model = train(&words);
    // println!("done training words");

    let edits = edits1(to_bytevector("aa"));
    for s in edits.iter() {
        print_bytevector(s);
    }
}