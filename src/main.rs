/*
 *
 * Copyright (c) Jérémy Audiger.
 * All rights reserved.
 *
 */

mod button;
mod operation;

use bevy::input::keyboard::Key;
use bevy::input_focus::directional_navigation::DirectionalNavigationPlugin;
use bevy::input_focus::{InputDispatchPlugin, InputFocusVisible, IsFocused, IsFocusedHelper};
use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use bevy::window::CompositeAlphaMode;
use bevy::window::WindowResolution;

use button::{
    ADD_BUTTON, ButtonVariant, CLEAR_BUTTON, CalcButton, DIVIDE_BUTTON, DOT_BUTTON, EIGHT_BUTTON,
    EQUAL_BUTTON, FIVE_BUTTON, FOCUSED_BUTTON, FOUR_BUTTON, HOVERED_BUTTON, INVERT_BUTTON,
    MULTIPLY_BUTTON, NINE_BUTTON, NORMAL_BUTTON, ONE_BUTTON, POURCENT_BUTTON, PRESSED_BUTTON,
    SEVEN_BUTTON, SIX_BUTTON, SUB_BUTTON, THREE_BUTTON, TWO_BUTTON, ZERO_BUTTON,
};
use operation::{CalcOperator, OperationMetadata};

struct AppPlugin;

impl AppPlugin {
    fn window_plugin() -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Calculator".to_string(),
                resolution: WindowResolution::new(330, 315),
                resizable: false,
                transparent: true,
                decorations: false,
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: false,
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                #[cfg(target_os = "linux")]
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(Self::window_plugin()));
        app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
        app.insert_resource(ClearColor(Color::NONE));
        app.add_systems(Startup, (app_setup, calc_setup));
        app.add_systems(
            Update,
            (keyboard_input, button_input, button_state, buttons_state),
        );
    }
}

