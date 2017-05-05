/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsTArray<T> {
    pub mBuff: *mut T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for nsTArray<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsStyleAutoArray<T> {
    pub mFirstElement: T,
    pub mOtherElements: nsTArray<T>,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nsStyleAutoArray_WithSingleInitialElement {
    WITH_SINGLE_INITIAL_ELEMENT = 0,
}
impl <T> Default for nsStyleAutoArray<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}