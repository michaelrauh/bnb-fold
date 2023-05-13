use std::cmp::Ordering;

fn index_array(dims: Vec<usize>) -> Vec<Vec<usize>> {
    cartesian_product(dims.into_iter().map(|x| (0..x).collect()).collect())
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

pub fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: Vec<T>) -> Vec<Vec<T>> {
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

pub fn cartesian_product<T: Clone>(lists: Vec<Vec<T>>) -> Vec<Vec<T>> {
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
    use crate::rule::order_by_distance;

    use super::index_array;

    #[test]
    fn it_produces_an_index_matrix_with_dims() {
        assert_eq!(
            index_array(vec![3, 3]),
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
            index_array(vec![2, 2, 2]),
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
}
