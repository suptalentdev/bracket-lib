use std::cmp::{max, min};
pub mod lines;
pub mod point;
pub mod point3;
pub use {lines::line2d, point::Point, point3::Point3};

/// Enumeration of available 2D Distance algorithms
pub enum DistanceAlg {
    Pythagoras,
    PythagorasSquared,
    Manhattan,
    Chebyshev,
}

/// Enumeration of available 2D Distance algorithms
pub enum LineAlg {
    Bresenham,
    Vector,
}

#[allow(dead_code)]
/// Provides a 2D distance between points, using the specified algorithm.
pub fn distance2d(algorithm: DistanceAlg, start: Point, end: Point) -> f32 {
    match algorithm {
        DistanceAlg::Pythagoras => distance2d_pythagoras(start, end),
        DistanceAlg::PythagorasSquared => distance2d_pythagoras_squared(start, end),
        DistanceAlg::Manhattan => distance2d_manhattan(start, end),
        DistanceAlg::Chebyshev => distance2d_chebyshev(start, end),
    }
}

#[allow(dead_code)]
/// Provides a 3D distance between points, using the specified algorithm.
pub fn distance3d(algorithm: DistanceAlg, start: Point3, end: Point3) -> f32 {
    match algorithm {
        DistanceAlg::Pythagoras => distance3d_pythagoras(start, end),
        DistanceAlg::PythagorasSquared => distance3d_pythagoras_squared(start, end),
        DistanceAlg::Manhattan => distance3d_manhattan(start, end),
        DistanceAlg::Chebyshev => distance3d_pythagoras(start, end), // Not implemented yet
    }
}

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    (dx * dx) + (dy * dy)
}

#[allow(dead_code)]
/// Calculates a Manhattan distance between two points
fn distance2d_manhattan(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    dx + dy
}

#[allow(dead_code)]
/// Calculates a Manhattan distance between two 3D points
fn distance3d_manhattan(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    dx + dy + dz
}

