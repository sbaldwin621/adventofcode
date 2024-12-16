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

    pub fn calculate_fence_prices(mut self) -> RegionFencePrice {
        let mut regions: HashMap<usize, Region> = HashMap::new();
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
                    let region = regions.entry(walker_id).or_insert_with(|| Region::new());
                    region.add_plot(x, y);
                    
                    let mut neighbors = vec![];
                    for (next_x, next_y) in vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                        let neighbor = self.garden_map.plot_at(next_x, next_y);
                        neighbors.push(neighbor);
                        
                        if let Some(next_plot) = neighbor {
                            if next_plot == plot {
                                walkers.push_front((walker_id, next_x, next_y, next_plot))
                            } else {
                                region.add_edge();
                            }
                        } else {
                            region.add_edge();
                        }
                    }

                    let north_neighbor = neighbors[0];
                    let east_neighbor = neighbors[1];
                    let south_neighbor = neighbors[2];
                    let west_neighbor = neighbors[3];
                    
                    let plot = Some(plot);
                    
                    // northeast corner check
                    if plot != north_neighbor && plot != east_neighbor {
                        region.add_corner();
                    }

                    if plot == north_neighbor && plot == east_neighbor {
                        let northeast_neighbor = self.garden_map.plot_at(x + 1, y - 1);
                        if plot != northeast_neighbor {
                            region.add_corner();
                        }
                    }

                    // southeast corner check
                    if plot != south_neighbor && plot != east_neighbor {
                        region.add_corner();
                    }

                    if plot == south_neighbor && plot == east_neighbor {
                        let southeast_neighbor = self.garden_map.plot_at(x + 1, y + 1);
                        if plot != southeast_neighbor {
                            region.add_corner();
                        }
                    }

                    // southwest corner check
                    if plot != south_neighbor && plot != west_neighbor {
                        region.add_corner();
                    }

                    if plot == south_neighbor && plot == west_neighbor {
                        let southwest_neighbor = self.garden_map.plot_at(x - 1, y + 1);
                        if plot != southwest_neighbor {
                            region.add_corner();
                        }
                    }

                    // northwest corner check
                    if plot != north_neighbor && plot != west_neighbor {
                        region.add_corner();
                    }

                    if plot == north_neighbor && plot == west_neighbor {
                        let northwest_neighbor = self.garden_map.plot_at(x - 1, y - 1);
                        if plot != northwest_neighbor {
                            region.add_corner();
                        }
                    }
                }
            } else {
                break;
            }
        }

        let mut total_price = 0;
        let mut discount_price = 0;
        
        for (_, region) in regions {
            let perimeter = region.perimeter();         
            let sides = region.sides();
            let area = region.area();
            
            let region_price = perimeter * area;
            let region_discount_price = sides * area;

            total_price += region_price;
            discount_price += region_discount_price;
        }

        RegionFencePrice { total_price, discount_price }
    }
}

#[derive(Clone, Copy)]
pub struct RegionFencePrice {
    total_price: usize,
    discount_price: usize
}

impl RegionFencePrice {
    pub fn total_price(&self) -> usize {
        self.total_price
    }

    pub fn discount_price(&self) -> usize {
        self.discount_price
    }
}

struct Region {
    plots: Vec<(i32, i32)>,
    edges: usize,
    corners: usize
}

impl Region {
    pub fn new() -> Region {
        let plots = vec![];
        let edges = 0;
        let corners = 0;

        Region { plots, edges, corners }
    }
    
    pub fn add_plot(&mut self, x: i32, y: i32) {
        self.plots.push((x, y));
    }
    
    pub fn perimeter(&self) -> usize {
        self.edges
    }

    pub fn sides(&self) -> usize {
        self.corners
    }

    pub fn area(&self) -> usize {
        self.plots.len()
    }

    pub fn add_edge(&mut self) {
        self.edges += 1;
    }

    pub fn add_corner(&mut self) {
        self.corners += 1;
    }
}