use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default)]
/// `Actions` is a struct that contains a single field, `camera_movement`, which is an `Option<Vec2>`.
///
/// The `Option` type is a built-in type that can either be `Some(T)` or `None`. In this case,
/// `camera_movement` is an `Option<Vec2>`, which means it can either be `Some(Vec2)` or `None`.
///
/// The `Vec2` type is a struct that has two fields: `x` and `y`.
///
/// Properties:
///
/// * `camera_movement`: This is a Vec2 that represents the movement of the camera.
pub struct Actions {
    pub camera_movement: Option<Vec2>,
}

/// If the player is pressing or releasing a movement key, set the camera movement to the direction the
/// player is pressing
///
/// Arguments:
///
/// * `actions`: ResMut<Actions> - This is the Actions resource that we created earlier.
/// * `keyboard_input`: Res<Input<KeyCode>>
fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Up.just_released(&keyboard_input)
        || GameControl::Up.pressed(&keyboard_input)
        || GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Down.just_released(&keyboard_input)
        || GameControl::Down.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input)
            || GameControl::Down.just_released(&keyboard_input)
        {
            if GameControl::Up.pressed(&keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.camera_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.camera_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.camera_movement = Some(player_movement);
        }
    } else {
        actions.camera_movement = None;
    }
}

/// Creating a new type called `GameControl` that can be one of four values: `Up`, `Down`, `Left`, or
/// `Right`.
enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    /// If the player just released the key associated with the given GameControl, return true
    ///
    /// Arguments:
    ///
    /// * `keyboard_input`: &Res<Input<KeyCode>>
    ///
    /// Returns:
    ///
    /// A boolean value.
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    /// If the player is pressing the W, A, S, or D keys, or the up, left, down, or right arrow keys,
    /// then return true
    ///
    /// Arguments:
    ///
    /// * `keyboard_input`: &Res<Input<KeyCode>>
    ///
    /// Returns:
    ///
    /// A boolean value.
    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
