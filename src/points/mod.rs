#![allow(dead_code)]

#[cfg(test)]
mod tests;

#[cfg(feature = "points_import")]
mod import;

/// Struct Point for keeping additional information
/// along with a 2D coordinate.
///
/// # Fields
///
/// * `n` - The number of the point.
#[derive(Debug)]
pub struct Point {
    point: mint::Point2<u32>,
    pub n: usize,
}

impl Point {
    /// Returns the x-coordinate of this point.
    ///
    /// # Returns
    ///
    /// * The x-coordinate of this point.
    pub fn x(&self) -> u32 {
        self.point.x
    }

    /// Returns the y-coordinate of this point.
    ///
    /// # Returns
    ///
    /// * The y-coordinate of this point.
    pub fn y(&self) -> u32 {
        self.point.y
    }
}