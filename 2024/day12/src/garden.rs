use std::collections::{HashMap, HashSet, VecDeque};

pub struct GardenMap {
    width: i32,
    height: i32,
    plots: Vec<char>
}

impl GardenMap {
    pub fn from_lines(lines: &Vec<String>) -> GardenMap {
        let mut width = 0;
        let mut plots = vec![];

        for line in lines.iter() {
            width = line.len();
            
            for char in line.chars() {
                plots.push(char);
            }
        }

        let width: i32 = width.try_into().unwrap();
        let height: i32 = lines.len().try_into().unwrap();

        GardenMap { width, height, plots }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn plot_at(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let i = y * self.width + x;
            let i: usize = i.try_into().unwrap();

            self.plots.get(i).copied()
        }
    }

    pub fn find_regions(&self) {


        todo!()
    }
}

pub struct RegionFinder<'a> {
    garden_map: &'a GardenMap
}

impl<'a> RegionFinder<'a> {
    pub fn new(garden_map: &GardenMap) -> RegionFinder {
        RegionFinder { garden_map }
    }

    pub fn walk(&mut self) -> usize {
        let mut regions: HashMap<usize, Vec<(i32, i32, char)>> = HashMap::new();
        let mut visited = HashSet::new();
        let mut walkers = VecDeque::new();
        
        let mut next_walker_id = 0;
        for x in 0..self.garden_map.width() {
            for y in 0..self.garden_map.height() {
                if let Some(plot) = self.garden_map.plot_at(x, y) {
                    let walker_id = next_walker_id;
                    next_walker_id += 1;

                    walkers.push_back((walker_id, x, y, plot));
                }
            }
        }

        loop {
            if let Some((walker_id, x, y, plot)) = walkers.pop_front() {
                if visited.insert((x, y)) {
                    regions.entry(walker_id)
                        .and_modify(|e| e.push((x, y, plot)))
                        .or_insert(vec![(x, y, plot)]);
                    
                    for (next_x, next_y) in vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                        if let Some(next_plot) = self.garden_map.plot_at(next_x, next_y) {
                            if next_plot == plot {
                                walkers.push_front((walker_id, next_x, next_y, next_plot))
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        let mut total_price = 0;
        for (_, region) in regions {
            let mut perimeter = 0;
            for (x, y, plot) in region.iter().copied() {
                for (next_x, next_y) in vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                    if let Some(next_plot) = self.garden_map.plot_at(next_x, next_y) {
                        if next_plot != plot {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                }   
            }
            
            let area = region.len();
            let region_price = perimeter * area;

            total_price += region_price;
        }

        total_price
    }
}