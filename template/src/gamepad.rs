use web_sys::Gamepad;
use web_sys::GamepadButton;

use motoko::{ast::Id, Interruption};

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
        // buttons -- array of button objects
        // axes -- array of floats
        Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
    }
}

impl Dynamic for GamepadButtonValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        // pressed as a boolean field
        Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
    }
}
