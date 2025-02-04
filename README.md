# Skymask-rs
Compute piecewise analytical solutions of skymask for given polyhedra.
> Python binding available at [skymask-py](https://github.com/HellOwhatAs/skymask-py).

## Time Complexity
This crate uses an efficient algorithm to compute the piecewise analytical solution of skymask. Its time complexity is  

$$ O(k \cdot \log r \cdot n \log n) $$  

The obtained analytical solution is a `RangeMap`, therefore the time complexity for sampling skymask is  

$$ O(m \cdot \log r) $$  

> Where $n$ represents the number of line segments, and $k$ denotes the average number of segments each line overlaps with in the analytical result.
> $r$ denotes the number of segments in the analytical result, and $m$ refers to the number of discrete sample points taken from the skymask.  

## Benchmark
Runs on 11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz (8 Physical Cores / 16 Logical Threads) and NVIDIA GeForce RTX 3070 Laptop GPU.
The benchmark code is available at [benchmark.py](https://github.com/HellOwhatAs/Skymask/blob/main/benchmark.py).

|Method|Fps|Time Complexity|
|-|-|-|
|Parallel sampling in [`skymask_py`](https://github.com/HellOwhatAs/skymask-py)|1743.54|$O((k \cdot n \log n + m) \cdot \log r)$|
|Sequential sampling in [`skymask_py`](https://github.com/HellOwhatAs/skymask-py)|187.77|$O((k \cdot n \log n + m) \cdot \log r)$|
|[Naive approach](https://github.com/HellOwhatAs/Skymask/blob/main/skymask.py) with Cupy|84.98|$O(m \cdot n)$|
|[Naive approach](https://github.com/HellOwhatAs/Skymask/blob/main/skymask.py) with Numpy|4.91|$O(m \cdot n)$|

## Install
```toml
[dependencies]
skymask-rs = "0.1.1"
```

## Examples
### Basic Demo
```rust
use kdtree::distance::squared_euclidean;
use ordered_float::OrderedFloat as F;
use skymask_rs::read_shp;
use skymask_rs::ProjSegment;

fn main() {
    println!("{:#?}", skymask_rs::skymask(
        [
            [ 1.0,  1.0,  1.0, -1.0,  1.0, 1.0],
            [-1.0,  1.0,  1.0, -1.0, -1.0, 1.0],
            [-1.0, -1.0,  1.0,  1.0, -1.0, 1.0],
            [ 1.0, -1.0,  1.0,  1.0,  1.0, 1.0],
        ]
        .into_iter()
        .filter_map(|line| {
            ProjSegment::<F<f64>, (F<f64>, F<f64>)>::from_points(
                &[F(line[0]), F(line[1]), F(line[2])],
                &[F(line[3]), F(line[4]), F(line[5])],
            )
        }),
        F(1e-6),
    ));
}
```

### From Shapefile
```rust
use kdtree::distance::squared_euclidean;
use ordered_float::OrderedFloat as F;
use skymask_rs::read_shp;
use skymask_rs::ProjSegment;

fn main() {
    let path = "<path-to-shp-file>";
    let max_dist: f64 = 1000.0;
    let eps: f64 = 1e-6;

    let (arr1, xy, kdtree) = read_shp(path);
    let pos = [
        xy[0].iter().sum::<f64>() / xy[0].len() as f64,
        xy[1].iter().sum::<f64>() / xy[1].len() as f64,
    ];

    let lines_iter = kdtree
        .within(&pos, max_dist.powi(2), &squared_euclidean)
        .unwrap()
        .into_iter()
        .filter_map(|(_, &i)| {
            let row = arr1.row(i);
            ProjSegment::<F<f64>, (F<f64>, F<f64>)>::from_points(
                &[F(row[0] - pos[0]), F(row[1] - pos[1]), F(row[2])],
                &[F(row[3] - pos[0]), F(row[4] - pos[1]), F(row[5])],
            )
        });
    println!("{:?}", skymask_rs::skymask(lines_iter, F(eps)));
}
```