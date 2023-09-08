use web_sys::Gamepad;
use web_sys::GamepadButton;

use motoko::{ast::Id, value::Value, Interruption, Share};

use std::hash::{Hash, Hasher};

//#[macro_use]
use motoko::{
    dynamic::{Dynamic, Result},
    //    type_mismatch,
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GamepadValue {
    pub gamepad: Gamepad,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GamepadButtonValue {
    pub gamepad_button: GamepadButton,
}

impl Hash for GamepadValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Gamepad values, please");
    }
}

impl Hash for GamepadButtonValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Gamepad values, please");
    }
}

impl Dynamic for GamepadValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "buttons" {
            let mut res = im_rc::Vector::new();
            let buttons = self.gamepad.buttons();
            for button in buttons.iter() {
                res.push_back(
                    crate::gamepad::GamepadButtonValue {
                        gamepad_button: button.into(),
                    }
                    .into_value()
                    .into(),
                )
            }
            Ok(motoko::value::Value::Array(motoko::ast::Mut::Const, res).into())
        } else if name == "axes" {
            let mut res = im_rc::Vector::new();
            let axes = self.gamepad.axes();
            for axis in axes.iter() {
                res.push_back(Value::Float(axis.unchecked_into_f64().into()).share())
            }
            Ok(motoko::value::Value::Array(motoko::ast::Mut::Const, res).into())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}

impl Dynamic for GamepadButtonValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "pressed" {
            Ok(Value::Bool(self.gamepad_button.pressed()).share())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}
