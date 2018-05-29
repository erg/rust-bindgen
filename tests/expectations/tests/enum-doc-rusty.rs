/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(u32)]
#[doc = " Document enum"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum B {
    #[doc = " Document field with three slashes"]
    VAR_A = 0,
    #[doc = " Document field with preceeding star"]
    VAR_B = 1,
    #[doc = " Document field with preceeding exclamation"]
    VAR_C = 2,
    #[doc = "< Document field with following star"]
    VAR_D = 3,
    #[doc = "< Document field with following exclamation"]
    VAR_E = 4,
    #[doc = " Document field with preceeding star, with a loong long multiline"]
    #[doc = " comment."]
    #[doc = ""]
    #[doc = " Very interesting documentation, definitely."]
    VAR_F = 5,
}
