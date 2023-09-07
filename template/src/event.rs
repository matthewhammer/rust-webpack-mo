use web_sys::{Event, KeyboardEvent, MouseEvent};

use motoko::vm_types::CoreSource;
use motoko::{ast::Id, Interruption, Share, Value};

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventValue {
    pub event: Event,
}

impl Hash for EventValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Event values, please");
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MouseEventValue {
    pub mouse_event: MouseEvent,
}

impl Hash for MouseEventValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash MouseEvent values, please");
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardEventValue {
    pub keyboard_event: KeyboardEvent,
}

impl Hash for KeyboardEventValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash KeyBoardEvent values, please");
    }
}

//#[macro_use]
use motoko::{
    dynamic::{Dynamic, Result},
    type_mismatch,
    vm_types::Store,
};

impl Dynamic for EventValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "type" {
            Ok(Value::Variant(Id::new(self.event.type_()), None).share())
        } else {
            type_mismatch!(file!(), line!())
        }
    }
}

impl Dynamic for MouseEventValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "type" {
            Ok(Value::Variant(Id::new(self.mouse_event.type_()), None).share())

        // to do -- clientX, clientY
        } else {
            type_mismatch!(file!(), line!())
        }
    }
}

impl Dynamic for KeyboardEventValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "type" {
            Ok(Value::Variant(Id::new(self.keyboard_event.type_()), None).share())
        } else if name == "key" {
            Ok(Value::Text(motoko::value::Text::String(Box::new(
                self.keyboard_event.key(),
            )))
            .share())

        // to do -- all getter methods here -- https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html#implementations
        } else {
            type_mismatch!(file!(), line!())
        }
    }
}
