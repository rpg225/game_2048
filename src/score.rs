use bevy::prelude::*;
#[derive(Resource)]
pub struct Score{
    pub value:u32,
}
#[derive(Component)]
pub struct ScoreText;
pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.insert_resource(Score{value:0});
    commands.spawn(TextBundle{
        text: Text::from_section(
            "Score: 0",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        style: Style{
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .insert(ScoreText);
}
pub fn update_score(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
){
    for mut text in query.iter_mut(){
        text.sections[0].value = format!{"Score: {}", score.value}
    }
}