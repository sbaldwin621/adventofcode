use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

pub struct RobotSimulation {
    robots: Vec<Robot>,
    arena_size: (i64, i64)
}

impl RobotSimulation {
    pub fn new(robots: Vec<Robot>, arena_size: (i64, i64)) -> RobotSimulation {
        RobotSimulation { robots, arena_size }
    }

    pub fn simulate(&mut self, seconds: i64) {
        let (arena_width, arena_height) = self.arena_size;

        for robot in self.robots.iter_mut() {
            let (pos_x, pos_y) = robot.position;
            let (vel_x, vel_y) = robot.velocity;

            let mut new_x = (pos_x + vel_x * seconds) % arena_width;
            if new_x < 0 {
                new_x = arena_width + new_x;
            }

            let mut new_y = (pos_y + vel_y * seconds) % arena_height;
            if new_y < 0 {
                new_y = arena_height + new_y;
            }

            *robot = robot.with_position((new_x, new_y));
        }
    }

    pub fn safety_factor(&self) -> u64 {
        let (arena_width, arena_height) = self.arena_size;

        let mut nw_count = 0;
        let mut ne_count = 0;
        let mut se_count = 0;
        let mut sw_count = 0;

        for robot in self.robots.iter() {
            let (x, y) = robot.position;

            // Northwest quadrant
            if x < arena_width / 2 && y < arena_height / 2 {
                nw_count += 1;
            }

            // Northeast quadrant
            if x > arena_width / 2 && y < arena_height / 2 {
                ne_count += 1;
            }

            // Southeast quadrant
            if x > arena_width / 2 && y > arena_height / 2 {
                se_count += 1;
            }

            // Southwest quadrant
            if x < arena_width / 2 && y > arena_height / 2 {
                sw_count += 1;
            }
        }

        nw_count * ne_count * se_count * sw_count
    }

    pub fn print_map(&self) {
        let (arena_width, arena_height) = self.arena_size;

        let mut arena = HashMap::new();

        for robot in self.robots.iter() {
            arena.entry(robot.position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for y in 0..arena_height {
            for x in 0..arena_width {
                if let Some(count) = arena.get(&(x, y)) {
                    print!("{}", count);
                } else {
                    print!(".");
                }
            }

            println!();
        }
    }

    pub fn detect_christmas_tree(&self) -> bool {
        let (arena_width, arena_height) = self.arena_size;

        let mut arena = HashMap::new();

        for robot in self.robots.iter() {
            arena.entry(robot.position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let mut robots_in_a_row = 0;

        for y in 0..arena_height {
            robots_in_a_row = 0;
            for x in 0..arena_width {
                if let Some(count) = arena.get(&(x, y)) {
                    robots_in_a_row += 1;

                    if robots_in_a_row > 10 {
                        return true;
                    }
                } else {
                    robots_in_a_row = 0;
                }
            }
        }

        false
    }
}

pub struct Robot {
    position: (i64, i64),
    velocity: (i64, i64)
}

impl Robot {
    pub fn new(position: (i64, i64), velocity: (i64, i64)) -> Robot {
        Robot { position, velocity }
    }

    pub fn with_position(&self, new_position: (i64, i64)) -> Robot {
        Robot {
            position: new_position,
            ..*self
        }
    }
}

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let position;
        let velocity;
        
        if let Some((position_s, velocity_s)) = s.split_once(' ') {
            if position_s.starts_with("p=") {
                let position_s = &position_s[2..];
                if let Some((x, y)) = position_s.split_once(',') {
                    let x: i64 = x.parse()?;
                    let y: i64 = y.parse()?;

                    position = (x, y);
                } else {
                    return Err(ParseRobotError::InvalidPositionString);
                }
            } else {
                return Err(ParseRobotError::InvalidPositionString);
            }

            if velocity_s.starts_with("v=") {
                let velocity_s = &velocity_s[2..];
                if let Some((x, y)) = velocity_s.split_once(',') {
                    let x: i64 = x.parse()?;
                    let y: i64 = y.parse()?;

                    velocity = (x, y);
                } else {
                    return Err(ParseRobotError::InvalidVelocityString);
                }
            } else {
                return Err(ParseRobotError::InvalidVelocityString);
            }

            Ok(Robot::new(position, velocity))
        } else {
            Err(ParseRobotError::InvalidRobotString)
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseRobotError {
    #[error("invalid robot string")]
    InvalidRobotString,
    #[error("invalid position string")]
    InvalidPositionString,
    #[error("invalid velocity string")]
    InvalidVelocityString,
    #[error("couldn't parse int: {0}")]
    ParseIntError(#[from] ParseIntError)
}

#[cfg(test)]
mod tests {
    use super::Robot;

    #[test]
    pub fn robot_string_parses() {
        let robot: Robot = "p=0,4 v=3,-3".parse().unwrap();

        assert_eq!(robot.position, (0, 4));
        assert_eq!(robot.velocity, (3, -3));
    }
}