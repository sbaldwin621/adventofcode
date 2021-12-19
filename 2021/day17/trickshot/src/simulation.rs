use std::cmp::max;

pub struct Simulator {
    target: (isize, isize, isize, isize)
}

impl Simulator {
    pub fn new(target: (isize, isize, isize, isize)) -> Simulator {
        Simulator { target }
    }

    pub fn simulate(&self, initial_velocity: (isize, isize)) -> SimulationResult {
        let mut highest_y = 0;
        let mut simulation = Simulation::new(initial_velocity);

        loop {
            simulation.step();

            let (x, y) = simulation.position;

            highest_y = max(highest_y, y);

            // Hit the target
            if x >= self.target.0 && x <= self.target.1 && y >= self.target.2 && y <= self.target.3 {
                return SimulationResult::Hit(highest_y);
            }

            // Missed the target
            if y < self.target.2 && simulation.velocity.0 == 0 {
                return SimulationResult::Missed(x)
            }
        }
    }
}

pub enum SimulationResult {
    Missed(isize),
    Hit(isize)
}

pub struct Simulation {
    position: (isize, isize),
    velocity: (isize, isize)
}

impl Simulation {
    pub fn new(velocity: (isize, isize)) -> Simulation {
        Simulation { velocity, position: (0, 0) }
    }

    pub fn position(&self) -> (isize, isize) {
        self.position
    }

    pub fn velocity(&self) -> (isize, isize) {
        self.velocity
    }

    pub fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Drag
        if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        } else if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        }

        // Gravity
        self.velocity.1 -= 1;
    }
}