use either::Either;
use mint::Point2;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct Point {
    point: Point2<u32>,
    pub n: usize,
}

impl Point {
    pub fn x(&self) -> u32 {
        self.point.x
    }

    pub fn y(&self) -> u32 {
        self.point.y
    }

    // This function could return Vec<Self> or HashMap<String, Vec<Self>> (which is a map of image paths to theirs points. It have to return all from the filee)
    #[cfg(feature = "points_import")]
    pub fn import(image_path: Option<&str>) -> Either<Vec<Self>, HashMap<String, Vec<Self>>> {
        const ERROR_MESSAGE: &str = "Failed to parse .cazan/build/assets.json file";
        let path = Path::new(".cazan/build/assets.json");

        let json = fs::read_to_string(path).expect(ERROR_MESSAGE);

        let json = serde_json::from_str::<serde_json::Value>(&json).expect(ERROR_MESSAGE);

        match image_path {
            Some(path) => {
                let image = Self::find_image(&json, path);
                let points = Self::get_points(&image);
                Either::Left(points)
            }
            None => {
                let images = json.as_array().expect(ERROR_MESSAGE);
                let mut images_points = HashMap::new();

                for image in images {
                    let points = Self::get_points(image);
                    let image_path = image["path"].as_str().expect(ERROR_MESSAGE).to_string();
                    images_points.insert(image_path, points);
                }

                Either::Right(images_points)
            }
        }
    }

    fn find_image<'a>(json: &'a serde_json::Value, image_path: &'a str) -> &'a serde_json::Value {
        json.as_array()
            .expect("Failed to parse .cazan/build/assets.json file")
            .iter()
            .find(|image| {
                image["path"]
                    .as_str()
                    .expect("Failed to parse .cazan/build/assets.json file")
                    == image_path.replace('\\', "/")
            })
            .expect("Failed to parse .cazan/build/assets.json file")
    }

    fn get_points(image: &serde_json::Value) -> Vec<Self> {
        let points = image["points"]
            .as_array()
            .expect("Failed to parse .cazan/build/assets.json file");

        points
            .iter()
            .map(|json_point| Self {
                point: Point2 {
                    x: json_point["x"]
                        .clone()
                        .as_u64()
                        .expect("Failed to parse .cazan/build/assets.json file")
                        as u32,
                    y: json_point["y"]
                        .clone()
                        .as_u64()
                        .expect("Failed to parse .cazan/build/assets.json file")
                        as u32,
                },
                n: json_point["n"]
                    .clone()
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
            println!("Current working directory: {}", std::env::current_dir().unwrap().display());
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
        });
    }

    fn cleanup() {
        INIT.call_once(|| {
            fs::remove_dir_all(".cazan").expect("Failed to remove .cazan directory");
        });
    }

    #[test]
    fn test_from_json() {
        init();

        let points = Point::from(Some("assets/test.png")).left().unwrap();

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

        let images_points = Point::from(None).right().unwrap();

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

        cleanup();
    }
    // If you add a new test, move the cleanup call to the end of the test
}
