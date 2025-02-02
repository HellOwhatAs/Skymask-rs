use num_traits::float::Float;
use num_traits::FloatConst;
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::Debug,
    ops::Range,
};

pub fn float_cmp<T: Float + Ord>(a: T, b: T, eps: T) -> Ordering {
    if (a - b).abs() < eps {
        Ordering::Equal
    } else {
        a.cmp(&b)
    }
}

/// Infinite projection of a line segment in space onto the target coordinate axis.
/// <div class="warning">
/// Ensure that the projections of two line segments on the domain have at most one intersection point.
/// </div>
pub trait ProjLine<T>: Eq + Copy
where
    Self: Sized,
{
    /// Creates an instance from two points.
    fn from_points(p1: &[T; 3], p2: &[T; 3]) -> Option<Self>;
    /// Calculates a value at a given angle `theta`.
    fn at(&self, theta: T) -> T;
    /// Calculates the intersection point with another instance within a given domain.
    fn cross_point(&self, other: &Self, dom: &Range<T>) -> Option<T>;
}

impl<T: Copy + Float + FloatConst + Eq + Debug> ProjLine<T> for (T, T) {
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
    fn cross_point(&self, other: &Self, dom: &Range<T>) -> Option<T> {
        let cross = ((self.0 - other.0) / (other.1 - self.1)).atan();
        if cross > dom.start && cross < dom.end {
            Some(cross)
        } else {
            let cross = cross - T::signum(cross) * T::PI();
            if cross > dom.start && cross < dom.end {
                Some(cross)
            } else {
                None
            }
        }
    }
}

/// A `ProjLine` with domain on `[-pi, pi)`.  
/// The comparison of this struct is based on the maximum value at its endpoints.  
/// Which is the result of `Self::top_endpoint`.  
/// If `dom.0 < dom.1`, the domain is `[dom.0, dom.1)`.  
/// If `dom.0 > dom.1`, the domain is `[dom.0, pi)` and `[-pi, dom.1)`.  
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
    /// Will return None if the two endpoints of `dom` are symmetric relative to the origin.
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
    /// Constructing a projection segment from the two endpoints of a line segment.
    pub fn from_points(p1: &[T; 3], p2: &[T; 3]) -> Option<Self> {
        let line = LT::from_points(p1, p2)?;
        Self::new(line, (p1[1].atan2(p1[0]), p2[1].atan2(p2[0])))
    }
    /// The maximum angle of elevation of the endpoints.
    pub fn top_endpoint(&self) -> T {
        Float::max(self.line.at(self.dom.0), self.line.at(self.dom.1))
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
