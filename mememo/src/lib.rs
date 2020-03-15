use std::collections::HashMap;

pub struct Stats {
    pub mean: Option<f32>,
    pub median: Option<f32>,
    pub mode: Option<i32>
}

pub fn mememo(ints: &[i32]) -> Stats {
    Stats {
        mean: calculate_mean(ints),
        median: calculate_median(ints),
        mode: calculate_mode(ints)
    }
}

fn calculate_mean(ints: &[i32]) -> Option<f32> {
    match ints.get(0) {
        Some(_) => {
            let mean = ints.iter().sum::<i32>() as f32 / ints.len() as f32;
            Some(mean)
        },
        None => None
    }
}

// we'll assume for now that +ints+ is sorted
fn calculate_median(ints: &[i32]) -> Option<f32> {
    match ints.get(0) {
        Some(_) => {
            let size = ints.len();
            if size % 2 == 0 {
                let end_idx = size / 2;
                let start_idx = end_idx - 1;
                calculate_mean(&ints[start_idx..end_idx])
            } else {
                let idx = (size - 1) / 2;
                Some(ints[idx] as f32)
            }
        },
        None => None
    }
}

fn calculate_mode(ints: &[i32]) -> Option<i32> {
    match ints.get(0) {
        Some(_) => {
            let mut map = HashMap::new();
            for i in ints.iter() {
                *map.entry(i).or_insert(0) += 1;
            }

            map.into_iter()
                .max_by_key( |&(_, count)| count)
                .map(|(val, _)| *val)
        },
        None => None
    }
}
