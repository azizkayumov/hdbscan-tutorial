# hdbscan-tutorial
Minimal and simplified tutorial on hierarchical spatial clustering

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

## License
This project is licensed under the [Apache License, Version 2.0](LICENSE.md) - See the [LICENSE.md](https://github.com/azizkayumov/lctree/blob/main/LICENSE) file for details.
