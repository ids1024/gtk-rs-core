// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Container;
use Scrollable;
use Widget;
use ffi;
use gdk;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Layout(Object<ffi::GtkLayout, ffi::GtkLayoutClass>): Container, Widget, Buildable, Scrollable;

    match fn {
        get_type => || ffi::gtk_layout_get_type(),
    }
}

impl Layout {
    pub fn new<'a, 'b, P: Into<Option<&'a Adjustment>>, Q: Into<Option<&'b Adjustment>>>(hadjustment: P, vadjustment: Q) -> Layout {
        assert_initialized_main_thread!();
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_layout_new(hadjustment.0, vadjustment.0)).downcast_unchecked()
        }
    }
}

pub trait LayoutExt: 'static {
    fn get_bin_window(&self) -> Option<gdk::Window>;

    fn get_size(&self) -> (u32, u32);

    fn move_<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32);

    fn put<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32);

    fn set_size(&self, width: u32, height: u32);

    fn get_property_height(&self) -> u32;

    fn set_property_height(&self, height: u32);

    fn get_property_width(&self) -> u32;

    fn set_property_width(&self, width: u32);

    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32);

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32);

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Layout>> LayoutExt for O {
    fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_bin_window(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_layout_get_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn move_<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(self.to_glib_none().0, child_widget.to_glib_none().0, x, y);
        }
    }

    fn put<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(self.to_glib_none().0, child_widget.to_glib_none().0, x, y);
        }
    }

    fn set_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(self.to_glib_none().0, width, height);
        }
    }

    fn get_property_height(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"height\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_height(&self, height: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"height\0".as_ptr() as *const _, Value::from(&height).to_glib_none().0);
        }
    }

    fn get_property_width(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"width\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_width(&self, width: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"width\0".as_ptr() as *const _, Value::from(&width).to_glib_none().0);
        }
    }

    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"x\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"x\0".as_ptr() as *const _, Value::from(&x).to_glib_none().0);
        }
    }

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"y\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"y\0".as_ptr() as *const _, Value::from(&y).to_glib_none().0);
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::height\0".as_ptr() as *const _,
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::width\0".as_ptr() as *const _,
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::GtkLayout, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Layout> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Layout::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::GtkLayout, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Layout> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Layout::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layout")
    }
}
