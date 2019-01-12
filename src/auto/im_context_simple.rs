// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use IMContext;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct IMContextSimple(Object<ffi::GtkIMContextSimple, ffi::GtkIMContextSimpleClass>): IMContext;

    match fn {
        get_type => || ffi::gtk_im_context_simple_get_type(),
    }
}

impl IMContextSimple {
    pub fn new() -> IMContextSimple {
        assert_initialized_main_thread!();
        unsafe {
            IMContext::from_glib_full(ffi::gtk_im_context_simple_new()).downcast_unchecked()
        }
    }
}

impl Default for IMContextSimple {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for IMContextSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IMContextSimple")
    }
}
