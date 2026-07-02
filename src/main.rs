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
use bevy::input_focus::{FocusCause, InputFocus, InputFocusVisible, IsFocused, IsFocusedHelper};
use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy::text::{EditableText, EditableTextFilter, TextEdit, TextEditChange};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
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

#[derive(Component)]
struct InitialFocus;

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
                composite_alpha_mode: std::cfg_select! {
                    target_os = "macos" => CompositeAlphaMode::PostMultiplied,
                    target_os = "linux" => CompositeAlphaMode::PreMultiplied,
                    _ => CompositeAlphaMode::Auto,
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(Self::window_plugin()));
        app.add_plugins(DirectionalNavigationPlugin);
        app.insert_resource(ClearColor(Color::NONE));
        app.add_systems(Startup, (app_setup, calc_setup));
        app.add_systems(Update, (keyboard_input, button_state, buttons_state));
        app.add_observer(sync_display_to_operand);
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
                        EditableText::new("0"),
                        TextColor::WHITE,
                        TextLayout::justify(Justify::Center),
                        EditableTextFilter::new(is_calc_char),
                        Node {
                            width: Val::Percent(90.),
                            ..Default::default()
                        },
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
    let mut entity = builder.spawn((
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
            TextLayout::justify(Justify::Center),
            TextShadow::default(),
        )],
    ));
    if row == 0 && col == 0 {
        entity.insert(InitialFocus);
    }
    entity.observe(on_button_click);
}

/// Filter callback for the calculator display, allowing only valid input characters.
const fn is_calc_char(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == '-'
}

/// Replace the editable text and move the cursor to the end.
fn reset_editable(editable: &mut EditableText, text: &str) {
    editable.editor_mut().set_text(text);
    editable.queue_edit(TextEdit::TextEnd(false));
}

/// Process a button action (digit, operator, etc.) and update the display and operation state
fn process_button_action(
    button: &str,
    editable: &mut EditableText,
    op_metadata: &mut OperationMetadata,
) -> Result {
    match button {
        // Digit buttons
        ZERO_BUTTON | ONE_BUTTON | TWO_BUTTON | THREE_BUTTON | FOUR_BUTTON | FIVE_BUTTON
        | SIX_BUTTON | SEVEN_BUTTON | EIGHT_BUTTON | NINE_BUTTON => {
            let current = editable.value();
            if current == "0" {
                reset_editable(editable, button);
            } else {
                editable.queue_edit(TextEdit::Insert(button.into()));
            }
        }

        // Operator buttons
        CLEAR_BUTTON => {
            reset_editable(editable, "0");
            op_metadata.reset();
        }
        INVERT_BUTTON => {
            let current = editable.value().to_string();
            let negated = (!current.is_empty() && current != "0").then(|| format!("-{current}"));
            let new_text = current
                .strip_prefix('-')
                .map(str::to_string)
                .or(negated)
                .unwrap_or(current);
            reset_editable(editable, &new_text);
        }
        POURCENT_BUTTON => {
            let current = editable.value().to_string();
            let display_value = current.parse::<f64>()?;
            let result_value = display_value / 100.0;

            info!("Calculating: {display_value} % = {result_value}");

            reset_editable(editable, &result_value.to_string());
            op_metadata.reset();
        }
        ADD_BUTTON | SUB_BUTTON | MULTIPLY_BUTTON | DIVIDE_BUTTON => {
            let current = editable.value().to_string();

            // Handle the case the user clicks on an operator before clicking on number buttons
            if op_metadata.left_operand().is_none() {
                op_metadata.set_left_operand(&current)?;
            }

            let operator = match button {
                ADD_BUTTON => CalcOperator::Add,
                SUB_BUTTON => CalcOperator::Sub,
                MULTIPLY_BUTTON => CalcOperator::Mul,
                DIVIDE_BUTTON => CalcOperator::Div,
                _ => unreachable!(),
            };
            op_metadata.set_operator(operator);

            // Clear the display for the next operand. The TextEditChange observer
            // will sync the cleared value into the appropriate operand.
            reset_editable(editable, "0");
        }
        DOT_BUTTON => {
            let current = editable.value().to_string();
            if !current.contains('.') {
                editable.queue_edit(TextEdit::Insert(".".into()));
            }
        }
        EQUAL_BUTTON if op_metadata.is_under_operation() => {
            let result_value = op_metadata.calculate()?;
            reset_editable(editable, &result_value.to_string());
            op_metadata.reset();
        }

        _ => {}
    }

    Ok(())
}

