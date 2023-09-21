use std::collections::HashMap;
use std::fs;
use std::path::Path;

use either::Either;
use mint::Point2;

use super::Point;

impl Point {
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
    fn extract_points(image: &serde_json::Value) -> Vec<Self> {
        image["points"]
            .as_array()
            .expect("Failed to parse .cazan/build/assets.json file")
            .iter()
            .map(|json_point| Self {
                point: Point2 {
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