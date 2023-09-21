use std::fs;
use std::path::Path;
use mint::Point2;

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

    #[cfg(feature = "points_import")]
    pub fn from_json(image_path: &str) -> Vec<Self> {
        let path = Path::new(".cazan/build/assets.json");

        let json = fs::read_to_string(path).expect("Failed to read .cazan/build/assets.json file");

        // Parse json
        let json = serde_json::from_str::<serde_json::Value>(&json).expect("Failed to parse .cazan/build/assets.json file");

        // Get the image
        let image = json
            .as_array()
            .expect("Failed to parse .cazan/build/assets.json file")
            .iter()
            .find(|image| {
                image["path"].as_str().expect("Failed to parse .cazan/build/assets.json file") == image_path.replace('\\', "/")
            })
            .expect("Failed to parse .cazan/build/assets.json file");

        // Get the points
        let points = image["points"]
            .as_array()
            .expect("Failed to parse .cazan/build/assets.json file");

        // Convert the points to Points
        points
            .iter()
            .map(
                |json_point| Self {
                    point: Point2 {
                        x: json_point["x"].clone().as_u64().expect("Failed to parse .cazan/build/assets.json file") as u32,
                        y: json_point["y"].clone().as_u64().expect("Failed to parse .cazan/build/assets.json file") as u32,
                    },
                    n: json_point["n"].clone().as_u64().expect("Failed to parse .cazan/build/assets.json file") as usize,
                }
            )
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
        fs::remove_dir_all(".cazan").expect("Failed to remove .cazan directory");
    }

    #[test]
    fn test_from_json() {
        init();

        let points = Point::from_json("assets/test.png");

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

        cleanup();
    }
    // If you add a new test, move the cleanup call to the end of the test
}