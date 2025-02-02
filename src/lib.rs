#![doc = include_str!("../README.md")]
pub mod data;
pub mod utils;
pub use data::read_shp;
use num_traits::{Float, FloatConst};
pub use rangemap::RangeMap;
use std::cmp::{max, min, Ordering::*};
use std::collections::BinaryHeap;
use std::fmt::Debug;
use utils::float_cmp;
pub use utils::{ProjLine, ProjSegment};

/// Computes the skymask from a set of projected segments.  
/// This function takes an iterator of projected segments and an epsilon value for floating-point comparisons.  
/// It returns a `RangeMap` that represents the skymask with dom `[-pi, pi)`.
/// ## Type Parameters
/// - `T`: The numeric type used for coordinates.
/// - `LT`: The line type used for projections.
/// ## Parameters
/// - `lines`: An iterator over `ProjSegment<T, LT>` items representing the projected segments.
/// - `eps`: A tolerance value for floating-point comparisons.
/// ## Returns
/// A `RangeMap<T, LT>` representing the skymask with dom `[-pi, pi)`.
pub fn skymask<T, LT>(lines: impl Iterator<Item = ProjSegment<T, LT>>, eps: T) -> RangeMap<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T> + Debug,
{
    let mut heap = BinaryHeap::from_iter(lines);
    let mut rmap: RangeMap<T, LT> = RangeMap::new();
    let (mut lower, mut fill_all) = (T::infinity(), false);

    while let Some(l) = heap.pop() {
        if fill_all {
            if l.top_endpoint() < lower {
                break;
            }
        } else {
            if rmap.gaps(&(-T::PI()..T::PI())).next().is_none() {
                fill_all = true;
            }
        }
        let doms = [l.dom, (l.dom.0, T::PI()), (-T::PI(), l.dom.1)];
        for &(dom_s, dom_e) in &doms[if l.dom.0 < l.dom.1 { 0..1 } else { 1..3 }] {
            let mut updates = vec![];
            for (seg_dom, seg_func) in rmap.overlapping(dom_s..dom_e) {
                let dom = max(dom_s, seg_dom.start)..min(dom_e, seg_dom.end);
                match (
                    float_cmp(l.line.at(dom.start), seg_func.at(dom.start), eps),
                    float_cmp(l.line.at(dom.end), seg_func.at(dom.end), eps),
                ) {
                    (Less | Equal, Less | Equal) => {}
                    (Greater | Equal, Greater | Equal) => {
                        updates.push((dom, l.line));
                    }
                    (Less, Greater) => {
                        let cross = l.line.cross_point(seg_func, &dom).unwrap();
                        updates.push((cross..dom.end, l.line));
                    }
                    (Greater, Less) => {
                        let cross = l.line.cross_point(seg_func, &dom).unwrap();
                        updates.push((dom.start..cross, l.line));
                    }
                }
            }
            if !fill_all {
                for gap in rmap.gaps(&(dom_s..dom_e)) {
                    updates.push((gap, l.line));
                }
            }
            updates.into_iter().for_each(|(dom, func)| {
                lower = min(lower, min(func.at(dom.start), func.at(dom.end)));
                rmap.insert(dom, func);
            });
        }
    }
    rmap
}
