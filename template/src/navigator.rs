//use wasm_bindgen::prelude::*;

use web_sys::Navigator;

//use motoko::vm_types::CoreSource;
use motoko::{ast::Id, Interruption, Value_};

use std::hash::{Hash, Hasher};

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    //    type_mismatch,
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigatorValue {
    pub navigator: Navigator,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum NavigatorMethod {
    GetGamepads,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NavigatorMethodValue {
    pub navigator: NavigatorValue,
    pub method: NavigatorMethod,
}

impl Hash for NavigatorValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Navigator values, please");
    }
}

impl Dynamic for NavigatorValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "getGamepads" {
            Ok(NavigatorMethodValue {
                navigator: self.clone(),
                method: NavigatorMethod::GetGamepads,
            }
            .into_value()
            .into())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}

impl Dynamic for NavigatorMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, _args: Value_) -> Result {
        match self.method {
            NavigatorMethod::GetGamepads => {
                let mut res = im_rc::Vector::new();
                let gamepads = self.navigator.navigator.get_gamepads().expect("get_gamepads");
                for gamepad in gamepads.iter() {
                    res.push_back(
                        crate::gamepad::GamepadValue {
                            gamepad: gamepad.into(),
                        }
                        .into_value()
                        .into(),
                    )
                }
                Ok(motoko::value::Value::Array(motoko::ast::Mut::Const, res).into())
            }
        }
    }
}
