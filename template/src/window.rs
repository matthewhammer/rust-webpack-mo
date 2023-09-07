use wasm_bindgen::prelude::*;

use web_sys::Window;

//use motoko::vm_types::CoreSource;
use motoko::{ast::Id, shared::FastClone, Interruption, Share, Value, Value_};

use std::hash::{Hash, Hasher};

use crate::event::{KeyboardEventValue, MouseEventValue};

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    //    type_mismatch,
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowValue {
    pub window: Window,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum WindowMethod {
    AddEventListener,
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
        } else if name == "navigator" {
            Ok(crate::navigator::NavigatorValue {
                navigator: self.window.navigator(),
            }
            .into_value()
            .share())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}

impl Dynamic for WindowMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        match self.method {
            WindowMethod::AddEventListener => {
                let tup = motoko::vm::match_tuple(2, args)?;
                let typ = motoko::vm::assert_value_is_string(&tup[0])?;
                match typ.as_str() {
                    "click" | "mousedown" | "mouseup" | "mouseenter" | "mouseleave"
                    | "mouseover" | "mouseout" => {
                        let f = tup[1].fast_clone();
                        let cl = Closure::<dyn FnMut(_)>::new(
                            move |mouse_event: web_sys::MouseEvent| {
                                crate::movm::call(
                                    f.fast_clone(),
                                    MouseEventValue { mouse_event }.into_value().share(),
                                )
                                .expect("movm::call, window element, mouse event handler.");
                            },
                        );
                        self.window
                            .window
                            .add_event_listener_with_callback(
                                typ.as_str(),
                                cl.as_ref().unchecked_ref(),
                            )
                            .expect("add_event_listener_with_callback");
                        cl.forget(); // to do -- fix potential memory leak here. -- https://stackoverflow.com/a/63641967
                        Ok(Value::Unit.share())
                    }
                    "keydown" | "keyup" | "keypress" => {
                        let f = tup[1].fast_clone();
                        let cl = Closure::<dyn FnMut(_)>::new(
                            move |keyboard_event: web_sys::KeyboardEvent| {
                                crate::movm::call(
                                    f.fast_clone(),
                                    KeyboardEventValue { keyboard_event }.into_value().share(),
                                )
                                .expect("movm::call, window element, keyboard event handler.");
                            },
                        );
                        self.window
                            .window
                            .add_event_listener_with_callback(
                                typ.as_str(),
                                cl.as_ref().unchecked_ref(),
                            )
                            .expect("add_event_listener_with_callback");
                        cl.forget(); // to do -- fix potential memory leak here. -- https://stackoverflow.com/a/63641967
                        Ok(Value::Unit.share())
                    }
                    _ => todo!(),
                }
            } /* _ => type_mismatch!(file!(), line!()), */
        }
    }
}
