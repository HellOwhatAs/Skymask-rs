mod data;
mod utils;
use data::read_shp;
use kdtree::distance::squared_euclidean;
use num_traits::{Float, FloatConst};
use ordered_float::OrderedFloat as F;
use rangemap::RangeMap;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use utils::{ProjLine, ProjSegment};

#[inline]
fn float_cmp<T: Float + Ord>(a: T, b: T, eps: T) -> std::cmp::Ordering {
    if (a - b).abs() < eps {
        std::cmp::Ordering::Equal
    } else {
        a.cmp(&b)
    }
}

fn skymask<T, LT>(lines: impl Iterator<Item = ProjSegment<T, LT>>, eps: T) -> RangeMap<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T> + Debug,
{
    let mut heap = BinaryHeap::from_iter(lines);
    let mut rmap: RangeMap<T, LT> = RangeMap::new();
    while let Some(l) = heap.pop() {
        let doms = [l.dom, (l.dom.0, T::PI()), (-T::PI(), l.dom.1)];
        for &(dom_s, dom_e) in &doms[if l.dom.0 < l.dom.1 { 0..1 } else { 1..3 }] {
            let mut updates = vec![];
            for (seg_dom, seg_func) in rmap.overlapping(dom_s..dom_e) {
                use std::cmp::{max, min, Ordering::*};
                let dom = (max(dom_s, seg_dom.start), min(dom_e, seg_dom.end));
                match (
                    float_cmp(l.line.at(dom.0), seg_func.at(dom.0), eps),
                    float_cmp(l.line.at(dom.1), seg_func.at(dom.1), eps),
                ) {
                    (Less | Equal, Less | Equal) => {}
                    (Greater | Equal, Greater | Equal) => {
                        updates.push((dom, l.line));
                    }
                    (Less, Greater) => {
                        let cross = l.line.cross_point(seg_func, dom).unwrap();
                        updates.push(((cross, dom.1), l.line));
                    }
                    (Greater, Less) => {
                        let cross = l.line.cross_point(seg_func, dom).unwrap();
                        updates.push(((dom.0, cross), l.line));
                    }
                }
            }
            if updates.is_empty() {
                updates.push(((dom_s, dom_e), l.line));
            }
            updates.into_iter().for_each(|((dom_s, dom_e), func)| {
                let range = dom_s..dom_e;
                rmap.insert(range, func);
            });
        }
    }
    rmap
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
    println!("{:?}", skymask(lines_iter, F(1e-6)));
}
