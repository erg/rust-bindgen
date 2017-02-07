/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub m_baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 4usize , concat ! (
               "Size of: " , stringify ! ( Bar ) ));
    assert_eq! (::std::mem::align_of::<Bar>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( Bar ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Bar ) ) . m_baz as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( Bar ) , "::" ,
                stringify ! ( m_baz ) ));
}
extern "C" {
    #[link_name = "_ZN3BarC1Ei"]
    pub fn Bar_Bar(this: *mut Bar, baz: ::std::os::raw::c_int);
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
impl Bar {
    #[inline]
    pub unsafe fn new(baz: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Bar_Bar(&mut __bindgen_tmp, baz);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Baz {
    pub _address: u8,
}
extern "C" {
    #[link_name = "_ZN3Baz3FOOE"]
    pub static mut Baz_FOO: [Bar; 0usize];
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(::std::mem::size_of::<Baz>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Baz ) ));
    assert_eq! (::std::mem::align_of::<Baz>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Baz ) ));
}
impl Clone for Baz {
    fn clone(&self) -> Self { *self }
}
