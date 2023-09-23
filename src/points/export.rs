use std::collections::HashMap;
use std::fs;

use super::Point;

const ASSETS_JSON_PATH: &str = ".cazan/build/assets.json";

impl Point {
    /// Function to export points to a JSON file. It takes a HashMap where the keys are image paths and the values are Vectors of points for the respective image.
    /// It returns a [`Result`] that is either [`Ok`] or [`Err`].
    ///
    /// # Parameters
    ///
    /// * `images_points`: [`HashMap<String, Vec<Point>>`] that holds the image paths and the points associated with the respective image.
    ///
    /// # Returns
    ///
    /// * A [`Result`] that is either [`Ok`] or [`Err`].
    ///
    /// # Errors
    ///
    /// * If the file writing fails.
    /// * If the JSON serialization fails.
    pub fn export_all(images_points: HashMap<String, Vec<Self>>) -> Result<(), String> {
        let mut json = String::new();
        json.push_str("[\n");

        for (image_path, points) in images_points {
            json.push_str(&Self::add_4_spaces_before_each_line(Self::serialize(
                image_path, points,
            )?));
            json.push_str(",\n");
        }
        // Remove the last comma
        json.pop(); // Remove the last '\n'
        json.pop(); // Remove the last ','

        json.push_str("\n]");

        fs::write(".cazan/build/assets.json", json)
            .map_err(|_| "Failed to write to .cazan/build/assets.json file".to_string())?;

        Ok(())
    }

    /// Add an image path and points to the JSON file.
    /// It returns a [`Result`] that is either [`Ok`] or [`Err`].
    ///
    /// # Parameters
    ///
    /// * `image_path`: [`String`] that holds the image path.
    /// * `points`: [`Vec<Point>`] that holds the points associated with the image.
    ///
    /// # Returns
    ///
    /// * A [`Result`] that is either [`Ok`] or [`Err`].
    ///
    /// # Errors
    ///
    /// * If the file reading fails.
    /// * If the JSON parsing fails.
    /// * If the JSON serialization fails.
    /// * If the file writing fails.
    pub fn export(image_path: String, points: Vec<Self>) -> Result<(), String> {
        let mut json = fs::read_to_string(".cazan/build/assets.json").map_err(|e| e.to_string())?;

        // Remove the last ']'
        json.pop(); // Remove the last ']
                    // Remove the last '\n'
        if json.pop().unwrap() == '[' {
            // If the file is empty
            json.push_str("[\n");
        } else {
            json.push_str(",\n");
        }

        json.push_str(&Self::add_4_spaces_before_each_line(Self::serialize(
            image_path, points,
        )?));

        json.push_str("\n]");

        fs::write(".cazan/build/assets.json", json)
            .map_err(|_| "Failed to write to .cazan/build/assets.json file".to_string())?;

        Ok(())
    }
    /// Serializes the given `image_point` to a JSON string.
    ///
    /// # Parameters
    ///
    /// * `image_path`: [`String`] that holds the image path.
    /// * `points`: [`Vec<Point>`] that holds the points associated with the image.
    ///
    /// # Returns
    ///
    /// * A [`Result`] that is either [`Ok(String)`] or [`Err`].
    ///
    /// # Errors
    ///
    /// * If the JSON serialization fails.
    fn serialize(image_path: String, points: Vec<Self>) -> Result<String, String> {
        let mut json = String::new();

        json.push_str(&format!("{{\n    \"path\": \"{}\",\n", image_path));
        json.push_str("    \"points\": [");

        for point in points {
            json.push_str(&format!(
                "{{ \"x\": {}, \"y\": {}, \"n\": {} }}, ",
                point.x(),
                point.y(),
                point.n
            ));
        }
        // Remove the last comma
        json.pop(); // Remove the last ' '
        json.pop(); // Remove the last ','

        json.push_str("]\n}");

        Ok(json)
    }

    fn add_4_spaces_before_each_line(mut json: String) -> String {
        json.insert_str(0, "    ");
        json.replace('\n', "\n    ")
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::collections::HashMap;
    use std::fs;

    use super::Point;

    #[test]
    fn test_serialize() {
        let image_path = String::from("test.png");
        let points = vec![
            Point::new(0, 0, 0),
            Point::new(1, 1, 1),
            Point::new(2, 2, 2),
        ];

        let json = Point::serialize(image_path, points).unwrap();

        assert_eq!(
            json,
            "{\n    \"path\": \"test.png\",\n    \"points\": [{ \"x\": 0, \"y\": 0, \"n\": 0 }, { \"x\": 1, \"y\": 1, \"n\": 1 }, { \"x\": 2, \"y\": 2, \"n\": 2 }]\n}"
        );
    }

    #[test]
    #[serial]
    fn test_export_all() {
        let mut images_points = HashMap::new();
        let image_path = String::from("test.png");
        let points = vec![
            Point::new(0, 0, 0),
            Point::new(1, 1, 1),
            Point::new(2, 2, 2),
        ];
        images_points.insert(image_path, points);

        Point::export_all(images_points).unwrap();

        // Get json from the file
        let json = fs::read_to_string(".cazan/build/assets.json").unwrap();

        fs::write(".cazan/build/assets.json", "[]").unwrap();

        assert_eq!(
            json,
            "[\n    {\n        \"path\": \"test.png\",\n        \"points\": [{ \"x\": 0, \"y\": 0, \"n\": 0 }, { \"x\": 1, \"y\": 1, \"n\": 1 }, { \"x\": 2, \"y\": 2, \"n\": 2 }]\n    }\n]"
        );
    }

    #[test]
    #[serial]
    fn test_export_2_times() {
        let image_path = String::from("test.png");
        let points = vec![
            Point::new(0, 0, 0),
            Point::new(1, 1, 1),
            Point::new(2, 2, 2),
        ];

        Point::export(image_path.clone(), points.clone()).unwrap();
        Point::export(image_path, points).unwrap();

        let json = fs::read_to_string(".cazan/build/assets.json").unwrap();

        fs::write(".cazan/build/assets.json", "[]").unwrap();

        assert_eq!(
            json,
            "[\n    {\n        \"path\": \"test.png\",\n        \"points\": [{ \"x\": 0, \"y\": 0, \"n\": 0 }, { \"x\": 1, \"y\": 1, \"n\": 1 }, { \"x\": 2, \"y\": 2, \"n\": 2 }]\n    },\n    {\n        \"path\": \"test.png\",\n        \"points\": [{ \"x\": 0, \"y\": 0, \"n\": 0 }, { \"x\": 1, \"y\": 1, \"n\": 1 }, { \"x\": 2, \"y\": 2, \"n\": 2 }]\n    }\n]"
        );
    }
}
