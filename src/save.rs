use std::{collections::HashMap, io::Write};

#[allow(dead_code)]
pub fn save_clusters(clusters: HashMap<usize, Vec<usize>>, noise_points: Vec<usize>, path: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    // Save clusters
    let mut rows = Vec::new();
    for (cluster_id, point_ids) in clusters {
        for point_id in point_ids {
            rows.push((point_id, cluster_id));
        }
    }
    for point_id in noise_points {
        rows.push((point_id, usize::MAX));
    }
    rows.sort_by_key(|k| k.0);
    let rows = rows
        .iter()
        .map(|(_, cluster_id)| {
            if cluster_id == &usize::MAX {
                format!("-1")
            } else {
                format!("{cluster_id}")
            }
        })
        .collect::<Vec<_>>();
    save_rows(&path, &rows);
}

pub fn save_outlier_scores(outlier_scores: Vec<f64>, path: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let rows = outlier_scores
        .iter()
        .map(|score| format!("{:.6}", score))
        .collect::<Vec<_>>();
    save_rows(&path, &rows);
}

fn save_rows(path: &str, rows: &Vec<String>) {
    let now = std::time::Instant::now();
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut file = std::fs::File::create(path).unwrap();
    for row in rows {
        file.write_all(row.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    println!("Saved to: {path} ({:?})", now.elapsed());
    file.flush().unwrap();
}
