use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Passer<T> {
    pub item:   Option<Rc<RefCell<T>>>
}

impl<T> Passer<T> {
    pub fn new(thing: T) -> Self {
        Passer {
            item:   Some(Rc::new(RefCell::new(thing)))
        }
    }

    pub fn none() -> Self {
        Passer {
            item:   None
        }
    }

    pub fn pass_to(&mut self, pass_to: &mut Passer<T>) -> bool {
        if let Some(rc) = &self.item {
            pass_to.item = Some(Rc::clone(&rc));
            self.item = None;
            return true
        }
        false
    }

    pub fn return_val(&mut self, error: &str) -> Option<T> {
        if let Some(rc) = &self.item {
            let val = Rc::clone(rc);
            self.item = None;
            let val = Rc::try_unwrap(val);

            return Some(val.ok().expect(error).into_inner())
        }
        None
    }
}