use bevy::prelude::*;

use crate::{
    character::{spawn_character_sprite, CharacterSheet},
    fadeout::create_fadeout,
    GameState,
};

#[derive(Component)]
pub struct Enemy;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Combat)
                .with_system(test_exit_combat)
                .with_system(combat_camera),
        )
        .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(spawn_enemy))
        .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy));
    }
}

fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, character: Res<CharacterSheet>) {
    let sprite = spawn_character_sprite(
        &mut commands,
        &character,
        'b' as usize,
        Color::rgb(0.8, 0.8, 0.1),
        Vec3::new(0.0, 0.5, 100.0),
    );
    commands
        .entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Bat"));
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_combat(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    character: Res<CharacterSheet>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        create_fadeout(&mut commands, GameState::Overworld, &character)
    }
}
