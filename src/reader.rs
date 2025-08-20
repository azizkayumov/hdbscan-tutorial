use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn read_data(path: &str) -> Vec<Vec<f64>> {
    let mut data: Vec<Vec<f64>> = Vec::new();
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = &line {
            let mut point = Vec::new();
            for val in line.split(',') {
                let chars = val.chars();
                let val = chars.as_str();
                let c: f64 = val.parse().unwrap_or(f64::INFINITY);
                point.push(c);
            }
            if !point.contains(&f64::INFINITY) {
                data.push(point);
            }
        }
    }
    data
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn read_partial_labels(path: &str) -> HashMap<usize, Vec<usize>> {
    let labels = read_data(path)
        .iter()
        .flatten()
        .copied()
        .collect::<Vec<f64>>();
    let mut input = HashMap::new();
    for (i, label) in labels.iter().enumerate() {
        if *label >= 0.0 {
            input
                .entry(*label as usize)
                .or_insert_with(Vec::new)
                .push(i);
        }
    }
    input
}
