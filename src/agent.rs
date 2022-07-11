use std::ops::DerefMut;

use crate::loading::TextureAssets;
use crate::windows::UiStates;
use crate::GameState;

use bevy::prelude::*;
use bevy::render::camera::{Camera2d, RenderTarget};

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_agent))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_agent));
    }
}

#[derive(Debug, Component)]
pub struct Agent {
    pub name: String,
    lifespan: i64,
}

fn spawn_agent(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Agent {
            name: "john".to_string(),
            lifespan: 0,
        });
}

fn update_agent(
    agent_query: Query<(&Agent, &Transform, &Sprite, Entity)>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut ui_states: ResMut<UiStates>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = win.cursor_position() {
            println!("click at {:?}", cursor_pos);

            // convert the cursor position to a world position
            // get the camera info and transform
            // assuming there is exactly one main camera entity, so query::single() is OK
            let (camera, camera_transform) = camera_query.single();

            // get the window that the camera is displaying to (or the primary window)
            let wnd = if let RenderTarget::Window(id) = camera.target {
                windows.get(id).unwrap()
            } else {
                windows.get_primary().unwrap()
            };

            // check if the cursor is inside the window and get its position
            if let Some(screen_pos) = wnd.cursor_position() {
                // get the size of the window
                let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                // matrix for undoing the projection and camera transform
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix.inverse();

                // use it to convert ndc to world-space coordinates
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                // reduce it to a 2D value
                let world_pos: Vec2 = world_pos.truncate();

                eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

                for (agent, transform, sprite, entity) in agent_query.iter() {
                    // println!("{:?}", agent)
                    let agent: &Agent = agent;
                    let transform: &Transform = transform;
                    let _sprite: &Sprite = sprite;

                    println!("{:?}", transform);

                    let scale_x: f32 = 300.0;
                    let scale_y: f32 = 300.0;
                    let pos_x: f32 = transform.translation.x;
                    let pos_y: f32 = transform.translation.y;

                    // if the cursor is within the bounds of the agent then print the agent name
                    if world_pos.x >= pos_x - scale_x / 2.0
                        && world_pos.x <= pos_x + scale_x / 2.0
                        && world_pos.y >= pos_y - scale_y / 2.0
                        && world_pos.y <= pos_y + scale_y / 2.0
                    {
                        if !ui_states.agents.contains(&entity){
                            ui_states.deref_mut().agents.push(entity);
                        }
                        println!("{}", agent.name);
                    }
                }
            }
        }
    }
}
