#[derive(Clone, Default, Copy, Debug)]
pub struct Point {
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    /// The approximate distance between two points in meters, using equirectangular projection.
    pub fn distance(self, other: Self) -> f32 {
        // the radius of the earth in meters
        const EARTH_RADIUS: f32 = 6371008.8;

        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();
        let mean_lat = (self.latitude + other.latitude) / 2.0;
        EARTH_RADIUS * f32::hypot(delta_lat, f32::cos(mean_lat) * delta_lon)
    }

    /// Linear interpolation.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Point {
            latitude: self.latitude + t * (other.latitude - self.latitude),
            longitude: self.longitude + t * (other.longitude - self.longitude),
        }
    }
}

pub fn interpolate_segment(points: &[Point], progress: f32) -> Option<Point> {
    let total_length = points.windows(2).map(|slice| slice[0].distance(slice[1])).sum::<f32>();
    let length = progress * total_length;

    let mut current_length = 0.0;
    for slice in points.windows(2) {
        let start = slice[0];
        let end = slice[1];
        let segment_length = start.distance(end);
        if current_length + segment_length > length {
            let progress = (length - current_length) / segment_length;
            return Some(start.lerp(end, progress));
        }
        current_length += segment_length;
    }
    points.last().copied()
}
