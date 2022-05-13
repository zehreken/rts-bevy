use bevy::{
    core::Time,
    prelude::{Commands, Plugin, Query, Res},
};

use crate::{Actor, Attack, Life};

pub struct FightPlugin;

fn fight_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&Actor, &mut Attack, &Life)>,
) {
    let delta_seconds = time.delta_seconds();
    for (actor, mut attack, life) in query.iter_mut() {
        attack.timer += delta_seconds;
        if attack.timer >= attack.rate {
            attack.timer -= attack.rate;
            // attack
        }
    }
}

impl Plugin for FightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(fight_system);
    }
}
