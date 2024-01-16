use super::point::Point;
use crate::level::point::Coordinate;
use crate::BlockType;
use bevy::prelude::{Commands, Resource};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use bevy_rapier2d::prelude::*;
use std::path::Path;
use thiserror::Error;

use crate::constants;

/// Read a file and generate a level out if it
/// Example level below
/// 4         -- height
/// 0 1       -- starting point
/// 4 6       -- finishing_point
/// o o o / o
/// o o o o o
/// o x x o o
/// o o o \ o
///
/// Note that we 1 index.
///
/// We start in bottom left, laser from the left side, and we are supposed to end up in the upper
/// left, laser leaving on the right side:
///
/// o o o / -
/// o o o | o
/// o x x | o
/// - - - / o
///
/// is the solution here
pub fn read_level_from_file(file: &Path) -> Result<Level, LevelCreationError> {
    let x = String::from_utf8(std::fs::read(file).unwrap())
        .unwrap()
        .replace("\r", "");
    let level_vec: Vec<&str> = x.split('\n').collect();

    if level_vec.len() < 3 {
        panic!("Input too short - at least three lines expected")
    }

    let height = level_vec[0].parse::<isize>().unwrap();
    let start = Point::new_from_tuple(parse_line_to_tuple(level_vec[1])?);
    let finish = Point::new_from_tuple(parse_line_to_tuple(level_vec[2])?);

    let mut grid: Vec<Vec<BlockType>> = vec![];
    for line_index in 3..(height + 3) {
        grid.push(parse_line_to_block_types(level_vec[line_index as usize]));
    }

    return Ok(Level::new(grid, start, finish)?);
}

fn parse_line_to_tuple(line: &str) -> Result<(isize, isize), LevelCreationError> {
    let line_vec: Vec<&str> = line.split(' ').collect();
    if line_vec.len() != 2 {
        return Err(LevelCreationError::LineParse(line.to_string()));
    };

    let x = line_vec[0].parse::<isize>().unwrap();
    let y = line_vec[1].parse::<isize>().unwrap();
    Ok((x, y))
}

fn parse_line_to_block_types(line: &str) -> Vec<BlockType> {
    line.chars()
        .filter(|c| c != &' ')
        .map(|character| BlockType::from_char(&character))
        .collect()
}

#[derive(Debug, Resource)]
pub struct Level {
    grid: Vec<Vec<BlockType>>,
    starting_point: Point,
    finishing_point: Point,
}

impl Level {
    /// We are unable to construct a level that is not valid.
    /// We are able to construct a level which is unsolvable
    pub fn new(
        grid: Vec<Vec<BlockType>>,
        starting_point: Point,
        finishing_point: Point,
    ) -> Result<Self, LevelCreationError> {
        let height = grid.len() as isize;
        if height == 0 {
            return Err(LevelCreationError::EmptyLevel);
        }
        let width = grid[0].len() as isize;
        Self::verify_point(height, width, starting_point)?;
        Self::verify_point(height, width, finishing_point)?;

        for row in grid.iter() {
            if row.len() as isize != width {
                return Err(LevelCreationError::GridWidth(
                    width,
                    row.len() as isize,
                    row.to_vec(),
                ));
            }
        }

        Ok(Self {
            grid,
            starting_point,
            finishing_point,
        })
    }

    fn verify_point(height: isize, width: isize, point: Point) -> Result<(), LevelCreationError> {
        let x_per = Self::is_perimeter(height, width, point.x);
        let y_per = Self::is_perimeter(height, width, point.y);

        if (x_per && y_per) || (!x_per && !y_per) {
            return Err(LevelCreationError::PointLocation(
                point,
                height + 1,
                width + 1,
            ));
        }

        Ok(())
    }

    fn is_perimeter(height: isize, width: isize, coord: Coordinate) -> bool {
        coord == height + 1 || coord == width + 1 || coord == 0
    }

    pub fn get_height(&self) -> isize {
        self.grid.len() as isize
    }

    pub fn get_width(&self) -> isize {
        self.grid[0].len() as isize
    }

    pub fn get_grid(&self) -> &Vec<Vec<BlockType>> {
        &self.grid
    }
}

#[derive(Debug, Error)]
pub enum LevelCreationError {
    #[error("Empty level!")]
    EmptyLevel,
    #[error("Expected width {0}, got {1} on row {2:?}")]
    GridWidth(isize, isize, Vec<BlockType>),
    #[error("Point {0:?} should be on the perimeter of the grid - one and only one of the coordinates should be 0 or {1} or {2}")]
    PointLocation(Point, isize, isize),
    #[error("Expected two isize elements, got {0}")]
    LineParse(String),
}
