#[derive(Debug)]
pub struct MapBuilder {
    map: Vec<u8>,
    width: Option<usize>
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder { width: None, map: vec![] }
    }

    pub fn add_line(&mut self, line: &String) {
        if self.width == None {
            self.width = Some(line.len());
        }

        for char in line.chars() {
            self.map.push(char.to_string().parse::<u8>().unwrap());
        }
    }

    pub fn to_map(self) -> Map {
        Map { map: self.map, width: self.width.unwrap() }
    }
}

pub struct Map {
    map: Vec<u8>,
    width: usize
}

impl Map {
    pub fn get(&self, x: i64, y: i64) -> Option<u8> {
        let i: i64 = y * (self.width as i64) + x;
        if i >= 0 {
            self.map.get(i as usize).map(|v| *v)
        } else {
            None
        }
    }

    pub fn is_low_point(&self, x: i64, y: i64) -> bool {
        let this = self.get(x, y).unwrap();
        let north = self.get(x, y - 1);
        let west = self.get(x - 1, y);
        let east = self.get(x + 1, y);
        let south = self.get(x, y + 1);

        if let Some(north) = north {
            if north <= this {
                return false;
            }
        }

        if let Some(east) = east {
            if east <= this {
                return false;
            }
        }
        
        if let Some(south) = south {
            if south <= this {
                return false;
            }
        }
        
        if let Some(west) = west {
            if west <= this {
                return false;
            }
        }

        return true;
    }

    pub fn calculate_risk_level(&self) -> u64 {
        let mut total_risk_level: u64 = 0;

        for i in 0..self.map.len() {
            let x = (i % self.width) as i64;
            let y = (i / self.width) as i64;

            if self.is_low_point(x, y) {
                total_risk_level += (*self.map.get(i).unwrap()) as u64 + 1;
            }
        }

        total_risk_level
    }
}