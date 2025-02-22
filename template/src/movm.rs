use motoko::{
    ast::{Exp, NodeData},
    value::Value_,
    vm_types::{Core, Result},
    Share,
};
use std::cell::RefCell;

// Define a global mutable cell for the_core using RefCell and Rc
thread_local! {
    static THE_CORE: RefCell<Core> = RefCell::new(Core::empty());
}

// Accessor to get the value of the_core (read-only)
pub fn get() -> Core {
    THE_CORE.with(|c| c.borrow().clone())
}

// Accessor to read and update the value of the_core (mutably)
pub fn update<F, R>(update_fn: F) -> R
where
    F: FnOnce(&mut Core) -> R,
{
    let mut r: Option<R> = None;
    THE_CORE.with(|c| {
        let mut core = c.borrow_mut();
        r = Some(update_fn(&mut core));
    });
    r.unwrap()
}

pub fn call(f: Value_, v: Value_) -> Result {
    update(|core| {
        core.eval_exp(
            NodeData::eval(Exp::Call(
                NodeData::eval(Exp::Value_(f)).share(),
                None,
                NodeData::eval(Exp::Value_(v)).share(),
            ))
            .share(),
        )
    })
}
