pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn convert_to_point(row: &[f64; 3]) -> Point{
    Point {x: row[0], y: row[1], z: row[2]}
}

pub fn convert_to_raw_data(point: &Point) -> [f64; 3]{
    [
        point.x,
        point.y,
        point.z
    ]
}

pub fn convert_raw_data_set_to_point_dataset(p : &[f64], nb: u64) -> Vec<Point> {
    let mut points : Vec<Point> = Vec::new();
    let mut i = 0;

    while i < nb {
        let tmp : [f64;3] = [p[i as usize], p[(i+1) as usize], p[(i+2) as usize]];
        let point = convert_to_point(&tmp);
        points.push(point);
        i += 3;
    }
    points
}

pub fn calculate_weights(_points : &[Point]) -> [f64;10] {
    let res : [f64;10] = [0.0 as f64;10];
    return res;
}
