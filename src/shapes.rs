struct Shape {
    length: u8,
    width: u8,
    top_right: (u8, u8),
    top_left: (u8, u8),
    bottom_right: (u8, u8),
    bottom_left: (u8, u8),
    velocity: f32,
    acceleration: f32,
    shape_type: String, // a mapping must be created between these strings and the respective draw function
}