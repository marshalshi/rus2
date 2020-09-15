use std::ops::{Deref, DerefMut};

pub trait Handle<T> {
    type Item: Deref<Target = T>;
    type ItemMut: DerefMut<Target = T>;

    fn get(self) -> Self::Item;
    fn get_mut(self) -> Self::ItemMut;
}
