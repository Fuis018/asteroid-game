use bevy::prelude::*;
use crate::components::*;

pub fn apply_player_materials(
    player_q: Query<Entity, With<Player>>,
    children_q: Query<&Children>,
    mesh_materials_q: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    let Ok(player) = player_q.single() else {
        return;
    };

    let mut mat_handles: Vec<Handle<StandardMaterial>> = Vec::new();
    let mut stack = vec![player];
    while let Some(entity) = stack.pop() {
        if let Ok(h) = mesh_materials_q.get(entity) {
            mat_handles.push(h.0.clone());
        }
        if let Ok(children) = children_q.get(entity) {
            for child in children.iter() {
                stack.push(child);
            }
        }
    }

    if mat_handles.is_empty() {
        return;
    }

    let colors = [
        Color::srgb(0.3, 0.7, 0.3),
        Color::srgb(0.5, 0.5, 0.5),
        Color::srgb(0.8, 0.15, 0.1),
        Color::srgb(0.15, 0.15, 0.15),
        Color::srgb(0.3, 0.3, 0.35),
        Color::srgb(1.0, 0.8, 0.0),
        Color::srgb(0.25, 0.25, 0.25),
        Color::srgb(0.55, 0.55, 0.6),
        Color::srgb(1.0, 0.8, 0.0),
        Color::srgb(0.25, 0.25, 0.25),
        Color::srgb(0.35, 0.45, 0.6),
        Color::srgb(0.55, 0.55, 0.6),
        Color::srgb(0.25, 0.25, 0.25),
    ];

    for (i, h) in mat_handles.iter().enumerate() {
        if let Some(mut mat) = materials.get_mut(h) {
            if i < colors.len() {
                mat.base_color = colors[i];
                if i == 5 || i == 8 {
                    mat.emissive = LinearRgba::new(1.0, 0.8, 0.0, 1.0);
                }
            }
        }
    }

    *done = true;
}
