use bevy::{
    core::Time,
    prelude::{Commands, Entity, Plugin, Query, Res},
};

use crate::{Attack, Life};

pub struct FightPlugin;

fn fight_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Attack, &Life)>,
) {
    let delta_seconds = time.delta_seconds();
    for (entity, mut attack, life) in query.iter_mut() {
        attack.timer += delta_seconds;
        if life.hp <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for FightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(fight_system);
    }
}
