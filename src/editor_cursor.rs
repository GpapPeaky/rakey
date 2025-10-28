pub struct EditorCursor {
    pub x: usize,
    pub y: usize
}

impl EditorCursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}