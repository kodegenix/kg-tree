use super::*;

pub struct DiffOptions {
    detect_move: bool,
    min_count: Option<u32>,
    min_distance: Option<f64>,
}