use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_shapes(num_shapes: usize) -> Vec<String> {
    let shapes = vec![
        "square".to_string(),
        "triangle".to_string(),
        "rectangle".to_string(),
        "circle".to_string(),
        "rhombus".to_string(),
    ];
    let mut rng = thread_rng();
    let mut shapes = shapes;
    shapes.shuffle(&mut rng);

    shapes.into_iter().take(num_shapes).collect()
}
