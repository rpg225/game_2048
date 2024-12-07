use bevy::prelude::*;
use itertools::Itertools;

#[derive(Component)]
pub struct Playground{
    pub grid: u8,
    pub size: f32,
}

const TILE_SIZE: f32 = 100.0;
const TILE_SPACER: f32 =10.0;

impl Playground{
    pub fn new(grid:u8) -> Self{
        let size = f32::from(grid)*TILE_SIZE + f32::from(grid+1)*TILE_SPACER;
        Playground{grid, size}
    }

    pub fn tile_pos(&self, pos: u8) -> f32{
        let offset = -self.size/2.0 + 0.5 *TILE_SIZE;
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos+1)*TILE_SPACER
    }


}

pub fn spawn_playground(mut commands: Commands){
    let playground = Playground::new(4);
    commands
    .spawn(SpriteBundle{
        sprite: Sprite{
            color:Color::rgb(228.0, 183.0/255.0),
            custom_size:Some(Vec2::new(playground.size, playground.size)),
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        for tile in (0..playground.grid).cartesian_product(0..playground.grid) {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    playground.tile_pos(tile.0),
                    playground.tile_pos(tile.1),
                    0.1,
                ),
                ..default()
            });
        }
    })
    .insert(playground);
} 
