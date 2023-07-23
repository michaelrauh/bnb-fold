use std::{hash::Hasher, iter::zip, sync::Mutex};
use tinyset::Set64;

use itertools::Itertools;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::rule::{make_blank, get_impacted_phrase_locations, get_diagonals, next_open_position, full};
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn decode(coded: &Vec<Option<u32>>) -> Vec<String> {
    let mut res = vec![];
    for word in coded {
        if word.is_some() {
            res.push(DECODER[&word.unwrap()].clone().to_string())
        } else {
            break;
        }
    }
    res
}

pub fn solve_for_dims() {

    let mut dims = vec![];
    let vocab = VOCAB.into_iter().collect_vec();

    for x in DIMS.iter() {
        dims = x.split(",").map(|y| {str::parse(y).unwrap()}).collect_vec();
    }
    
    let initial = make_blank(&dims);
    dbg!(&dims);
    dbg!(&initial);
    let mut stack = Mutex::new(vec![initial]);
    let impacted_phrase_locations = get_impacted_phrase_locations(&dims);
    let impacted_diagonals = get_diagonals(&dims);
    let mut i = 0;
    let mut max_index = 0;
    let mut previous_example = vec![];

    loop {
        // if i % 1000 == 0 {
            let first_at_default = stack.get_mut().unwrap()
                .iter()
                .position(|x| next_open_position(x) > 1)
                .unwrap_or_default();
            let touched = stack.get_mut().unwrap().len() - first_at_default;
            let percent = (first_at_default as f32) / (vocab.len() as f32);
            let example = decode(stack.get_mut().unwrap().last().unwrap_or(&vec![]));

            let mut overlap = 0;
            for (cur, prev) in zip(&example, &previous_example) {
                if cur == prev {
                    overlap += 1;
                } else {
                    break;
                }
            }

            println!("iteration: {}", i);
            println!("vocab size: {}", vocab.len());
            println!("best attempt: {}", max_index);
            println!("percent: {:?}", percent);
            println!("untouched tree size: {}", first_at_default);
            println!("touched tree size: {}", touched);
            println!("working on layer: {}", overlap);
            println!("example: {:?}", example);
            println!();

            previous_example = example;
        // }

        i += 1;

        let cur = stack.get_mut().unwrap().pop();
        if cur.is_none() {
            println!("no results");
            break;
        }
        let current_answer = cur.unwrap();

        if full(&current_answer) {
            print!("found result:");
            println!("{:?}", decode(&current_answer));
            break;
        }

        let next_index = next_open_position(&current_answer);
        dbg!(&next_index);

        if next_index > max_index {
            max_index = next_index;
        }
        let impacted_phrases = &impacted_phrase_locations[next_index];
        let impacted_diagonal = &impacted_diagonals[next_index];
        let forbidden_words:Set64<u32> = impacted_diagonal
            .iter()
            .map(|idx| current_answer[*idx].as_ref().unwrap()).copied().collect();

            vocab
            .iter()
            .filter(|v| !forbidden_words.contains(***v))
            .filter(|v| {
                for ip in impacted_phrases {
                    let mut h = ahash::AHasher::default();
                    for word in ip {
                        dbg!(&word);
                        let thing = current_answer[*word].as_ref().unwrap();
                        h.write_u32(*thing);
                    }
                    h.write_u32(***v);
                    let f = h.finish();
                    if !PHRASES.contains(&f) {
                        return false;
                    }
                }
                true
            }).map(|new_word| { 
                dbg!();
                let mut res = current_answer.clone();
                res[next_index] = Some(**new_word);
                res
            }).for_each(|res| {
                stack.lock().unwrap().push(res);
            })
    }
}
