use std::{collections::HashSet, fs::read_to_string};

use crate::{
    rule::{full, get_diagonals, get_impacted_phrase_locations, make_blank, next_open_position},
    string_handlers,
};

pub fn solve_for_dims(dims: Vec<usize>) {
    let max_length = *dims.iter().max().unwrap();
    let corpus = read_to_string("example.txt").unwrap();
    let vocab = string_handlers::vocabulary(&corpus);
    let phrases = string_handlers::corpus_to_set(&corpus, max_length);
    let initial = make_blank(&dims);
    let mut stack = vec![initial];
    let impacted_phrase_locations = get_impacted_phrase_locations(&dims);
    let impacted_diagonals = get_diagonals(&dims);
    let mut i = 0;
    let mut max_index = 0;

    loop {
        if i % 1000 == 0 {
            let next_index = next_open_position(&stack.last().unwrap());
            let first_at_default = stack.iter().position(|x| next_open_position(x) > 1).unwrap_or_default();
            let touched = stack.len() - first_at_default;
            let index = stack.len() - (touched+1);
            let last_n = stack.get(index..).unwrap_or_default();
            let total_index: usize = last_n.iter().map(|x| next_open_position(x)).sum();
            let average_index: f32 = (total_index as f32) / (last_n.len() as f32);
            let percent = (first_at_default as f32) / (vocab.len() as f32);


            println!("vocab size: {}", vocab.len());
            println!("iteration: {}", i);
            println!("best attempt: {}", max_index);
            println!("next position: {}", next_index);
            println!("stack depth: {}", stack.len());
            println!("average progress of those touched: {}", average_index);
            println!("untouched tree size: {}", first_at_default);
            println!("touched tree size: {}", touched);
            println!("example: {:?}", stack.last().unwrap());
            println!("percent: {:?}", percent);
            
            println!("");
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
            println!("{:?}", current_answer);
            break;
        }

        let next_index = next_open_position(&current_answer);

        if next_index > max_index {
            max_index = next_index;
        }
        let impacted_phrases = &impacted_phrase_locations[next_index];
        let impacted_diagonal = &impacted_diagonals[next_index];
        let forbidden_words: HashSet<String> = impacted_diagonal
            .iter()
            .map(|idx| current_answer[*idx].as_ref().unwrap())
            .cloned()
            .collect();

        'outer: for new_word in &vocab {
            for forbidden_word in &forbidden_words {
                if new_word == forbidden_word {
                    continue 'outer;
                }
            }

            for phrase in impacted_phrases {
                let mut current_real_phrase = Vec::with_capacity(max_length + 1);
                for word in phrase {
                    let thing = current_answer[*word].as_ref().unwrap();
                    current_real_phrase.push(thing.to_string());
                }
                current_real_phrase.push(new_word.to_string());
                if !phrases.contains(&current_real_phrase) {
                    continue 'outer;
                }
            }

            let mut res = current_answer.clone();
            res[next_index] = Some(new_word.to_string());
            stack.push(res);
        }
    }
}
