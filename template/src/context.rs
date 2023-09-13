use motoko::vm_types::CoreSource;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

use motoko::{ast::Id, Interruption, Share, Value, Value_};

use std::hash::{Hash, Hasher};

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    vm_types::Store,
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextValue {
    pub context: CanvasRenderingContext2d,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageDataValue {
    pub image_data: ImageData,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ContextMethod {
    BeginPath,
    Arc,
    Stroke,
    StrokeRect,
    ClearRect,
    SetFillStyle,
    FillRect,
    GetImageData,
    PutImageData,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContextMethodValue {
    pub context: ContextValue,
    pub method: ContextMethod,
}

impl Hash for ContextValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Context values, please");
    }
}

impl Dynamic for ContextValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if false {
            web_sys::console::log_1(&JsValue::from_str(
                format!("ContextValue::get_field {}", name).as_str(),
            ));
        }
        let method = match name {
            "beginPath" => ContextMethod::BeginPath,
            "arc" => ContextMethod::Arc,
            "stroke" => ContextMethod::Stroke,
            "setFillStyle" => ContextMethod::SetFillStyle,
            "fillRect" => ContextMethod::FillRect,
            "strokeRect" => ContextMethod::StrokeRect,
            "clearRect" => ContextMethod::ClearRect,
            "getImageData" => ContextMethod::GetImageData,
            "putImageData" => ContextMethod::PutImageData,
            _ => return Err(Interruption::UnboundIdentifer(Id::new(name.to_string()))),
        };
        Ok(ContextMethodValue {
            context: self.clone(),
            method,
        }
        .into_value()
        .into())
    }
}

impl Dynamic for ImageDataValue {}

impl Hash for ImageDataValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Image Data values, please");
    }
}

impl Dynamic for ContextMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        if false {
            web_sys::console::log_1(&JsValue::from_str(
                format!("ContextMethodValue::call {:?} {:?}", &self.method, &args).as_str(),
            ));
        }
        match self.method {
            ContextMethod::BeginPath => {
                drop(motoko::vm::match_tuple(0, args)?);
                self.context.context.begin_path();
                if false {
                    web_sys::console::log_1(&JsValue::from_str(
                        format!("ContextMethodValue::call BeginPath () ==> Ok").as_str(),
                    ));
                }
                Ok(Value::Unit.share())
            }
            ContextMethod::GetImageData => {
                let tup = motoko::vm::match_tuple(4, args)?;
                let x = motoko::vm::assert_value_is_f64(&tup[0])?;
                let y = motoko::vm::assert_value_is_f64(&tup[1])?;
                let x2 = motoko::vm::assert_value_is_f64(&tup[2])?;
                let y2 = motoko::vm::assert_value_is_f64(&tup[3])?;
                let image_data = self
                    .context
                    .context
                    .get_image_data(x, y, x2, y2)
                    .expect("get image data");
                Ok(ImageDataValue { image_data }.into_value().into())
            }
            ContextMethod::PutImageData => {
                let tup = motoko::vm::match_tuple(3, args)?;
                let i = todo!();
                let x = motoko::vm::assert_value_is_f64(&tup[1])?;
                let y = motoko::vm::assert_value_is_f64(&tup[2])?;
                self.context
                    .context
                    .put_image_data(i, x, y)
                    .expect("put image data");
                Ok(Value::Unit.into())
            }
            ContextMethod::Arc => {
                let tup = motoko::vm::match_tuple(5, args)?;
                let x = motoko::vm::assert_value_is_f64(&tup[0])?;
                let y = motoko::vm::assert_value_is_f64(&tup[1])?;
                let r = motoko::vm::assert_value_is_f64(&tup[2])?;
                let start = motoko::vm::assert_value_is_f64(&tup[3])?;
                let end = motoko::vm::assert_value_is_f64(&tup[4])?;
                self.context.context.arc(x, y, r, start, end).expect("arc");
                if false {
                    web_sys::console::log_1(&JsValue::from_str(
                        format!("ContextMethodValue::call arc (..) ==> Ok").as_str(),
                    ));
                }
                Ok(Value::Unit.share())
            }
            ContextMethod::SetFillStyle => {
                let s = motoko::vm::assert_value_is_string(&args)?;
                self.context.context.set_fill_style(&s.into());
                Ok(Value::Unit.share())
            }
            ContextMethod::FillRect => {
                let tup = motoko::vm::match_tuple(4, args)?;
                let x = motoko::vm::assert_value_is_f64(&tup[0])?;
                let y = motoko::vm::assert_value_is_f64(&tup[1])?;
                let x2 = motoko::vm::assert_value_is_f64(&tup[2])?;
                let y2 = motoko::vm::assert_value_is_f64(&tup[3])?;
                self.context.context.fill_rect(x, y, x2, y2);
                if false {
                    web_sys::console::log_1(&JsValue::from_str(
                        format!("ContextMethodValue::call fill rect (..) ==> Ok").as_str(),
                    ));
                }
                Ok(Value::Unit.share())
            }
            ContextMethod::Stroke => {
                drop(motoko::vm::match_tuple(0, args)?);
                self.context.context.stroke();
                if false {
                    web_sys::console::log_1(&JsValue::from_str(
                        format!("ContextMethodValue::call stroke (..) ==> Ok").as_str(),
                    ));
                }
                Ok(Value::Unit.share())
            }
            _ => todo!(),
        }
    }
}
