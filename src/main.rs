pub struct BrailleGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl BrailleGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        let cols = cols.div_ceil(2);
        let rows = rows.div_ceil(4);

        Self {
            cols,
            rows,
            data: vec![0; cols * rows],
        }
    }

    pub fn set_at(&mut self, x: usize, y: usize) {
        let col = x / 2;
        let row = y / 4;
        let local_col = x % 2;
        let local_row = 3 - (y % 4);

        if col >= self.cols || row >= self.rows {
            return;
        }

        let shift = 0x1 << ([1, 2, 3, 7, 4, 5, 6, 8][local_col * 4 + local_row] - 1);

        self.data[row * self.cols + col] |= shift;
 
    }

    pub fn set_batch(&mut self, v: Vec<(usize, usize)>) {
        for (x, y) in v {
            self.set_at(x, y);
        }
    }

    pub fn to_string(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for r in (0..self.rows).rev() {
            let s: String = self.data[(r * self.cols)..((r + 1) * self.cols)]
                .iter()
                .map(|&x| char::from_u32(0x2800 + x as u32).unwrap())
                .collect();
            
            res.push(s);
        }
        res
    }
    
}

fn print_vector(v: Vec<String>) {
    for s in v {
        println!("{}", s);
    }
}

// scale factor
fn normalize(window_min: i32, window_max: i32, input_min: i32, input_max: i32) -> (f32, f32) {
    let scale: f32 = (window_max - window_min) as f32 / (input_max - input_min) as f32;
    let shift: f32 = window_min as f32 - scale * input_min as f32;

    (scale, shift)
}

fn bresenham_line_algo(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let dx = (x1 - x0).abs();
    let sx = x0 < x1 ? 

}

fn main() {

    let x_window_min = 0;
    let x_window_max = 100;
    let y_window_min = 0;
    let y_window_max = 100;

    let x_norm = normalize(x_window_min, x_window_max, 0, 100);
    let y_norm = normalize(y_window_min, y_window_max, 0, 100);

    let mut test = BrailleGrid::new(x_window_max as usize, y_window_max as usize);

    let mut v: Vec<(usize, usize)> = vec![];
    for x in 0..100 {
        v.push((x, x))
    }

    let v: Vec<(usize, usize)> = v.iter()
        .map(|&(x, y)| ((x_norm.0 * x as f32 + x_norm.1).round() as usize, (y_norm.0 * y as f32 + y_norm.1).round() as usize)).collect();

    test.set_batch(v);

    print_vector(test.to_string());
}


/*
 * Need to figure out scaling somehow
 * would be 
 *
 * 
 * if we have an range, then we have 2n x values and 2y y values
 * want to map the x range down to window of size x_max
 */

