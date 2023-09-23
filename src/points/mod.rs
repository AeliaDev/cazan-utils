#![allow(dead_code)]

#[cfg(feature = "points_import")]
mod import;

#[cfg(feature = "points_export")]
mod export;

/// Struct Point for keeping additional information
/// along with a 2D coordinate.
///
/// # Fields
///
/// * `n` - The number of the point.
#[derive(Clone, Debug)]
pub struct Point {
    point: mint::Point2<u32>,
    pub n: usize,
}

impl Point {
    /// Creates a new [`Point`] instance.
    ///
    /// # Parameters
    ///
    /// * `x` - The x-coordinate of the point.
    /// * `y` - The y-coordinate of the point.
    /// * `n` - The number of the point.
    ///
    /// # Returns
    ///
    /// * A new [`Point`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use cazan_utils::points::Point;
    ///
    /// let point = Point::new(0, 0, 0);
    /// ```
    pub fn new(x: u32, y: u32, n: usize) -> Self {
        Self {
            point: mint::Point2 { x, y },
            n,
        }
    }

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

    /// Gets the [`mint::Point2`] instance of this [`Point`].
    ///
    /// # Returns
    ///
    /// * The [`mint::Point2`] instance of this [`Point`].
    pub fn point(&self) -> mint::Point2<u32> {
        self.point
    }
}