fn app_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn calc_setup(mut commands: Commands) {
    const N_COLS: u16 = 4;
    const N_ROWS: u16 = 6;

    commands
        .spawn((
            // Main grid
            Node {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                grid_template_columns: RepeatedGridTrack::auto(N_COLS),
                grid_template_rows: RepeatedGridTrack::auto(N_ROWS),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|builder| {
            // Result value row
            builder.spawn((
                Node {
                    display: Display::Grid,
                    grid_column: GridPlacement::span(4),
                    padding: UiRect::right(Val::Percent(3.)),
                    ..Default::default()
                },
                children![(
                    (
                        Node {
                            border: UiRect::all(Val::Px(2.)),
                            border_radius: BorderRadius::MAX,
                            margin: UiRect::all(Val::Percent(1.)),
                            justify_content: JustifyContent::Center, // Horizontal
                            align_items: AlignItems::Center,         // Vertical
                            ..Default::default()
                        },
                        BorderColor::all(Color::BLACK),
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ),
                    children![(
                        Text::new("0".to_string()),
                        TextColor::WHITE,
                        OperationMetadata::default(),
                    )],
                )],
            ));

            // Buttons
            let buttons = vec![
                // Row 1
                CLEAR_BUTTON,
                INVERT_BUTTON,
                POURCENT_BUTTON,
                DIVIDE_BUTTON,
                // Row 2
                SEVEN_BUTTON,
                EIGHT_BUTTON,
                NINE_BUTTON,
                MULTIPLY_BUTTON,
                // Row 3
                FOUR_BUTTON,
                FIVE_BUTTON,
                SIX_BUTTON,
                SUB_BUTTON,
                // Row 4
                ONE_BUTTON,
                TWO_BUTTON,
                THREE_BUTTON,
                ADD_BUTTON,
                // Row 5
                ZERO_BUTTON,
                DOT_BUTTON,
                EQUAL_BUTTON,
            ];

            for (index, button) in buttons.iter().enumerate() {
                let row = index / 4;
                let col = index % 4;

                create_button(builder, button, row, col);
            }
        });
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn create_button(builder: &mut ChildSpawnerCommands<'_>, button: &str, row: usize, col: usize) {
    builder.spawn((
        (
            Button,
            CalcButton,
            AutoDirectionalNavigation::default(),
            Node {
                width: Val::Px(80.),
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(2.)),
                border_radius: BorderRadius::MAX,
                margin: UiRect::all(Val::Percent(1.)),
                display: Display::Grid,
                grid_row: GridPlacement::start_end(row as i16 + 2, row as i16 + 3), // Offset by 1 for the result value row
                grid_column: GridPlacement::start_end(col as i16 + 1, col as i16 + 2),
                justify_content: JustifyContent::Center, // Horizontal
                align_items: AlignItems::Center,         // Vertical
                ..Default::default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(NORMAL_BUTTON),
        ),
        children![(
            Text::new(button),
            TextColor::WHITE,
            TextLayout::new_with_justify(Justify::Center),
            TextShadow::default(),
        )],
    ));
}

/// Process a button action (digit, operator, etc.) and update the display and operation state
#[allow(clippy::too_many_lines)]
fn process_button_action(
    button: &str,
    op_result: &mut Mut<'_, Text>,
    op_metadata: &mut Mut<'_, OperationMetadata>,
) -> Result {
    match button {
        // Number buttons
        ZERO_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "0".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        ONE_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "1".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        TWO_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "2".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        THREE_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "3".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        FOUR_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "4".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        FIVE_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "5".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        SIX_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "6".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        SEVEN_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "7".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        EIGHT_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "8".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        NINE_BUTTON if op_result.0 == "0" || op_metadata.is_new_operand() => {
            op_result.0 = "9".to_string();
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        ZERO_BUTTON => {
            op_result.0 = format!("{}0", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        ONE_BUTTON => {
            op_result.0 = format!("{}1", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        TWO_BUTTON => {
            op_result.0 = format!("{}2", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        THREE_BUTTON => {
            op_result.0 = format!("{}3", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        FOUR_BUTTON => {
            op_result.0 = format!("{}4", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        FIVE_BUTTON => {
            op_result.0 = format!("{}5", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        SIX_BUTTON => {
            op_result.0 = format!("{}6", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        SEVEN_BUTTON => {
            op_result.0 = format!("{}7", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        EIGHT_BUTTON => {
            op_result.0 = format!("{}8", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }
        NINE_BUTTON => {
            op_result.0 = format!("{}9", op_result.0);
            op_metadata.set_operand(op_result.0.as_str())?;
        }

        // Operator buttons
        CLEAR_BUTTON => {
            op_result.0 = "0".to_string();
            op_metadata.reset();
        }
        INVERT_BUTTON if op_result.0.starts_with('-') => {
            op_result.0 = op_result.0[1..].to_string();
        }
        INVERT_BUTTON if op_result.0 != "0" => {
            op_result.0 = format!("-{}", op_result.0);
        }
        POURCENT_BUTTON => {
            let display_value = op_result.0.parse::<f64>()?;
            let result_value = display_value / 100.0;

            info!("Calculating: {} {} = {}", display_value, "%", result_value);

            op_result.0 = result_value.to_string();
            op_metadata.reset();
        }
        DIVIDE_BUTTON => {
            // Handle the case the user clicks on an operator before clicking on number buttons
            if op_metadata.left_operand().is_none() {
                op_metadata.set_left_operand(op_result.0.as_str())?;
            }

            op_metadata.set_operator(CalcOperator::Div);
        }
        MULTIPLY_BUTTON => {
            // Handle the case the user clicks on an operator before clicking on number buttons
            if op_metadata.left_operand().is_none() {
                op_metadata.set_left_operand(op_result.0.as_str())?;
            }

            op_metadata.set_operator(CalcOperator::Mul);
        }
        SUB_BUTTON => {
            // Handle the case the user clicks on an operator before clicking on number buttons
            if op_metadata.left_operand().is_none() {
                op_metadata.set_left_operand(op_result.0.as_str())?;
            }

            op_metadata.set_operator(CalcOperator::Sub);
        }
        ADD_BUTTON => {
            // Handle the case the user clicks on an operator before clicking on number buttons
            if op_metadata.left_operand().is_none() {
                op_metadata.set_left_operand(op_result.0.as_str())?;
            }

            op_metadata.set_operator(CalcOperator::Add);
        }
        DOT_BUTTON if !op_result.0.contains('.') => {
            op_result.0 = format!("{}.", op_result.0);
        }
        EQUAL_BUTTON => {
            if op_metadata.is_under_operation() {
                let result_value = op_metadata.calculate()?;
                op_result.0 = result_value.to_string();

                op_metadata.reset();
            }
        }

        _ => {}
    }

    Ok(())
}

/// Map a logical key to a button label (keyboard layout independent)
fn key_to_button(key: &Key) -> Option<&'static str> {
    match key {
        Key::Character(c) => match c.as_str() {
            "0" => Some(ZERO_BUTTON),
            "1" => Some(ONE_BUTTON),
            "2" => Some(TWO_BUTTON),
            "3" => Some(THREE_BUTTON),
            "4" => Some(FOUR_BUTTON),
            "5" => Some(FIVE_BUTTON),
            "6" => Some(SIX_BUTTON),
            "7" => Some(SEVEN_BUTTON),
            "8" => Some(EIGHT_BUTTON),
            "9" => Some(NINE_BUTTON),
            "+" => Some(ADD_BUTTON),
            "-" => Some(SUB_BUTTON),
            "*" => Some(MULTIPLY_BUTTON),
            "/" => Some(DIVIDE_BUTTON),
            "=" => Some(EQUAL_BUTTON),
            "." => Some(DOT_BUTTON),
            "c" | "C" => Some(CLEAR_BUTTON),
            _ => None,
        },
        Key::Backspace | Key::Delete => Some(CLEAR_BUTTON),
        _ => None,
    }
}

/// Handle keyboard input for calculator operations and navigation
#[allow(clippy::needless_pass_by_value)]
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    logical_keys: Res<ButtonInput<Key>>,
    mut operation_query: Query<(&mut Text, &mut OperationMetadata), With<OperationMetadata>>,
    mut input_focus_visible: ResMut<InputFocusVisible>,
    button_query: Query<(Entity, &Children), With<CalcButton>>,
    text_query: Query<&Text, Without<OperationMetadata>>,
    mut auto_nav: bevy::ui::auto_directional_navigation::AutoDirectionalNavigator,
) -> Result {
    // On ESC press, clear focus indicator
    if keys.just_pressed(KeyCode::Escape) {
        input_focus_visible.0 = false;
    }

    // Handle arrow key navigation (physical keys)
    let nav_direction = if keys.just_pressed(KeyCode::ArrowUp) {
        Some(CompassOctant::North)
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        Some(CompassOctant::South)
    } else if keys.just_pressed(KeyCode::ArrowLeft) {
        Some(CompassOctant::West)
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        Some(CompassOctant::East)
    } else {
        None
    };

    if let Some(direction) = nav_direction {
        // Make focus visible when using keyboard navigation
        input_focus_visible.0 = true;

        // Check if current focus is a calculator button
        let current_focus = auto_nav.input_focus();
        let focus_is_button = current_focus.is_some_and(|e| button_query.get(e).is_ok());

        if focus_is_button {
            // Navigate in the specified direction
            let _ = auto_nav.navigate(direction);
        } else {
            // No button focused, set focus to first button
            if let Some((entity, _)) = button_query.iter().next() {
                auto_nav.manual_directional_navigation.focus.set(entity);
            }
        }
        return Ok(());
    }

    // Handle Enter to activate focused button, or trigger EQUAL if no button focused
    if logical_keys.just_pressed(Key::Enter) {
        // If a button is focused, activate it
        if let Some(focused_entity) = auto_nav.input_focus()
            && let Ok((_, children)) = button_query.get(focused_entity)
            && let Ok(button_text) = text_query.get(children[0])
        {
            debug!("Activating focused button: {}", button_text.0.as_str());

            let (mut op_result, mut op_metadata) = operation_query.single_mut()?;
            process_button_action(button_text.0.as_str(), &mut op_result, &mut op_metadata)?;
        } else {
            // No focused button, Enter triggers EQUAL
            debug!("Key pressed: Enter -> button: {}", EQUAL_BUTTON);

            let (mut op_result, mut op_metadata) = operation_query.single_mut()?;
            process_button_action(EQUAL_BUTTON, &mut op_result, &mut op_metadata)?;
        }
        return Ok(());
    }

    // Handle character input (logical keys - keyboard layout independent)
    for key in logical_keys.get_just_pressed() {
        if let Some(button) = key_to_button(key) {
            debug!("Key pressed: {:?} -> button: {}", key, button);

            let (mut op_result, mut op_metadata) = operation_query.single_mut()?;
            process_button_action(button, &mut op_result, &mut op_metadata)?;
            return Ok(());
        }
    }

    Ok(())
}

/// Handle the button input, and update the operation result
///
/// Depending on the button pressed, and the current operation state, perform the corresponding action.
#[allow(clippy::type_complexity)]
fn button_input(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<CalcButton>),
    >,
    text_query: Query<&Text, Without<OperationMetadata>>,
    mut operation_query: Query<(&mut Text, &mut OperationMetadata), With<OperationMetadata>>,
    mut input_focus_visible: ResMut<InputFocusVisible>,
) -> Result {
    for (interaction, children) in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            // Hide focus indicator when using mouse
            input_focus_visible.0 = false;

            let button_text = text_query.get(children[0])?;

            debug!("Clicking on button: {}", button_text.0.as_str());

            let (mut op_result, mut op_metadata) = operation_query.single_mut()?;
            process_button_action(button_text.0.as_str(), &mut op_result, &mut op_metadata)?;
        }
    }

    Ok(())
}

/// Handle the button state (background color, border color)
#[allow(clippy::type_complexity)]
fn button_state(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<CalcButton>),
    >,
    text_query: Query<&Text, Without<OperationMetadata>>,
    operation_query: Query<&OperationMetadata>,
) -> Result {
    for (interaction, mut bg_color, mut border_color, children) in &mut interaction_query {
        let button_text = text_query.get(children[0])?;

        debug!(
            "Interaction '{:?}' on button: {}",
            *interaction,
            button_text.0.as_str()
        );

        let op_metadata = operation_query.single()?;

        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
            }
            Interaction::None => {
                // Prevent the current operator button to be un-highlighted
                if let Some(operator) = op_metadata.operator() {
                    let button_variant: ButtonVariant = operator.into();

                    if button_variant != button_text.0.as_str() {
                        *bg_color = NORMAL_BUTTON.into();
                        *border_color = BorderColor::all(Color::BLACK);
                    }
                } else {
                    *bg_color = NORMAL_BUTTON.into();
                    *border_color = BorderColor::all(Color::BLACK);
                }
            }
        }
    }

    Ok(())
}

/// Handle all the buttons state (background color, border color), depending on the current operation state and focus
#[allow(clippy::needless_pass_by_value)]
fn buttons_state(
    mut buttons: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<CalcButton>,
    >,
    texts_query: Query<&Text, Without<OperationMetadata>>,
    operation_query: Query<&OperationMetadata>,
    focus_helper: IsFocusedHelper,
) -> Result {
    for (entity, interaction, mut bg_color, mut border_color, children) in &mut buttons {
        let button_text = texts_query.get(children[0])?;
        let op_metadata = operation_query.single()?;

        // Check if this button is focused (and focus should be visible)
        let is_focused = focus_helper.is_focus_visible(entity);

        if op_metadata.is_under_operation()
            && let Some(operator) = op_metadata.operator()
        {
            let button: ButtonVariant = operator.into();

            if button_text.0.as_str() == button {
                *border_color = BorderColor::all(Color::WHITE);
            } else if is_focused {
                // Show focus indicator with blue background
                *bg_color = FOCUSED_BUTTON.into();
                *border_color = BorderColor::all(Color::srgb(0.3, 0.5, 1.0));
            } else if *interaction != Interaction::Hovered {
                *bg_color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        } else if is_focused {
            // Show focus indicator with blue background
            *bg_color = FOCUSED_BUTTON.into();
            *border_color = BorderColor::all(Color::srgb(0.3, 0.5, 1.0));
        } else if *interaction != Interaction::Hovered {
            *bg_color = NORMAL_BUTTON.into();
            *border_color = BorderColor::all(Color::BLACK);
        }
    }

    Ok(())
}

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
