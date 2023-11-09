#[derive(Clone, Default, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn distance(self, other: Self) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Linear interpolation.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        Point {
            x: self.x + t * dx,
            y: self.y + t * dy,
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
