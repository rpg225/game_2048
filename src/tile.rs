use crate::playground::Playground;
use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use crate::input::MoveTiles;
use crate::events::NewTileEvent;
use crate::score::Score;
#[derive(Component)]
pub struct TileText;
#[derive(Component)]
pub struct Points{
    pub value: u32,
}
#[derive(Component, PartialEq, Clone, Copy)]
pub struct Position{
    pub x: u8,
    pub y: u8,
}
pub fn spawn_tile(commands: &mut Commands, playground: &Playground, pos: Position){
    commands
    .spawn(SpriteBundle{
        sprite:Sprite{
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        transform: Transform::from_xyz(
            playground.tile_pos(pos.x),
            playground.tile_pos(pos.y),
            2.0,
        ),
        ..default()
    })
    .with_children(|child_builder|{
        child_builder
        .spawn(Text2dBundle{
            text: Text::from_section(
                "2",
                TextStyle{
                    font_size: 50.0,
                    color:Color::rgb(195.0/255.0, 80.0/255.0, 72.0/255.0),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(TileText);
    })
    .insert(Points{value:2})
    .insert(pos);
}
pub fn spawn_tiles(mut commands: Commands, query_playground:Query<&Playground>){
    let playground = query_playground.single();
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8,u8)> = (0..playground.grid).cartesian_product(0..playground.grid).choose_multiple(&mut rng, 2);
    for (x,y) in starting_tiles.iter(){
        let pos = Position{x:*x, y:*y};
        spawn_tile(&mut commands, playground, pos)
    }
}
pub fn move_tiles(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    query_playground: Query<&Playground>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
){
    let playground = query_playground.single();
    let shift_direction = input
                            .get_just_pressed()
                            .find_map(|key_code| MoveTiles::try_from(key_code).ok());
    let mut highest_tile_value = 0;
    if let Some(move_tiles) = shift_direction{
        let mut it = tiles
                        .iter_mut()
                        .sorted_by(|a,b| move_tiles.sort(&a.1, &b.1))
                        .peekable();
        let mut column: u8 = 0;
        let mut any_tile_moved = false;
        while let Some(mut tile) = it.next(){
            let original_pos = tile.1.clone();
            move_tiles.set_column(playground.grid, &mut tile.1, column);
            if original_pos != *tile.1{
                any_tile_moved = true;
            }
            if let Some(peeked_tile) = it.peek(){
                if move_tiles.get_row(&tile.1) != move_tiles.get_row(&peeked_tile.1){
                    column = 0;
                } else if tile.2.value != peeked_tile.2.value{
                    column += 1;
                } else{
                    let next_tile = it.next().expect("expected peeked tile");
                    tile.2.value *= 2;
                    score.value += tile.2.value;
                    highest_tile_value = highest_tile_value.max(tile.2.value);
                    if tile.2.value == 2048{
                        handle_game_win(&mut commands, &*asset_server);
                    }
                    commands.entity(next_tile.0).despawn_recursive();
                    any_tile_moved = true;
                    if let Some(more_tile) = it.peek(){
                        if move_tiles.get_row(&tile.1) != move_tiles.get_row(&more_tile.1){
                            column = 0;
                        } else {
                            column += 1;
                        }
                    }
                }
            }
        }
        if any_tile_moved{
            tile_writer.send(NewTileEvent);
        } else {
            println!("no tile has moved");
            let immut_tiles_data:Vec<(Position, u32)> = tiles.iter().map(|(_,pos,points)| (*pos, points.value)).collect();
            if !has_available_moves(&immut_tiles_data, playground.grid){
                handle_game_over(&mut commands, &*asset_server);
            }
        }
    }
}
pub fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
){
    for (points, children) in tiles.iter(){
        if let Some(entity) = children.first(){
            let mut text = texts.get_mut(*entity).expect("expected text");
            let text_section = text.sections.first_mut().expect("expecred editable");
            text_section.value = points.value.to_string()
        }
    }
}
pub fn render_tiles(
    mut tiles: Query<(&mut Transform, &Position), Changed<Position>>,
    query_playground: Query<&Playground>,
){
    let playground = query_playground.single();
    for (mut transform, pos) in tiles.iter_mut(){
        transform.translation.x = playground.tile_pos(pos.x);
        transform.translation.y = playground.tile_pos(pos.y);
    }
}
pub fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_playground: Query<&Playground>,
    tiles: Query<&Position>,
){
    let playground = query_playground.single();
    for _ in tile_reader.read(){
        let mut rng = rand::thread_rng();
        let possible_position: Option<Position> = (0..playground.grid).cartesian_product(0..playground.grid).filter_map(|tile_pos|{
            let new_pos = Position{
                x: tile_pos.0,
                y: tile_pos.1,
            };
            match tiles.iter().find(|&&pos| pos ==new_pos){
                Some(_) => None,
                None => Some(new_pos),
            }
        })
        .choose(&mut rng);
        if let Some(pos) = possible_position{
            spawn_tile(&mut commands, playground, pos);
        }
    }
}
fn handle_game_win(commands: &mut Commands, asset_server: &AssetServer){
    commands.spawn(TextBundle{
        style:Style{
            position_type:PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..Default::default()
        },
        text:Text{
            sections: vec![
                TextSection{
                    value: "You've won the game".to_string(),
                    style: TextStyle{
                        font_size: 40.0,
                        color:Color::WHITE,
                        ..default()
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    });
}
fn has_available_moves(tiles: &[(Position, u32)], grid_size: u8) -> bool{
    let mut positions = vec![vec![None; grid_size as usize]; grid_size as usize];
    for (pos,value) in tiles.iter(){
        positions[pos.x as usize][pos.y as usize] = Some(*value);
    }
    for x in 0..grid_size{
        for y in 0..grid_size{
            if positions[x as usize][y as usize].is_none(){
                return true;
            }
            let current_value = positions[x as usize][y as usize].unwrap();
            if x>0 && positions[(x-1) as usize][y as usize] == Some(current_value){
                return true;
            }
            if y>0 && positions[x as usize][(y-1) as usize] == Some(current_value){
                return true;
            }
            if x < grid_size-1 && positions[(x+1) as usize][y as usize] == Some(current_value){
                return true
            }
            if y < grid_size-1 && positions[x as usize][(y+1) as usize] == Some(current_value){
                return true
            }
        }
    }
    false
}
fn handle_game_over(commands: &mut Commands, asset_server: &AssetServer){
    commands.spawn(TextBundle{
        style:Style{
            position_type:PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..Default::default()
        },
        text:Text{
            sections: vec![
                TextSection{
                    value: "Game Over".to_string(),
                    style: TextStyle{
                        font_size: 40.0,
                        color:Color::WHITE,
                        ..default()
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    });
}