/// Sync the current operation operand with the editable display value.
fn sync_display_to_operand(
    _change: On<TextEditChange>,
    mut display: Single<(&EditableText, &mut OperationMetadata)>,
) {
    let (editable, op_metadata) = &mut *display;
    let value = editable.value().to_string();
    let _ = op_metadata.set_operand(&value);
}

/// Handle keyboard input for calculator navigation and the Enter-to-activate shortcut.
///
/// Character entry is handled by the focused [`EditableText`] widget via the
/// `EditableTextInputPlugin` which is part of `DefaultPlugins`.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    logical_keys: Res<ButtonInput<Key>>,
    mut operation_query: Query<
        (&mut EditableText, &mut OperationMetadata),
        With<OperationMetadata>,
    >,
    mut input_focus_visible: ResMut<InputFocusVisible>,
    button_query: Query<(Entity, &Children), With<CalcButton>>,
    initial_focus_query: Query<Entity, (With<CalcButton>, With<InitialFocus>)>,
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
            let _ = auto_nav.navigate(direction);
        } else if let Ok(entity) = initial_focus_query.single() {
            auto_nav
                .manual_directional_navigation
                .focus
                .set(entity, FocusCause::Navigated);
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

            let (mut editable, mut op_metadata) = operation_query.single_mut()?;
            process_button_action(button_text.0.as_str(), &mut editable, &mut op_metadata)?;
        } else {
            // No focused button, Enter triggers EQUAL
            debug!("Key pressed: Enter -> button: {}", EQUAL_BUTTON);

            let (mut editable, mut op_metadata) = operation_query.single_mut()?;
            process_button_action(EQUAL_BUTTON, &mut editable, &mut op_metadata)?;
        }
        return Ok(());
    }

    Ok(())
}

/// Handle a click on a calculator button and apply the corresponding action.
#[allow(clippy::needless_pass_by_value)]
fn on_button_click(
    click: On<Pointer<Click>>,
    mut input_focus: ResMut<InputFocus>,
    mut input_focus_visible: ResMut<InputFocusVisible>,
    children_query: Query<&Children>,
    text_query: Query<&Text, Without<OperationMetadata>>,
    mut operation_query: Query<
        (&mut EditableText, &mut OperationMetadata),
        With<OperationMetadata>,
    >,
) -> Result {
    let entity = click.entity;
    input_focus_visible.0 = false;
    input_focus.set(entity, FocusCause::Navigated);

    let children = children_query.get(entity)?;
    let button_text = text_query.get(children[0])?;

    debug!("Clicking on button: {}", button_text.0.as_str());

    let (mut editable, mut op_metadata) = operation_query.single_mut()?;
    process_button_action(button_text.0.as_str(), &mut editable, &mut op_metadata)?;

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
    input_focus_visible: Res<InputFocusVisible>,
) -> Result {
    let show_hover = !input_focus_visible.0;

    for (entity, interaction, mut bg_color, mut border_color, children) in &mut buttons {
        let button_text = texts_query.get(children[0])?;
        let op_metadata = operation_query.single()?;

        let is_focused = focus_helper.is_focus_visible(entity);

        if op_metadata.is_under_operation()
            && let Some(operator) = op_metadata.operator()
        {
            let button: ButtonVariant = operator.into();

            if button_text.0.as_str() == button {
                *border_color = BorderColor::all(Color::WHITE);
            } else if is_focused {
                *bg_color = FOCUSED_BUTTON.into();
                *border_color = BorderColor::all(Color::srgb(0.3, 0.5, 1.0));
            } else if show_hover && *interaction == Interaction::Hovered {
                *bg_color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
            } else {
                *bg_color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        } else if is_focused {
            *bg_color = FOCUSED_BUTTON.into();
            *border_color = BorderColor::all(Color::srgb(0.3, 0.5, 1.0));
        } else if show_hover && *interaction == Interaction::Hovered {
            *bg_color = HOVERED_BUTTON.into();
            *border_color = BorderColor::all(Color::WHITE);
        } else {
            *bg_color = NORMAL_BUTTON.into();
            *border_color = BorderColor::all(Color::BLACK);
        }
    }

    Ok(())
}

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
