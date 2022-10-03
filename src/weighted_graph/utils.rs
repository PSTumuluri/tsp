/// Computes the distance between two points in Euclidian 2D space.
pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_is_computed_correctly() {
        let p1 = (0.0, 0.0);
        let p2 = (3.0, 4.0);

        assert_eq!(5.0, distance(p1, p2));
    }
}
