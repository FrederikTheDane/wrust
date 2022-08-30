use std::mem;

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

pub trait ZeroSized : Sized {}

impl<C> ZeroSized for C where Assert<{ mem::size_of::<C>() == 0 }> : IsTrue, Assert<{ mem::size_of::<C>() == 0 }> : Sized {}