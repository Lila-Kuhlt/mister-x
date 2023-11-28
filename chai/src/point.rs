#[derive(Clone, Default, Copy, Debug)]
pub struct Point {
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    pub fn distance(self, other: Self) -> f32 {
        f32::hypot(
            other.latitude - self.latitude,
            other.longitude - self.longitude,
        )
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
    let total_length = points
        .windows(2)
        .map(|slice| slice[0].distance(slice[1]))
        .sum::<f32>();
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
