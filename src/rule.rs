use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

pub fn make_blank(dims: &Vec<usize>) -> Vec<Option<String>> {
    let size = dims.iter().fold(1, |acc, cur| acc * cur);
    let mut vec = Vec::with_capacity(size);
    vec.resize(size, None);
    vec
}

pub fn full(answer: &Vec<Option<String>>) -> bool {
    answer.iter().all(|x| x.is_some())
}

fn indices_in_order(dims: &Vec<usize>) -> Vec<Vec<usize>> {
    order_by_distance(index_array(dims))
}

fn index_to_location_mapping(dims: &Vec<usize>) -> HashMap<usize, Vec<usize>> {
    indices_in_order(&dims).into_iter().enumerate().collect()
}

fn location_to_index_mapping(dims: &Vec<usize>) -> HashMap<Vec<usize>, usize> {
    indices_in_order(&dims)
        .into_iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect()
}

fn impacted_locations(
    location: usize,
    location_to_index: &HashMap<usize, Vec<usize>>,
    index_to_location: &HashMap<Vec<usize>, usize>,
) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let index = &location_to_index[&location];

    let indices = 0..index.len();
    for focus in indices {
        let cur = index[focus];
        let mut subres = vec![];
        for i in 0..cur {
            let mut location = index.clone();
            location[focus] = i;

            let fin = index_to_location[&location];
            subres.push(fin);
        }
        res.push(subres)
    }

    res
}

pub fn get_impacted_phrase_locations(dims: &Vec<usize>) -> Vec<Vec<Vec<usize>>> {
    let location_to_index = location_to_index_mapping(&dims);
    let index_to_location = index_to_location_mapping(&dims);
    let blank = make_blank(&dims);

    (0..blank.len())
        .map(|location| impacted_locations(location, &index_to_location, &location_to_index))
        .collect()
}

pub fn next_open_position(answer: &Vec<Option<String>>) -> usize {
    answer.iter().position(|x| x.is_none()).unwrap()
}

pub fn get_diagonals(dims: &Vec<usize>) -> Vec<Vec<usize>> {
    let location_to_index = location_to_index_mapping(&dims);
    let index_to_location = index_to_location_mapping(&dims);
    let blank = make_blank(&dims);
    let indices = indices_in_order(&dims);

    (0..blank.len())
        .map(|location| {
            let current_index = &index_to_location[&location];
            let current_distance: usize = current_index.iter().sum();
            indices
                .iter()
                .filter(|index| {
                    index.to_owned() < current_index
                        && index.iter().sum::<usize>() == current_distance
                })
                .map(|x| location_to_index[x])
                .collect_vec()
        })
        .collect_vec()
}

fn index_array(dims: &Vec<usize>) -> Vec<Vec<usize>> {
    cartesian_product(dims.into_iter().map(|x| (0..*x).collect()).collect())
}

fn order_by_distance(indices: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut sorted = indices;
    sorted.sort_by(|a, b| {
        match a.iter().sum::<usize>().cmp(&b.iter().sum()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // distance is equal, and so order by each
                for (x, y) in a.iter().zip(b) {
                    if x > y {
                        return Ordering::Greater;
                    }

                    if x < y {
                        return Ordering::Less;
                    }
                }
                unreachable!("There should not be duplicate indices")
            }
            Ordering::Greater => Ordering::Greater,
        }
    });
    sorted
}

fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: Vec<T>) -> Vec<Vec<T>> {
    a.into_iter()
        .flat_map(|xs| {
            b.iter()
                .cloned()
                .map(|y| {
                    let mut vec = xs.clone();
                    vec.push(y);
                    vec
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn cartesian_product<T: Clone>(lists: Vec<Vec<T>>) -> Vec<Vec<T>> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();

            rest.iter()
                .cloned()
                .fold(init, |vec, list| partial_cartesian(vec, list))
        }
        None => {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::{get_diagonals, get_impacted_phrase_locations, order_by_distance};

    use super::index_array;

    #[test]
    fn it_produces_an_index_matrix_with_dims() {
        assert_eq!(
            index_array(&vec![3, 3]),
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2]
            ]
        );
        assert_eq!(
            index_array(&vec![2, 2, 2]),
            vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 1]
            ]
        );
    }

    #[test]
    fn it_orders_indices() {
        assert_eq!(
            order_by_distance(vec![
                vec![0, 0],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2]
            ]),
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![1, 1],
                vec![2, 0],
                vec![1, 2],
                vec![2, 1],
                vec![2, 2]
            ]
        );
    }

    #[test]
    fn it_gets_impacted_phrase_locations() {
        assert_eq!(
            get_impacted_phrase_locations(&vec![2, 2]),
            vec![
                vec![vec![], vec![]],
                vec![vec![], vec![0]],
                vec![vec![0], vec![]],
                vec![vec![1], vec![2]]
            ]
        )
    }

    #[test]
    fn it_gets_impacted_diagonals() {
        assert_eq!(
            get_diagonals(&vec![3, 3]),
            vec![
                vec![],
                vec![],
                vec![1],
                vec![],
                vec![3],
                vec![3, 4],
                vec![],
                vec![6],
                vec![]
            ]
        )
    }
}
