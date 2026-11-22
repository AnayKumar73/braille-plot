pub struct BrailleGrid {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl BrailleGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec!['\u{2800}'; cols * rows],
        }
    }

    pub fn set_at(&mut self, x: usize, y: usize, c: char) {
        if x >= self.cols || y >= self.rows {
            return;
        }
        self.data[y * self.cols + x] = c;
    }

    pub fn set_batch(&mut self, v: Vec<(usize, usize, char)>) {
        for (x, y, c) in v {
            if x >= self.cols || y >= self.rows {
                continue;
            }
            self.data[y * self.cols + x] = c;
        }
    }

    pub fn get_at(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.cols || y >= self.rows {
            return None
        }
        Some(self.data[y * self.cols + x])
    }

    pub fn to_string(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for r in 0..self.rows {
            let s = &self.data[(r * self.rows)..(r + 1) * self.rows];
            res.push(s.into_iter().collect());
        }
        res
    }
    
}

fn print_vector(v: Vec<String>) {
    for s in v {
        println!("{}", s);
    }
}


fn main() {
    let mut test = BrailleGrid::new(10,10);
    test.set_batch(Vec::from([(1, 2, 'x'), (4, 5, 'x'), (7, 8, 'y')]));
    print_vector(test.to_string());
}


/*
 *
 *
 *
 */

