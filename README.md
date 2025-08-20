# Minimal and Simplified Analysis of Hierarchical Density Based Spatial Clustering

Hierarchical density based spatial clustering is a state-of-the-art clustering algorithm that is widely used by the research community for the analysis of spatial data. This popularity is in part due to its accessibility in well-known open-source libraries, which allow researchers to easily install and use the algorithm for their use cases. Although easy to use, the underlying algorithmic steps are quite complex and difficult to understand, which can lead to potential misuse or misinterpretation. Therefore, we aim to provide a step-by-step tutorial of the algorithm’s underlying computations using a simple and minimal two-dimensional example. Specifically, we guide the reader through the example by illustrating the steps of density estimation, minimum spanning tree computation, hierarchy construction, and extraction of flat clustering results. In addition, we provide notes on recent studies for further exploration by readers interested in a deeper analysis of each step of the algorithm. We believe that this tutorial provides the reader with a better understanding of the algorithm, helping them grasp both strengths and limitations of the algorithm through a hands-on approach that can easily be reproduced with just pen and paper.

The repository provides a reproducible code example for the paper accessible here:
https://ieeexplore.ieee.org/document/11126992

### Setup
Install [Rust and its dependencies](https://doc.rust-lang.org/book/ch01-00-getting-started.html):
```
cargo --version
```
Create a virtual environment:
```
python3 -m venv .env
source .env/bin/activate
```
Install dependencies:
```
pip install --upgrade pip
pip install -r requirements.txt
```

### Usage
Run the following to generate clustering results:
```
cargo run --release
```
Plot the clustering results:
```
python plot.py
```

### References
[1] Campello, Ricardo JGB, et al. "Hierarchical density estimates for data clustering, visualization, and outlier detection." ACM Transactions on Knowledge Discovery from Data (TKDD) 10.1 (2015): 1-51.

[2] Castro Gertrudes, Jadson, et al. "A unified view of density-based methods for semi-supervised clustering and classification." Data mining and knowledge discovery 33.6 (2019): 1894-1952.

### License
This project is licensed under the [Apache License, Version 2.0](LICENSE.md) - See the [LICENSE.md](https://github.com/azizkayumov/lctree/blob/main/LICENSE) file for details.
