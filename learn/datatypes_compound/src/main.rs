fn main() {
    // Tuples are groups of data which is immutable.
    let shape_data = ("Square", 4, true);
    // Data is accessed from a tuple using <tuple>.<index> notation
    println!(
        "{} has {} sides. Is it a quadrilateral? : {}",
        shape_data.0, shape_data.1, shape_data.2
    );
    // Tuples can be destructured
    let (shape, sides_count, is_quadrilateral) = shape_data;
    println!(
        "{} has {} sides. Is it a quadrilateral? : {}",
        shape, sides_count, is_quadrilateral
    );
}
