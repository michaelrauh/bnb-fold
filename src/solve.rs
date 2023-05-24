use std::{collections::HashSet, fs::read_to_string, hash::Hasher, iter::zip};


use itertools::Itertools;
use rustc_hash::FxHasher;

use crate::{
    rule::{full, get_diagonals, get_impacted_phrase_locations, make_blank, next_open_position},
    string_handlers::{self, Codec},
};

fn decode(coded: &Vec<Option<u32>>, codec: &Codec) -> Vec<String> {
    let mut res = vec![];
    for word in coded {
        if word.is_some() {
            res.push(codec.decoder[&word.unwrap()].clone())
        } else {
            break;
        }
    }
    res
}

pub fn solve_for_dims(dims: Vec<usize>) {
    let max_length = *dims.iter().max().unwrap();
    let corpus = read_to_string("example.txt").unwrap();
    let codec = string_handlers::make_codec(&corpus);
    let vocab: Vec<&u32> = codec.coder.values().sorted().collect();
    let phrases = string_handlers::corpus_to_set(&corpus, max_length, &codec);
    let initial = make_blank(&dims);
    let mut stack = vec![initial];
    let impacted_phrase_locations = get_impacted_phrase_locations(&dims);
    let impacted_diagonals = get_diagonals(&dims);
    let mut i = 0;
    let mut max_index = 0;
    let mut previous_example = vec![];

    loop {
        if i % 1000 == 0 {
            let first_at_default = stack.iter().position(|x| next_open_position(x) > 1).unwrap_or_default();
            let touched = stack.len() - first_at_default;
            let percent = (first_at_default as f32) / (vocab.len() as f32);
            let example = decode(stack.last().unwrap(), &codec);


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
            println!("");

            previous_example = example;
        }

        i += 1;

        let cur = stack.pop();
        if cur.is_none() {
            println!("no results");
            break;
        }
        let current_answer = cur.unwrap();

        if full(&current_answer) {
            print!("found result:");
            println!("{:?}", decode(&current_answer, &codec));
            break;
        }

        let next_index = next_open_position(&current_answer);

        if next_index > max_index {
            max_index = next_index;
        }
        let impacted_phrases = &impacted_phrase_locations[next_index];
        let impacted_diagonal = &impacted_diagonals[next_index];
        let forbidden_words: HashSet<u32> = impacted_diagonal
            .iter()
            .map(|idx| current_answer[*idx].as_ref().unwrap())
            .cloned()
            .collect();

        'outer: for new_word in &vocab {
            for forbidden_word in &forbidden_words {
                if new_word == &forbidden_word {
                    continue 'outer;
                }
            }

            for phrase in impacted_phrases {
                let mut h = FxHasher::default();
                
                for word in phrase {
                    let thing = current_answer[*word].as_ref().unwrap();
                    h.write_u32(*thing);
                }

                h.write_u32(**new_word);
                let f = h.finish();
                if !phrases.contains(&f) {
                    continue 'outer;
                }
            }

            let mut res = current_answer.clone();
            res[next_index] = Some(**new_word);
            stack.push(res);
        }
    }
}
