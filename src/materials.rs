use bevy::prelude::*;
use crate::components::*;

#[derive(Component)]
pub struct NeedsMaterial(Color);

pub fn apply_materials(
    mut commands: Commands,
    needs_mat: Query<(Entity, &NeedsMaterial), Without<MeshMaterial3d<StandardMaterial>>>,
    children_q: Query<&Children>,
    mut mesh_materials: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut done: Local<Vec<Entity>>,
) {
    for (entity, desired) in &needs_mat {
        if done.contains(&entity) {
            continue;
        }

        let handle = materials.add(StandardMaterial {
            base_color: desired.0,
            ..default()
        });

        let mut stack = vec![entity];
        while let Some(current) = stack.pop() {
            if let Ok(children) = children_q.get(current) {
                for child in children.iter() {
                    if let Ok(mut mat) = mesh_materials.get_mut(child) {
                        *mat = MeshMaterial3d(handle.clone());
                    }
                    stack.push(child);
                }
            }
        }

        commands.entity(entity).remove::<NeedsMaterial>();
        done.push(entity);
    }
}

pub fn setup_materials(
    mut commands: Commands,
    player_q: Query<Entity, (With<Player>, Without<NeedsMaterial>)>,
    enemy_q: Query<Entity, (With<Enemy>, Without<NeedsMaterial>)>,
) {
    if let Ok(e) = player_q.single() {
        commands.entity(e).insert(NeedsMaterial(Color::srgb(0.15, 0.55, 0.85)));
    }

    if let Ok(e) = enemy_q.single() {
        commands.entity(e).insert(NeedsMaterial(Color::srgb(0.75, 0.12, 0.12)));
    }
}
