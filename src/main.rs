/*
 *
 * Copyright (c) Jérémy Audiger.
 * All rights reserved.
 *
 */

mod button;
mod operation;

use bevy::prelude::*;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use bevy::window::CompositeAlphaMode;
use bevy::window::WindowResolution;

use button::{
    ADD_BUTTON, ButtonVariant, CLEAR_BUTTON, CalcButton, DIVIDE_BUTTON, DOT_BUTTON, EIGHT_BUTTON,
    EQUAL_BUTTON, FIVE_BUTTON, FOUR_BUTTON, HOVERED_BUTTON, INVERT_BUTTON, MULTIPLY_BUTTON,
    NINE_BUTTON, NORMAL_BUTTON, ONE_BUTTON, POURCENT_BUTTON, PRESSED_BUTTON, SEVEN_BUTTON,
    SIX_BUTTON, SUB_BUTTON, THREE_BUTTON, TWO_BUTTON, ZERO_BUTTON,
};
use operation::{CalcOperator, OperationMetadata};

struct AppPlugin;

impl AppPlugin {
    fn window_plugin() -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Calculator".to_string(),
                resolution: WindowResolution::new(330., 315.),
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
                            margin: UiRect::all(Val::Percent(1.)),
                            justify_content: JustifyContent::Center, // Horizontal
                            align_items: AlignItems::Center,         // Vertical
                            ..Default::default()
                        },
                        BorderColor(Color::BLACK),
                        BorderRadius::MAX,
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
            Node {
                width: Val::Px(80.),
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(2.)),
                margin: UiRect::all(Val::Percent(1.)),
                display: Display::Grid,
                grid_row: GridPlacement::start_end(row as i16 + 2, row as i16 + 3), // Offset by 1 for the result value row
                grid_column: GridPlacement::start_end(col as i16 + 1, col as i16 + 2),
                justify_content: JustifyContent::Center, // Horizontal
                align_items: AlignItems::Center,         // Vertical
                ..Default::default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ),
        children![(
            Text::new(button),
            TextColor::WHITE,
            TextLayout::new_with_justify(JustifyText::Center),
            TextShadow::default(),
        )],
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

#[allow(
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::type_complexity
)]
fn button_input(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<CalcButton>),
    >,
    text_query: Query<&Text, Without<OperationMetadata>>,
    mut operation_query: Query<(&mut Text, &mut OperationMetadata), With<OperationMetadata>>,
) -> Result {
    for (interaction, children) in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            let button_text = text_query.get(children[0])?;

            debug!("Clicking on button: {}", button_text.0.as_str());

            let (mut op_result, mut op_metadata) = operation_query.single_mut()?;

            match button_text.0.as_str() {
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
                    op_metadata.set_operator(CalcOperator::Div);
                }
                MULTIPLY_BUTTON => {
                    op_metadata.set_operator(CalcOperator::Mul);
                }
                SUB_BUTTON => {
                    op_metadata.set_operator(CalcOperator::Sub);
                }
                ADD_BUTTON => {
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
        }
    }

    Ok(())
}

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
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // Prevent the current operator button to be unhighlighted
                if let Some(operator) = op_metadata.operator() {
                    let button_variant: ButtonVariant = operator.into();

                    if button_variant != button_text.0.as_str() {
                        *bg_color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                    }
                } else {
                    *bg_color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }

    Ok(())
}

fn buttons_state(
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<CalcButton>,
    >,
    texts_query: Query<&Text, Without<OperationMetadata>>,
    operation_query: Query<&OperationMetadata>,
) -> Result {
    for (interaction, mut bg_color, mut border_color, children) in &mut buttons {
        let button_text = texts_query.get(children[0])?;
        let op_metadata = operation_query.single()?;

        if op_metadata.is_under_operation()
            && let Some(operator) = op_metadata.operator()
        {
            let button: ButtonVariant = operator.into();

            if button_text.0.as_str() == button {
                border_color.0 = Color::WHITE;
            } else if *interaction != Interaction::Hovered {
                *bg_color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        } else if *interaction != Interaction::Hovered {
            *bg_color = NORMAL_BUTTON.into();
            border_color.0 = Color::BLACK;
        }
    }

    Ok(())
}

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
