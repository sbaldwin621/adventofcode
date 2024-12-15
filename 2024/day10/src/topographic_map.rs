use std::collections::HashSet;

pub struct TopographicMap {
    width: usize,
    height: usize,
    contents: Vec<Vec<u32>>
}

impl TopographicMap {
    pub fn from_lines(lines: &Vec<String>) -> TopographicMap {
        let height = lines.len();
        let mut width = 0;
        
        let mut contents = vec![];

        for line in lines.iter() {
            width = line.len();

            let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            contents.push(row);
        }

        TopographicMap { width, height, contents }
    }

    pub fn score(&self) -> usize {
        let mut score = 0;

        for (y, row) in self.contents.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let tile = *tile;

                if tile == 0 {
                    score += self.follow_trail(x, y);
                }
            }
        }

        score
    }

    fn follow_trail(&self, x: usize, y: usize) -> usize {
        let mut already_traversed = HashSet::new();
        let mut nines = HashSet::new();
        
        let x: i32 = x.try_into().unwrap();
        let y: i32 = y.try_into().unwrap();

        let mut current_points = vec![(x, y, 0)];

        while current_points.len() > 0 {
            let mut next_points = vec![];
            
            for (x, y, height) in current_points {
                if !already_traversed.insert((x, y)) {
                    continue;
                }

                if height == 9 {
                    nines.insert((x, y));
                    continue;
                }
                
                let target_height = height + 1;
                for (next_x, next_y) in vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                    if let Some(next_height) = self.height_at(next_x, next_y) {
                        if next_height == target_height {
                            next_points.push((next_x, next_y, next_height))
                        }
                    }
                }
            }

            current_points = next_points;
        }

        nines.len()
    }

    fn height_at(&self, x: i32, y: i32) -> Option<u32> {
        let width: i32 = self.width.try_into().unwrap();
        let height: i32 = self.height.try_into().unwrap();

        if x < 0 || x >= width || y < 0 || y >= height {
            None
        } else {
            let y: usize = y.try_into().unwrap();
            let x: usize = x.try_into().unwrap();

            Some(self.contents[y][x])
        }
    }

}