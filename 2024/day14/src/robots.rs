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
}

pub struct Robot {
    starting_position: (i64, i64),
    velocity: (i64, i64)
}

impl Robot {
    pub fn new(starting_position: (i64, i64), velocity: (i64, i64)) -> Robot {
        Robot { starting_position, velocity }
    }
}

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let starting_position;
        let velocity;
        
        if let Some((position_s, velocity_s)) = s.split_once(' ') {
            if position_s.starts_with("p=") {
                let position_s = &position_s[2..];
                if let Some((x, y)) = position_s.split_once(',') {
                    let x: i64 = x.parse()?;
                    let y: i64 = y.parse()?;

                    starting_position = (x, y);
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

            Ok(Robot::new(starting_position, velocity))
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

        assert_eq!(robot.starting_position, (0, 4));
        assert_eq!(robot.velocity, (3, -3));
    }
}