mod divide_skymask;

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use ndarray::{array, Array2, Axis};
use shapefile::{Reader, Shape};
use std::path::Path;

use std::time::Instant;

fn read_shp<P: AsRef<Path>>(path: P) -> (Array2<f64>, [[f64; 2]; 2], KdTree<f64, usize, [f64; 2]>) {
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

fn main() {
    let path = "../Shanghai/Shanghai_Buildings_DWG-Polygon.shp";
    let max_dist: f64 = 1000.0;

    let now = Instant::now();
    let (arr1, xy, kdtree) = read_shp(path);
    println!("{}", now.elapsed().as_millis());

    let pos = [
        xy[0].iter().sum::<f64>() / xy[0].len() as f64,
        xy[1].iter().sum::<f64>() / xy[1].len() as f64,
    ];

    let now = Instant::now();
    (0..1000).for_each(|idx| {
        let idxs = kdtree
            .within(
                &[pos[0] + 0.5 * idx as f64, pos[1] + 0.5 * idx as f64],
                max_dist.powi(2),
                &squared_euclidean,
            )
            .unwrap()
            .into_iter()
            .map(|(_, &i)| i)
            .collect::<Vec<_>>();
        arr1.select(Axis(0), &idxs);
    });
    println!("{}", now.elapsed().as_millis());
}
