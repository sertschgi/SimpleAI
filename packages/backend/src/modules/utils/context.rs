use derive_new::new;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
// -------------------- STRONG CONTEXT -------------------- //
#[derive(new, Clone, Default)]
pub struct StrongContext<T> {
    pub context: Arc<Mutex<T>>,
}

impl<T> StrongContext<T> {
    pub fn downgrade(self) -> WeakContext<T> {
        WeakContext::from(self)
    }
    pub fn set(&mut self, context: Self) {
        self.context = context.context;
    }
}

impl<T> PartialEq for StrongContext<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        //  TODO: try to make this reliable (with blocking_lock -> remove the error you'll get)
        let self_data = self.context.try_lock();
        let other_data = other.context.try_lock();
        if self_data.is_err() || other_data.is_err() {
            return true;
        }
        *self_data.unwrap() == *other_data.unwrap()
    }
}

impl<T> From<T> for StrongContext<T> {
    fn from(t: T) -> Self {
        Self::new(Arc::new(Mutex::new(t)))
    }
}

impl<T> From<WeakContext<T>> for Option<StrongContext<T>> {
    fn from(weak: WeakContext<T>) -> Option<StrongContext<T>> {
        if let Some(c) = weak.context.upgrade() {
            return Some(StrongContext::new(c));
        }
        None
    }
}

// -------------------- WEAK CONTEXT -------------------- //
#[derive(Clone, Default)]
pub struct WeakContext<T> {
    pub context: Weak<Mutex<T>>,
}
impl<T> WeakContext<T> {
    pub fn new() -> Self {
        Self::from(Weak::new())
    }
    pub fn upgrade(self) -> Option<StrongContext<T>> {
        Option::<StrongContext<T>>::from(self)
    }
}
impl<T> PartialEq for WeakContext<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if let Some(self_ctx) = self.clone().upgrade() {
            if let Some(other_ctx) = other.clone().upgrade() {
                return self_ctx == other_ctx;
            }
        }
        false
    }
}
impl<T> From<Weak<Mutex<T>>> for WeakContext<T> {
    fn from(weak: Weak<Mutex<T>>) -> Self {
        Self { context: weak }
    }
}
impl<T> From<StrongContext<T>> for WeakContext<T> {
    fn from(strong: StrongContext<T>) -> Self {
        Self::from(Arc::downgrade(&strong.context))
    }
}
