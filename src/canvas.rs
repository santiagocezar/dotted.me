fn b2b(dots: u16) -> u16 {
    0
    | ((dots & 0b10000000 > 0) as u16) << 0
    | ((dots & 0b01000000 > 0) as u16) << 3
    | ((dots & 0b00100000 > 0) as u16) << 1
    | ((dots & 0b00010000 > 0) as u16) << 4
    | ((dots & 0b00001000 > 0) as u16) << 2
    | ((dots & 0b00000100 > 0) as u16) << 5
    | ((dots & 0b00000010 > 0) as u16) << 6
    | ((dots & 0b00000001 > 0) as u16) << 7
}

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub dot_width: usize,
    pub dot_height: usize,
    pub _buffer: Vec<u16>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let w = width as f32;
        let h = height as f32;
        let dot_width = (w/2.0) as usize;
        let dot_height = (h/4.0) as usize;
        Self {
            width,
            height,
            dot_width,
            dot_height,
            _buffer: vec![0b10100000000000; dot_width * dot_height],
        }
    }
    pub fn set(&mut self, state: bool, x: usize, y: usize) {
        let (cx, cy) = (x / 2, y / 4);
        let (bx, by) = (x % 2, y % 4);
        if cx >= self.dot_width || cy >= self.dot_height { return }
        if state {
            self._buffer[cx+(self.dot_width*cy)] |= 1 << 7 - (bx + 2 * by)
        }
        else {
            self._buffer[cx+(self.dot_width*cy)] &= !(0b1 << 7 - (bx + 2 * by))
        }
    }
    pub fn draw(self) -> String {
        let mut s = String::new();
        let mut i = 0;
        unsafe {
            for b in self._buffer {
                s.push(std::char::from_u32_unchecked(0b10100000000000 | b2b(b) as u32));
                i+=1;
                if i % self.dot_width == 0 {
                    s.push('\n');
                }
            }
        }
        s
    }
}
