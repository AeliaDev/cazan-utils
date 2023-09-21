use std::fs;
use std::sync::Once;

use super::Point;

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
