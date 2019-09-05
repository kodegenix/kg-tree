#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeDiffOptions {
    detect_move: bool,
    min_count: Option<u32>,
    max_distance: Option<f64>,
}

impl NodeDiffOptions {
    pub fn new(detect_move: bool, min_count: Option<u32>, max_distance: Option<f64>) -> Self {
        NodeDiffOptions {
            detect_move,
            min_count,
            max_distance,
        }
    }

    pub fn detect_move(&self) -> bool {
        self.detect_move
    }

    pub fn set_detect_move(&mut self, detect_move: bool) {
        self.detect_move = detect_move;
    }

    pub fn min_count(&self) -> Option<u32> {
        self.min_count
    }

    pub fn set_min_count(&mut self, min_count: Option<u32>) {
        self.min_count = min_count
    }

    pub fn max_distance(&self) -> Option<f64> {
        self.max_distance
    }

    pub fn set_max_distance(&mut self, max_distance: Option<f64>) {
        self.max_distance = max_distance
    }
}