use crate::{
    reader::{read_data, read_partial_labels},
    save::{save_clusters, save_outlier_scores},
};
use ndarray::Array2;
use petal_clustering::{Fit, HDbscan};
mod reader;
mod save;

fn main() {
    let data = read_data("data/data.csv");
    let (n, d) = (data.len(), data[0].len());
    println!("# of points:   {n}");
    println!("# of features: {d}");

    let mut model = HDbscan {
        min_samples: 4,
        min_cluster_size: 4,
        ..Default::default()
    };

    // Unsupervised clustering
    let flattened = data.iter().flatten().copied().collect::<Vec<f64>>();
    let input = Array2::from_shape_vec((n, d), flattened).expect("data shape error");
    let (clusters, noise_points, _) = model.fit(&input, None);
    save_clusters(clusters, noise_points, "output/clusters.csv");

    // Semi-supervised clustering
    let partial_labels = read_partial_labels("data/partial_labels.csv");
    let (clusters, noise_points, outlier_scores) = model.fit(&input, Some(&partial_labels));
    save_clusters(clusters, noise_points, "output/semi_clusters.csv");

    // Outlier scores
    save_outlier_scores(&outlier_scores, "output/outlier_scores.csv");
}
