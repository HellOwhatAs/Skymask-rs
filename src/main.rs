use kdtree::distance::squared_euclidean;
use ordered_float::OrderedFloat as F;
use skymask_rs::data::read_shp;
use skymask_rs::utils::ProjSegment;

fn main() {
    let path = "../Skymask/local/Shanghai/Shanghai_Buildings_DWG-Polygon.shp";
    let max_dist: f64 = 1000.0;
    let eps: f64 = 1e-6;

    let (arr1, xy, kdtree) = read_shp(path);
    let pos = [
        xy[0].iter().sum::<f64>() / xy[0].len() as f64,
        xy[1].iter().sum::<f64>() / xy[1].len() as f64,
    ];

    let idx = -8;
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
    println!(
        "{:?}",
        skymask_rs::skymask(lines_iter, F(eps))
            .into_iter()
            .map(|(r, f)| ((r.start, r.end), f))
            .collect::<Vec<_>>()
    );
}
