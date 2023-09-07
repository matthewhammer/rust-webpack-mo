use wasm_bindgen::prelude::*;
use web_sys::Document;

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
pub struct DocumentValue {
    pub document: Document,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum DocumentMethod {
    GetElementById,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DocumentMethodValue {
    pub document: DocumentValue,
    pub method: DocumentMethod,
}

impl Hash for DocumentValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Document values, please");
    }
}

impl Dynamic for DocumentValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "getElementById" {
            Ok(DocumentMethodValue {
                document: self.clone(),
                method: DocumentMethod::GetElementById,
            }
            .into_value()
            .into())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}

impl Dynamic for DocumentMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        match self.method {
            DocumentMethod::GetElementById => todo!(),
            _ => type_mismatch!(file!(), line!()),
        }
    }
}
