extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::env;

fn train<'a, 'b>(words: &Vec<Vec<u8>>) -> HashMap<Vec<u8>, i64> {
    let mut model = HashMap::<Vec<u8>, i64>::new();
    for word in words.iter() {
        let count = model.entry(word.to_owned()).or_insert(1);
        *count += 1;
    }
    return model;
}

fn edits1(word: &Vec<u8>) -> Vec<Vec<u8>> {
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

fn retain_known(words: &mut Vec<Vec<u8>>, model: &HashMap<Vec<u8>, i64>) {
    words.retain(|word| model.contains_key(&word[..]));
}

fn known_edits2(word: &Vec<u8>, model: &HashMap<Vec<u8>, i64>) -> Vec<Vec<u8>> {
    let mut result = Vec::<Vec<u8>>::new();
    let e1 = edits1(word);
    for edit_vec1 in e1.iter() {
        let e2 = edits1(edit_vec1);
        for edit_vec2 in e2.iter() {
            if model.contains_key(&edit_vec2[..]) {
                result.push(edit_vec2.to_owned());
            }
        }
    }
    remove_duplicates(&mut result);
    return result;
}

fn best_candidate(candidates: &Vec<Vec<u8>>, model: &HashMap<Vec<u8>, i64>) -> Vec<u8> {
    let mut count = -1i64;
    let mut result = Vec::<u8>::new();
    for cand in candidates.iter() {
        let cand_count = model.get(cand).or(Some(&1i64)).unwrap().to_owned();
        if cand_count > count {
            count = cand_count;
            result = cand.to_owned();
        }
    }
    return result;
}

fn correct(word: &str, model: &HashMap<Vec<u8>, i64>) -> Vec<u8> {
    let w: Vec<u8> = word.as_bytes().to_owned();
    if model.contains_key(&w[..]) {
        return w;
    }
    let mut candidates = edits1(&w);
    retain_known(&mut candidates, &model);
    if candidates.len() > 0 {
        return best_candidate(&candidates, &model);
    }
    candidates = known_edits2(&w, &model);
    if candidates.len() > 0 {
        return best_candidate(&candidates, &model);
    }
    return w;
}

fn print_bytevector(bv: &Vec<u8>) {
    for by in bv.iter() { print!("{}", by.to_owned() as char); }
}

fn main() {
    let path = Path::new("../../big.txt");
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // println!("done reading file");

    // as of 22-5-2015, regex implementation takes
    // nearly a minute, needs optimising
    let re = Regex::new(r"([a-z]+)").unwrap();
    let mut words = Vec::<Vec<u8>>::new();
    for capture in re.captures_iter(&contents) {
        let word = capture.at(1).unwrap();
        words.push(word.as_bytes().to_owned());
    }
    // println!("done finding words");

    let model = train(&words);
    // println!("done training words");

    for (itr, arg) in env::args().enumerate() {
        // skip the first argument
        if itr > 0 {
            let correction = correct(&arg, &model);
            print_bytevector(&correction);
            print!(" ");
        }
    }
}