mod utils;
mod data;
use data::read_shp;
use utils::{ProjLine, ProjSegment};
use kdtree::distance::squared_euclidean;
use num_traits::{Float, FloatConst};
use ordered_float::OrderedFloat as F;
use std::collections::BinaryHeap;
use std::fmt::Debug;


fn skymask<T, LT>(lines: impl Iterator<Item = ProjSegment<T, LT>>)
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T> + Debug,
{
    let mut heap = BinaryHeap::from_iter(lines);
    for _ in 0..10 {
        let l = heap.pop().unwrap();
        println!("{:?}", (&l, l.top_endpoint()));
    }
}

fn main() {
    let path = "../Skymask/local/Shanghai/Shanghai_Buildings_DWG-Polygon.shp";
    let max_dist: f64 = 1000.0;

    let (arr1, xy, kdtree) = read_shp(path);
    let pos = [
        xy[0].iter().sum::<f64>() / xy[0].len() as f64,
        xy[1].iter().sum::<f64>() / xy[1].len() as f64,
    ];

    let idx = 0;
    let pos = [pos[0] + 0.5 * idx as f64, pos[1] + 0.5 * idx as f64];
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
    println!("{:?}", skymask(lines_iter));
}
