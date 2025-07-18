/*
 *
 * Copyright (c) Jérémy Audiger.
 * All rights reserved.
 *
 */

use bevy::prelude::*;
use std::fmt;

use crate::button::{ADD_BUTTON, ButtonVariant, DIVIDE_BUTTON, MULTIPLY_BUTTON, SUB_BUTTON};

/// All possible operators for the calculator.
#[derive(Copy, Clone)]
pub enum CalcOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<CalcOperator> for ButtonVariant {
    fn from(val: CalcOperator) -> Self {
        match val {
            CalcOperator::Add => ADD_BUTTON,
            CalcOperator::Sub => SUB_BUTTON,
            CalcOperator::Mul => MULTIPLY_BUTTON,
            CalcOperator::Div => DIVIDE_BUTTON,
        }
    }
}

impl fmt::Display for CalcOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

/// Represents and manage the ongoing operation.
#[derive(Default, Component)]
pub struct OperationMetadata {
    left_operand: Option<f64>,
    right_operand: Option<f64>,
    operator: Option<CalcOperator>,
}

impl OperationMetadata {
    pub fn set_operand(&mut self, operand: &str) -> Result {
        if self.is_under_operation() {
            self.set_right_operand(operand)
        } else {
            self.set_left_operand(operand)
        }
    }

    fn set_left_operand(&mut self, left_operand: &str) -> Result {
        self.left_operand = Some(left_operand.parse::<f64>()?);

        Ok(())
    }

    fn set_right_operand(&mut self, right_operand: &str) -> Result {
        self.right_operand = Some(right_operand.parse::<f64>()?);

        Ok(())
    }

    pub const fn set_operator(&mut self, operator: CalcOperator) {
        self.operator = Some(operator);
    }

    pub const fn operator(&self) -> Option<CalcOperator> {
        self.operator
    }

    pub fn calculate(&self) -> Result<f64> {
        let left_operand = self.left_operand.ok_or("Left operand not found")?;
        let right_operand = self.right_operand.ok_or("Right operand not found")?;
        let operator = self.operator.ok_or("Operator not found")?;

        let result = match operator {
            CalcOperator::Add => left_operand + right_operand,
            CalcOperator::Sub => left_operand - right_operand,
            CalcOperator::Mul => left_operand * right_operand,
            CalcOperator::Div => left_operand / right_operand,
        };

        info!(
            "Calculating: {} {} {} = {}",
            left_operand, operator, right_operand, result
        );

        Ok(result)
    }

    pub const fn is_new_operand(&self) -> bool {
        if self.is_under_operation() {
            self.right_operand.is_none()
        } else {
            self.left_operand.is_none()
        }
    }

    pub const fn is_under_operation(&self) -> bool {
        self.operator.is_some()
    }

    pub const fn reset(&mut self) {
        self.left_operand = None;
        self.right_operand = None;
        self.operator = None;
    }
}
