/*
 *
 * Copyright (c) Jérémy Audiger.
 * All rights reserved.
 *
 */

use bevy::prelude::Color;
use bevy::prelude::Component;

/// Type definition to define the buttons
pub type ButtonVariant = &'static str;

pub const CLEAR_BUTTON: ButtonVariant = "C";
pub const INVERT_BUTTON: ButtonVariant = "+/-";
pub const POURCENT_BUTTON: ButtonVariant = "%";
pub const DIVIDE_BUTTON: ButtonVariant = "/";
pub const MULTIPLY_BUTTON: ButtonVariant = "*";
pub const SUB_BUTTON: ButtonVariant = "-";
pub const ADD_BUTTON: ButtonVariant = "+";
pub const EQUAL_BUTTON: ButtonVariant = "=";
pub const DOT_BUTTON: ButtonVariant = ".";
pub const ZERO_BUTTON: ButtonVariant = "0";
pub const ONE_BUTTON: ButtonVariant = "1";
pub const TWO_BUTTON: ButtonVariant = "2";
pub const THREE_BUTTON: ButtonVariant = "3";
pub const FOUR_BUTTON: ButtonVariant = "4";
pub const FIVE_BUTTON: ButtonVariant = "5";
pub const SIX_BUTTON: ButtonVariant = "6";
pub const SEVEN_BUTTON: ButtonVariant = "7";
pub const EIGHT_BUTTON: ButtonVariant = "8";
pub const NINE_BUTTON: ButtonVariant = "9";

/// Type definition for define the buttons' states
pub type ButtonState = Color;

pub const NORMAL_BUTTON: ButtonState = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: ButtonState = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: ButtonState = Color::srgb(0.75, 0.75, 0.75);
pub const FOCUSED_BUTTON: ButtonState = Color::srgb(0.2, 0.4, 0.8);

/// Struct definition used for tagging buttons for Bevy query
#[derive(Component)]
pub struct CalcButton;
