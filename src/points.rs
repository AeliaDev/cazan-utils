#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use either::Either;


/// Struct Point for keeping additional information
/// along with a 2D coordinate.
///
/// # Fields
///
/// * `n` - The number of the point.
#[derive(Debug)]
pub struct Point {
    point: _Point,
    pub n: usize,
}

/// Struct [`_Point`] for keeping 2D coordinate information.
///
/// # Fields
///
/// * `x` - The x-coordinate of the point.
/// * `y` - The y-coordinate of the point.
#[derive(Debug)]
struct _Point {
    pub x: u32,
    pub y: u32,
}

impl From<mint::Point2<u32>> for _Point {
    fn from(point: mint::Point2<u32>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<_Point> for mint::Point2<u32> {
    fn from(point: _Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
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

    /// Function to import points from a JSON file. It either returns a Vector of Points or a
    /// HashMap where the keys are image paths and the values are Vectors of points for the respective image.
    ///
    /// # Parameters
    ///
    /// * `image_path` - Could be [`Some`] String that represent the image path or [`None`]. If [`Some`], it returns
    ///                  a Vector of points associated with the provided image_path. If [`None`], it returns a
    ///                  [`HashMap`] where the keys are image paths and the values are Vectors of points for the respective image.
    ///
    /// # Returns
    ///
    /// * A [`Either<Vec<Point>, HashMap<String, Vec<Point>>>`]
    ///   where the left side is a Vector of points and the right side is a HashMap where the keys are image paths
    ///   and the values are Vectors of points for the respective image.
    ///
    /// # Errors
    ///
    /// * If the file reading fails.
    /// * If the JSON parsing fails.
    #[cfg(feature = "points_import")]
    pub fn import(image_path: Option<&str>) -> Either<Vec<Self>, HashMap<String, Vec<Self>>> {
        const ERROR_MESSAGE: &str = "Failed to parse .cazan/build/assets.json file";
        let path = Path::new(".cazan/build/assets.json");

        let json = fs::read_to_string(path).expect(ERROR_MESSAGE);
        let json = serde_json::from_str::<serde_json::Value>(&json).expect(ERROR_MESSAGE);

        match image_path {
            Some(path) => {
                let image = Self::get_image_from_json(&json, path);
                Either::Left(Self::extract_points(&image))
            }
            None => {
                let mut images_points_map = HashMap::new();
                for image in json.as_array().expect(ERROR_MESSAGE) {
                    let image_path = image["path"].as_str().expect(ERROR_MESSAGE).to_string();
                    images_points_map.insert(image_path, Self::extract_points(image));
                }
                Either::Right(images_points_map)
            }
        }
    }

    /// Searches and returns the JSON value of an image that matches the given `image_path`.
    ///
    /// # Parameters
    ///
    /// * `json`: [`serde_json::Value`] object that is to be searched for the image.
    /// * `image_path`: [`String slice`] that holds the reference to the image path that is to be found.
    ///
    /// # Returns
    ///
    /// * A reference to the [`serde_json::Value`] instance that points to the found image.
    ///
    /// # Errors
    ///
    /// * If the JSON parsing fails.
    #[cfg(feature = "points_import")]
    fn get_image_from_json<'a>(
        json: &'a serde_json::Value,
        image_path: &'a str,
    ) -> &'a serde_json::Value {
        json.as_array()
            .expect("Failed to parse .cazan/build/assets.json file")
            .iter()
            .find(|&image| {
                image["path"]
                    .as_str()
                    .expect("Failed to parse .cazan/build/assets.json file")
                    == image_path.replace('\\', "/")
            })
            .expect("Failed to parse .cazan/build/assets.json file")
    }

    /// Extracts points from given [`serde_json::Value`] instance and returns them as a [`Vec<Point>`].
    ///
    /// # Parameters
    ///
    /// * `image`: A reference to the [`serde_json::Value`] instance that needs to be processed for points.
    ///
    /// # Returns
    ///
    /// * A Vector of [`Point`] instances.
    ///
    /// # Errors
    ///
    /// * If the JSON parsing fails.
    #[cfg(feature = "points_import")]
    fn extract_points(image: &serde_json::Value) -> Vec<Self> {
        image["points"]
            .as_array()
            .expect("Failed to parse .cazan/build/assets.json file")
            .iter()
            .map(|json_point| Self {
                point: _Point {
                    x: json_point["x"]
                        .as_u64()
                        .expect("Failed to parse .cazan/build/assets.json file")
                        as u32,
                    y: json_point["y"]
                        .as_u64()
                        .expect("Failed to parse .cazan/build/assets.json file")
                        as u32,
                },
                n: json_point["n"]
                    .as_u64()
                    .expect("Failed to parse .cazan/build/assets.json file")
                    as usize,
            })
            .collect::<Vec<Self>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            // Show cwd
            fs::remove_dir_all(".cazan").ok();
            fs::create_dir(".cazan").expect("Failed to create .cazan directory");
            fs::write(
                ".cazan/config.json",
                r#"{
    "assets-dir": "assets"
}"#,
            )
                .expect("Failed to create .cazan/cazan.json file");
            fs::create_dir(".cazan/build").expect("Failed to create .cazan/build directory");
            fs::write(".cazan/build/assets.json", r#"[
    {
        "path": "assets/test.png",
        "points": [ {"x": 0, "y": 0, "n": 0}, {"x": 1, "y": 1, "n": 1}, {"x": 2, "y": 2, "n": 2}, {"x": 4, "y": 4, "n": 3} ]
    }
]"#).expect("Failed to create .cazan/build/assets.json file");
        })
    }

    #[test]
    fn test_from_json() {
        init();

        let points = Point::import(Some("assets/test.png")).left().unwrap();

        assert_eq!(points.len(), 4);
        assert_eq!(points[0].x(), 0);
        assert_eq!(points[0].y(), 0);
        assert_eq!(points[0].n, 0);
        assert_eq!(points[1].x(), 1);
        assert_eq!(points[1].y(), 1);
        assert_eq!(points[1].n, 1);
        assert_eq!(points[2].x(), 2);
        assert_eq!(points[2].y(), 2);
        assert_eq!(points[2].n, 2);
        assert_eq!(points[3].x(), 4);
        assert_eq!(points[3].y(), 4);
        assert_eq!(points[3].n, 3);
    }

    #[test]
    fn test_from_json_all() {
        init();

        let images_points = Point::import(None).right().unwrap();

        assert_eq!(images_points.len(), 1);
        assert_eq!(images_points["assets/test.png"].len(), 4);
        assert_eq!(images_points["assets/test.png"][0].x(), 0);
        assert_eq!(images_points["assets/test.png"][0].y(), 0);
        assert_eq!(images_points["assets/test.png"][0].n, 0);
        assert_eq!(images_points["assets/test.png"][1].x(), 1);
        assert_eq!(images_points["assets/test.png"][1].y(), 1);
        assert_eq!(images_points["assets/test.png"][1].n, 1);
        assert_eq!(images_points["assets/test.png"][2].x(), 2);
        assert_eq!(images_points["assets/test.png"][2].y(), 2);
        assert_eq!(images_points["assets/test.png"][2].n, 2);
        assert_eq!(images_points["assets/test.png"][3].x(), 4);
        assert_eq!(images_points["assets/test.png"][3].y(), 4);
        assert_eq!(images_points["assets/test.png"][3].n, 3);
    }
}