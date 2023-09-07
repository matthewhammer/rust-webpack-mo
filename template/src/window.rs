use wasm_bindgen::prelude::*;
use web_sys::Window;

use motoko::vm_types::CoreSource;
use motoko::{ast::Id, Interruption, Share, Value, Value_};

use std::hash::{Hash, Hasher};

use crate::context::ContextValue;

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    type_mismatch,
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowValue {
    pub window: Window,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum WindowMethod {
    AddEventListener,
    Document,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WindowMethodValue {
    pub window: WindowValue,
    pub method: WindowMethod,
}

impl Hash for WindowValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Window values, please");
    }
}

impl Dynamic for WindowValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "document" {
            match self.window.document() {
                None => Ok(Value::Null.share()),
                Some(document) => Ok(Value::Option(
                    crate::document::DocumentValue { document }
                        .into_value()
                        .share(),
                )
                .share()),
            }
        } else if name == "addEventListener" {
            Ok(WindowMethodValue {
                window: self.clone(),
                method: WindowMethod::AddEventListener,
            }
            .into_value()
            .into())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}

impl Dynamic for WindowMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        match self.method {
            WindowMethod::AddEventListener => {
                // to do -- look at arg to case analyze the event type.
                // handle each of: {key{Down, Up, Press}, mouse{Down, Up, Press}}
                todo!()
            }
            _ => type_mismatch!(file!(), line!()),
        }
    }
}
