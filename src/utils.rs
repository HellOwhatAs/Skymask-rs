use num_traits::float::Float;
use num_traits::FloatConst;
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::Debug,
};

pub trait ProjLine<T>
where
    Self: Sized,
{
    fn from_points(p1: &[T; 3], p2: &[T; 3]) -> Option<Self>;
    fn at(&self, theta: T) -> T;
}

impl<T: Copy + Float> ProjLine<T> for (T, T) {
    fn from_points(p1: &[T; 3], p2: &[T; 3]) -> Option<(T, T)> {
        let denominator = p2[0] * p1[1] - p1[0] * p2[1];
        if denominator == T::zero() {
            None
        } else {
            Some((
                (p1[1] * p2[2] - p2[1] * p1[2]) / denominator,
                (p2[0] * p1[2] - p1[0] * p2[2]) / denominator,
            ))
        }
    }
    fn at(&self, theta: T) -> T {
        (self.0 * theta.cos() + self.1 * theta.sin()).atan()
    }
}

#[derive(Debug)]
pub struct ProjSegment<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T>,
{
    pub line: LT,
    pub dom: (T, T),
}

impl<T, LT> ProjSegment<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T>,
{
    pub fn new(line: LT, dom: (T, T)) -> Option<Self> {
        let delta = (dom.0 - dom.1).abs();
        if delta % T::PI() == T::zero() {
            return None;
        }
        let swap = if delta > T::PI() {
            dom.0 < dom.1
        } else {
            dom.0 > dom.1
        };
        Some(Self {
            line,
            dom: if swap { (dom.1, dom.0) } else { dom },
        })
    }
    pub fn from_points(p1: &[T; 3], p2: &[T; 3]) -> Option<Self> {
        let line = LT::from_points(p1, p2)?;
        Self::new(line, (p1[1].atan2(p1[0]), p2[1].atan2(p2[0])))
    }
    pub fn at(&self, theta: T) -> Option<T> {
        if self.in_domain(theta) {
            Some(self.line.at(theta))
        } else {
            None
        }
    }
    pub fn in_domain(&self, theta: T) -> bool {
        theta.abs() <= T::PI()
            && (if self.dom.0 < self.dom.1 {
                theta >= self.dom.0 && theta <= self.dom.1
            } else {
                theta <= self.dom.0 || theta >= self.dom.1
            })
    }
    pub fn top_endpoint(&self) -> T {
        [self.dom.0, self.dom.1, T::PI()]
            .into_iter()
            .filter_map(|theta| self.at(theta))
            .max()
            .unwrap()
    }
}

impl<T, LT> Ord for ProjSegment<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.top_endpoint().cmp(&other.top_endpoint())
    }
}
impl<T, LT> PartialOrd for ProjSegment<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, LT> PartialEq for ProjSegment<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T, LT> Eq for ProjSegment<T, LT>
where
    T: Copy + Float + FloatConst + Ord + Debug,
    LT: ProjLine<T>,
{
}
