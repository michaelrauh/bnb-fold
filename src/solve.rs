use crate::{
    rule::{full, get_diagonals, get_impacted_phrase_locations, make_blank, next_open_position},
    string_handlers,
};
use rand_distr::{Distribution, Normal};
use rtplot::{Figure, PlotType};
use std::{collections::HashSet, fs::read_to_string, ops::ControlFlow};

pub fn solve_for_dims(dims: Vec<usize>) {
    let max_length = *dims.iter().max().expect("");
    let corpus = read_to_string("example.txt").unwrap();
    let vocab = string_handlers::vocabulary(&corpus);
    let phrases = string_handlers::corpus_to_set(&corpus, max_length);
    let initial = make_blank(&dims);
    let mut stack = vec![initial];
    let impacted_phrase_locations = get_impacted_phrase_locations(&dims);
    let impacted_diagonals = get_diagonals(&dims);

    println!("Vocab size: {}", vocab.len());

    let mut figure = Figure::new(100)
        .xlabel("Time (s)")
        .ylabel("Amplitude")
        .plot_type(PlotType::Line)
        .color(0x80, 0x00, 0x80);
    let mut v = vec![];

    let mut i = 0;
    v.push(0 as f32);
    Figure::display(&mut figure, |fig| {
        let res = fun_name(
            &mut stack,
            &impacted_phrase_locations,
            &impacted_diagonals,
            &vocab,
            max_length,
            &phrases,
        );

        i += 1;

        if i % 10 == 0 {
            v.push(res.depth as f32);
        }
        
        println!("{}", res.depth);

        fig.plot_stream(&v);
    });
}

struct Status {
    done: bool,
    depth: usize,
}

fn fun_name(
    stack: &mut Vec<Vec<Option<String>>>,
    impacted_phrase_locations: &Vec<Vec<Vec<usize>>>,
    impacted_diagonals: &Vec<Vec<usize>>,
    vocab: &Vec<String>,
    max_length: usize,
    phrases: &HashSet<Vec<String>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>,
) -> Status {
    let cur = stack.pop();
    if cur.is_none() {
        println!("no results");
        return Status {
            done: true,
            depth: 0,
        };
    }
    let current_answer = cur.unwrap();
    if full(&current_answer) {
        print!("found result:");
        println!("{:?}", current_answer);
        return Status {
            done: true,
            depth: 0,
        };
    }
    let next_index = next_open_position(&current_answer);
    let impacted_phrases = &*impacted_phrase_locations[next_index];
    let impacted_diagonal = &*impacted_diagonals[next_index];
    let forbidden_words: HashSet<String> = impacted_diagonal
        .iter()
        .map(|idx| current_answer[*idx].as_ref().unwrap())
        .cloned()
        .collect();

    'outer: for new_word in vocab {
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
    Status {
        done: false,
        depth: stack.len(),
    }
}
