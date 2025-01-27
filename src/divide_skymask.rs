#[derive(Debug)]
pub struct ProjLine {
    pub para: (f64, f64),
    pub dom: (f64, f64),
}

impl ProjLine {
    pub fn new(para: (f64, f64), dom: (f64, f64)) -> Self {
        ProjLine { para, dom }
    }

    pub fn from_points(mut p1: (f64, f64, f64), mut p2: (f64, f64, f64)) -> Option<Self> {
        if p2.0 * p1.1 == p1.0 * p2.1 {
            return None;
        }
        let (mut t1, mut t2) = (p1.1.atan2(p1.0), p2.1.atan2(p2.0));
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            std::mem::swap(&mut p1, &mut p2);
        }
        let a = (p1.1 * p2.2 - p2.1 * p1.2) / (p2.0 * p1.1 - p1.0 * p2.1);
        let b = (p2.0 * p1.2 - p1.0 * p2.2) / (p2.0 * p1.1 - p1.0 * p2.1);
        Some(ProjLine {
            para: (a, b),
            dom: (t1, t2),
        })
    }

    pub fn at(&self, theta: f64) -> Option<f64> {
        if theta >= self.dom.0 && theta <= self.dom.1 {
            Some(self.para.0 * theta.cos() + self.para.1 * theta.sin())
        } else {
            None
        }
    }
}