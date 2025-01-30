use kdtree::KdTree;
use ndarray::{array, Array2, Axis};
use shapefile::{Reader, Shape};
use std::path::Path;

pub fn read_shp<P: AsRef<Path>>(path: P) -> (Array2<f64>, [[f64; 2]; 2], KdTree<f64, usize, [f64; 2]>) {
    let mut reader = Reader::from_path(path).unwrap();
    let mut lines = Array2::<f64>::zeros((0, 6));
    let mut xy = [
        [f64::NEG_INFINITY, f64::INFINITY],
        [f64::NEG_INFINITY, f64::INFINITY],
    ];
    let mut kdtree = KdTree::new(2);

    for row in reader.iter_shapes_and_records() {
        let polyz = match row {
            Ok((Shape::PolygonZ(polyz), _)) => polyz,
            Ok((shape, _)) => panic!("{shape} is not PolygonZ"),
            Err(e) => panic!("{}", e),
        };
        let points = match polyz.rings() {
            [polyzring] => polyzring.points(),
            other => panic!("{other:?} exists more than one ring"),
        };

        points.iter().for_each(|p| {
            xy[0] = [xy[0][0].max(p.x), xy[0][1].min(p.x)];
            xy[1] = [xy[1][0].max(p.y), xy[1][1].min(p.y)];
        });

        for p in points.windows(2) {
            kdtree
                .add(
                    [(p[0].x + p[1].x) / 2.0, (p[0].y + p[1].y) / 2.0],
                    lines.shape()[0],
                )
                .unwrap();

            lines
                .push(
                    Axis(0),
                    array![p[0].x, p[0].y, p[0].z, p[1].x, p[1].y, p[1].z].view(),
                )
                .unwrap();
        }
    }

    (lines, xy, kdtree)
}