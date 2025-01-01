use bevy::prelude::*;
use std::cmp::Ordering;
use crate::tile::Position;

pub enum MoveTiles{
    Left,
    Right,
    Up,
    Down,
}

impl MoveTiles{
    pub fn sort(&self, a: &Position, b:&Position) -> Ordering{
        match self{
            MoveTiles::Left => match Ord::cmp(&a.x, &b.x){
            Ordering::Equal => Ord::cmp(&a.x, &b.x), ordering => ordering,
            },
            MoveTiles::Right => match Ord::cmp(&a.x, &b.x){
                Ordering::Equal => Ord::cmp(&b.x, &a.x), ordering => ordering,
            },
            MoveTiles::Up => match Ord::cmp(&b.x, &a.x){
                Ordering::Equal => Ord::cmp(&b.y, &a.y), ordering => ordering,
            },
            MoveTiles::Down => match Ord::cmp(&b.y, &a.y){
                Ordering::Equal => Ord::cmp(&a.y, &a.y), ordering => ordering,
            },
        }
    }

    pub fn set_column(&self, playground_grid: u8, position: &mut Mut<Position>, index: u8){
        match self{
            MoveTiles::Left => {
                position.x = index;
            }

            MoveTiles::Right => {
                position.x = playground_grid -1 -index;
            }

            MoveTiles::Up => {
                position.y = playground_grid -1 - index;
            }

            MoveTiles::Down => {
                position.x = index;
            }


        }
    }

    pub fn get_row(&self, position: &Position) -> u8{
        match self {
            MoveTiles::Left | MoveTiles::Right => position.y,
            MoveTiles::Up | MoveTiles::Down => position.x,
        }
    }
}

impl TryFrom<&KeyCode> for MoveTiles{
    type Error = & 'static str;

    fn  try_from(value: &KeyCode) -> Result<Self, Self::Error>{
        match value{
            KeyCode::ArrowLeft => Ok(MoveTiles::Left),
            KeyCode::ArrowRight => Ok(MoveTiles::Right),
            KeyCode::ArrowUp => Ok(MoveTiles::Up),
            KeyCode::ArrowDown => Ok(MoveTiles::Down),
            _=>Err("please use arrow keys"),

        }
    }
}