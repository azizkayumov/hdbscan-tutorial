use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_data(path: &str) -> Vec<Vec<f64>> {
    let mut data: Vec<Vec<f64>> = Vec::new();
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if line.is_ok() {
            let mut point = Vec::new();
            let line = line.as_ref().unwrap();
            for (_, val) in line.split(',').enumerate() {
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

pub fn read_partial_labels(path: &str) -> HashMap<usize, Vec<usize>> {
    let labels = read_data(path)
        .iter()
        .flatten()
        .cloned()
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