#[allow(dead_code)]
/// Calculates a Chebyshev distance between two points
/// See: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html
fn distance2d_chebyshev(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    if dx > dy {
        (dx - dy) + 1.0 * dy
    } else {
        (dy - dx) + 1.0 * dx
    }
}

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras_squared(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    (dx * dx) + (dy * dy) + (dz * dz)
}

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras(start: Point, end: Point) -> f32 {
    let dsq = distance2d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras(start: Point3, end: Point3) -> f32 {
    let dsq = distance3d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

#[allow(dead_code)]
/// From a given start point, project forward radius units at an angle of angle_radians degrees.
/// 0 Degrees is north (negative Y), 90 degrees is east (positive X)
pub fn project_angle(start: Point, radius: f32, angle_radians: f32) -> Point {
    let degrees_radians = angle_radians + 3.14159;
    Point::new(
        (0.0 - (start.x as f32 + radius * f32::sin(degrees_radians))) as i32,
        (start.y as f32 + radius * f32::cos(degrees_radians)) as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::{distance2d, distance3d, project_angle, DistanceAlg, Point, Point3};

    #[test]
    fn test_pythagoras_distance() {
        let mut d = distance2d(DistanceAlg::Pythagoras, Point::new(0, 0), Point::new(5, 0));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Pythagoras, Point::new(0, 0), Point::new(-5, 0));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Pythagoras, Point::new(0, 0), Point::new(0, 5));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Pythagoras, Point::new(0, 0), Point::new(0, -5));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Pythagoras, Point::new(0, 0), Point::new(5, 5));
        assert_eq!(d, 7.071068);
    }

    #[test]
    fn test_pythagoras_distance3d() {
        let mut d = distance3d(
            DistanceAlg::Pythagoras,
            Point3::new(0, 0, 0),
            Point3::new(5, 0, 0),
        );
        assert_eq!(d, 5.0);

        d = distance3d(
            DistanceAlg::Pythagoras,
            Point3::new(0, 0, 0),
            Point3::new(-5, 0, 0),
        );
        assert_eq!(d, 5.0);

        d = distance3d(
            DistanceAlg::Pythagoras,
            Point3::new(0, 0, 0),
            Point3::new(5, 5, 5),
        );
        assert_eq!(d, 8.6602545);
    }

    #[test]
    fn test_pythagoras_squared_distance() {
        let mut d = distance2d(
            DistanceAlg::PythagorasSquared,
            Point::new(0, 0),
            Point::new(5, 0),
        );
        assert_eq!(d, 25.0);

        d = distance2d(
            DistanceAlg::PythagorasSquared,
            Point::new(0, 0),
            Point::new(-5, 0),
        );
        assert_eq!(d, 25.0);

        d = distance2d(
            DistanceAlg::PythagorasSquared,
            Point::new(0, 0),
            Point::new(0, 5),
        );
        assert_eq!(d, 25.0);

        d = distance2d(
            DistanceAlg::PythagorasSquared,
            Point::new(0, 0),
            Point::new(0, -5),
        );
        assert_eq!(d, 25.0);

        d = distance2d(
            DistanceAlg::PythagorasSquared,
            Point::new(0, 0),
            Point::new(5, 5),
        );
        assert_eq!(d, 50.0);
    }

    #[test]
    fn test_pythagoras_squared_distance3d() {
        let mut d = distance3d(
            DistanceAlg::PythagorasSquared,
            Point3::new(0, 0, 0),
            Point3::new(5, 0, 0),
        );
        assert_eq!(d, 25.0);

        d = distance3d(
            DistanceAlg::PythagorasSquared,
            Point3::new(0, 0, 0),
            Point3::new(-5, 0, 0),
        );
        assert_eq!(d, 25.0);

        d = distance3d(
            DistanceAlg::PythagorasSquared,
            Point3::new(0, 0, 0),
            Point3::new(5, 5, 5),
        );
        assert_eq!(d, 75.0);
    }

    #[test]
    fn test_manhattan_distance() {
        let mut d = distance2d(DistanceAlg::Manhattan, Point::new(0, 0), Point::new(5, 0));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Manhattan, Point::new(0, 0), Point::new(-5, 0));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Manhattan, Point::new(0, 0), Point::new(0, 5));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Manhattan, Point::new(0, 0), Point::new(0, -5));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Manhattan, Point::new(0, 0), Point::new(5, 5));
        assert_eq!(d, 10.0);
    }

    #[test]
    fn test_manhattan_distance3d() {
        let mut d = distance3d(
            DistanceAlg::Manhattan,
            Point3::new(0, 0, 0),
            Point3::new(5, 0, 0),
        );
        assert_eq!(d, 5.0);

        d = distance3d(
            DistanceAlg::Manhattan,
            Point3::new(0, 0, 0),
            Point3::new(-5, 0, 0),
        );
        assert_eq!(d, 5.0);

        d = distance3d(
            DistanceAlg::Manhattan,
            Point3::new(0, 0, 0),
            Point3::new(5, 5, 5),
        );
        assert_eq!(d, 15.0);
    }

    #[test]
    fn test_chebyshev_distance() {
        let mut d = distance2d(DistanceAlg::Chebyshev, Point::new(0, 0), Point::new(5, 0));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Chebyshev, Point::new(0, 0), Point::new(-5, 0));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Chebyshev, Point::new(0, 0), Point::new(0, 5));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Chebyshev, Point::new(0, 0), Point::new(0, -5));
        assert_eq!(d, 5.0);

        d = distance2d(DistanceAlg::Chebyshev, Point::new(0, 0), Point::new(5, 5));
        assert_eq!(d, 5.0);
    }

    #[test]
    fn test_project_angle() {
        let start = Point::new(0, 0);
        let mut dest = project_angle(start, 10.0, 0.0);
        assert_eq!(dest, Point::new(0, -10));

        dest = project_angle(start, 10.0, std::f32::consts::PI); // 180 degrees
        assert_eq!(dest, Point::new(0, 10));

        dest = project_angle(start, 10.0, std::f32::consts::PI / 2.0); // 90 degrees, east
        assert_eq!(dest, Point::new(10, 0));

        dest = project_angle(
            start,
            10.0,
            std::f32::consts::PI + (std::f32::consts::PI / 2.0),
        ); // 270 degrees, west
        assert_eq!(dest, Point::new(-10, 0));

        dest = project_angle(start, 10.0, 0.785398); // 45 degrees, north-east
        assert_eq!(dest, Point::new(7, -7));

        dest = project_angle(start, 10.0, 2.35619); // 135 degrees, south-east
        assert_eq!(dest, Point::new(7, 7));

        dest = project_angle(start, 10.0, 3.92699); // 225 degrees, south-west
        assert_eq!(dest, Point::new(-7, 7));

        dest = project_angle(start, 10.0, 5.49779); // 315 degrees, north-west
        assert_eq!(dest, Point::new(-7, -7));
    }
}
