
mod interpreter;
/*
Need to add support for negative window dimensions

*/
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

    pub fn set_at(&mut self, x: i32, y: i32) {
        let col = x / 2;
        let row = y / 4;
        let local_col = x % 2;
        let local_row = 3 - (y % 4);

        if col >= self.cols as i32 || row >= self.rows as i32{
            return;
        }

        let shift = 0x1 << ([1, 2, 3, 7, 4, 5, 6, 8][(local_col * 4 + local_row) as usize] - 1);

        self.data[(row * self.cols as i32 + col) as usize] |= shift;
 
    }

    pub fn set_batch(&mut self, v: Vec<(i32, i32)>) {
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

fn bresenham_line_algo(mut x0: i32, mut y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 {1} else {-1};
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 {1} else {-1};
    let mut err = if dx > dy {dx / 2} else {-dy / 2};
    let mut e2: i32;

    let mut res: Vec<(i32, i32)> = vec![];
    loop {
        res.push((x0, y0));
        if x0 == x1 && y0 == y1 {
            return res;
        }
        e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

fn main() {

    let x_window_min = 0;
    let x_window_max = 100;
    let y_window_min = 0;
    let y_window_max = 100;

    let x_norm = normalize(x_window_min, x_window_max - 1, 0, 100);
    let y_norm = normalize(y_window_min, y_window_max - 1, 0, 100);

    let mut test = BrailleGrid::new(x_window_max as usize, y_window_max as usize);

    let mut v: Vec<(i32, i32)> = vec![];
    // for x in 0..100 as i32 {
    //     let y = x;
    //     v.push((x, y));
        
    //     if !v.is_empty() {
    //         v.extend(bresenham_line_algo(v[v.len() - 1].0, v[v.len() - 1].1, x, y));
    //     }
    // }
    // let mut v = bresenham_line_algo(0, 0, 99, 40);
    // v.extend(bresenham_line_algo(0, 10, 99, 90));
    // v.extend(bresenham_line_algo(10, 0, 40, 99));

    for x in 0..1000 {
        let x_val = -2.0 * std::f64::consts::PI
            + x as f64 * 4.0 * std::f64::consts::PI / 999.0;

        let y_val = x_val.sin();

        let px = (x as f64 * 99.0 / 999.0).round() as i32;
        let py = ((y_val + 1.0) * 49.5).round() as i32;

        v.push((px, py));
    }

    let mut pixels = vec![];

    for pair in v.windows(2) {
        pixels.extend(bresenham_line_algo(
            pair[0].0, pair[0].1,
            pair[1].0, pair[1].1,
        ));
    }

    test.set_batch(pixels);


    let v: Vec<(i32, i32)> = v.iter()
        .map(|&(x, y)| ((x_norm.0 * x as f32 + x_norm.1).round() as i32, (y_norm.0 * y as f32 + y_norm.1).round() as i32)).collect();

    test.set_batch(v);

    print_vector(test.to_string());
}