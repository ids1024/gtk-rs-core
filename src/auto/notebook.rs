// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use DirectionType;
use NotebookTab;
use PackType;
use PositionType;
use Widget;
use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Notebook(Object<ffi::GtkNotebook, ffi::GtkNotebookClass>): Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_notebook_get_type(),
    }
}

impl Notebook {
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_notebook_new()).downcast_unchecked()
        }
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

pub trait NotebookExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn detach_tab<P: IsA<Widget>>(&self, child: &P);

    fn get_action_widget(&self, pack_type: PackType) -> Option<Widget>;

    fn get_group_name(&self) -> Option<GString>;

    fn get_menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget>;

    fn get_menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString>;

    fn get_scrollable(&self) -> bool;

    fn get_show_border(&self) -> bool;

    fn get_show_tabs(&self) -> bool;

    fn get_tab_detachable<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn get_tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget>;

    fn get_tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString>;

    fn get_tab_pos(&self) -> PositionType;

    fn get_tab_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn next_page(&self);

    fn popup_disable(&self);

    fn popup_enable(&self);

    fn prev_page(&self);

    fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType);

    fn set_group_name<'a, P: Into<Option<&'a str>>>(&self, group_name: P);

    fn set_menu_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, menu_label: R);

    fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str);

    fn set_scrollable(&self, scrollable: bool);

    fn set_show_border(&self, show_border: bool);

    fn set_show_tabs(&self, show_tabs: bool);

    fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool);

    fn set_tab_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, tab_label: R);

    fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str);

    fn set_tab_pos(&self, pos: PositionType);

    fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool);

    fn get_property_enable_popup(&self) -> bool;

    fn set_property_enable_popup(&self, enable_popup: bool);

    fn get_property_page(&self) -> i32;

    fn set_property_page(&self, page: i32);

    #[doc(hidden)]
    fn get_child_detachable<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_detachable<T: IsA<Widget>>(&self, item: &T, detachable: bool);

    #[doc(hidden)]
    fn get_child_menu_label<T: IsA<Widget>>(&self, item: &T) -> Option<GString>;

    #[doc(hidden)]
    fn set_child_menu_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, menu_label: P);

    #[doc(hidden)]
    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    #[doc(hidden)]
    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    #[doc(hidden)]
    fn get_child_reorderable<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_reorderable<T: IsA<Widget>>(&self, item: &T, reorderable: bool);

    #[doc(hidden)]
    fn get_child_tab_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_tab_expand<T: IsA<Widget>>(&self, item: &T, tab_expand: bool);

    #[doc(hidden)]
    fn get_child_tab_fill<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_tab_fill<T: IsA<Widget>>(&self, item: &T, tab_fill: bool);

    #[doc(hidden)]
    fn get_child_tab_label<T: IsA<Widget>>(&self, item: &T) -> Option<GString>;

    #[doc(hidden)]
    fn set_child_tab_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, tab_label: P);

    fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_change_current_page(&self, object: i32) -> bool;

    fn connect_create_window<F: Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_focus_tab(&self, object: NotebookTab) -> bool;

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_focus_out(&self, object: DirectionType);

    fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_reorder_tab(&self, object: DirectionType, p0: bool) -> bool;

    fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_page(&self, object: bool) -> bool;

    fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enable_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_group_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scrollable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tab_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Notebook>> NotebookExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn detach_tab<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_notebook_detach_tab(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn get_action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_action_widget(self.to_glib_none().0, pack_type.to_glib()))
        }
    }

    fn get_group_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0))
        }
    }

    fn get_menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_scrollable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0))
        }
    }

    fn get_show_border(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0))
        }
    }

    fn get_show_tabs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0))
        }
    }

    fn get_tab_detachable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_detachable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_tab_pos(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0))
        }
    }

    fn get_tab_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn next_page(&self) {
        unsafe {
            ffi::gtk_notebook_next_page(self.to_glib_none().0);
        }
    }

    fn popup_disable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(self.to_glib_none().0);
        }
    }

    fn popup_enable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(self.to_glib_none().0);
        }
    }

    fn prev_page(&self) {
        unsafe {
            ffi::gtk_notebook_prev_page(self.to_glib_none().0);
        }
    }

    fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(self.to_glib_none().0, widget.to_glib_none().0, pack_type.to_glib());
        }
    }

    fn set_group_name<'a, P: Into<Option<&'a str>>>(&self, group_name: P) {
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.0);
        }
    }

    fn set_menu_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, menu_label: R) {
        let menu_label = menu_label.into();
        let menu_label = menu_label.to_glib_none();
        unsafe {
            ffi::gtk_notebook_set_menu_label(self.to_glib_none().0, child.to_glib_none().0, menu_label.0);
        }
    }

    fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(self.to_glib_none().0, child.to_glib_none().0, menu_text.to_glib_none().0);
        }
    }

    fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.to_glib());
        }
    }

    fn set_show_border(&self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.to_glib());
        }
    }

    fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.to_glib());
        }
    }

    fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(self.to_glib_none().0, child.to_glib_none().0, detachable.to_glib());
        }
    }

    fn set_tab_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, tab_label: R) {
        let tab_label = tab_label.into();
        let tab_label = tab_label.to_glib_none();
        unsafe {
            ffi::gtk_notebook_set_tab_label(self.to_glib_none().0, child.to_glib_none().0, tab_label.0);
        }
    }

    fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(self.to_glib_none().0, child.to_glib_none().0, tab_text.to_glib_none().0);
        }
    }

    fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos.to_glib());
        }
    }

    fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0, reorderable.to_glib());
        }
    }

    fn get_property_enable_popup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"enable-popup\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_enable_popup(&self, enable_popup: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"enable-popup\0".as_ptr() as *const _, Value::from(&enable_popup).to_glib_none().0);
        }
    }

    fn get_property_page(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"page\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_page(&self, page: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"page\0".as_ptr() as *const _, Value::from(&page).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_detachable<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"detachable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_detachable<T: IsA<Widget>>(&self, item: &T, detachable: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"detachable\0".as_ptr() as *const _, Value::from(&detachable).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_menu_label<T: IsA<Widget>>(&self, item: &T) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"menu-label\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[doc(hidden)]
    fn set_child_menu_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, menu_label: P) {
        let menu_label = menu_label.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"menu-label\0".as_ptr() as *const _, Value::from(menu_label).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"position\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"position\0".as_ptr() as *const _, Value::from(&position).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_reorderable<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"reorderable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_reorderable<T: IsA<Widget>>(&self, item: &T, reorderable: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"reorderable\0".as_ptr() as *const _, Value::from(&reorderable).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_tab_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"tab-expand\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_tab_expand<T: IsA<Widget>>(&self, item: &T, tab_expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"tab-expand\0".as_ptr() as *const _, Value::from(&tab_expand).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_tab_fill<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"tab-fill\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_tab_fill<T: IsA<Widget>>(&self, item: &T, tab_fill: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"tab-fill\0".as_ptr() as *const _, Value::from(&tab_fill).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_tab_label<T: IsA<Widget>>(&self, item: &T) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"tab-label\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[doc(hidden)]
    fn set_child_tab_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, tab_label: P) {
        let tab_label = tab_label.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0, b"tab-label\0".as_ptr() as *const _, Value::from(tab_label).to_glib_none().0);
        }
    }

    fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"change-current-page\0".as_ptr() as *const _,
                transmute(change_current_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_change_current_page(&self, object: i32) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("change-current-page", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_create_window<F: Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"create-window\0".as_ptr() as *const _,
                transmute(create_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, NotebookTab) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"focus-tab\0".as_ptr() as *const _,
                transmute(focus_tab_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_focus_tab(&self, object: NotebookTab) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("focus-tab", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"move-focus-out\0".as_ptr() as *const _,
                transmute(move_focus_out_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_focus_out(&self, object: DirectionType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-focus-out", &[&object]).unwrap() };
    }

    fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"page-added\0".as_ptr() as *const _,
                transmute(page_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"page-removed\0".as_ptr() as *const _,
                transmute(page_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"page-reordered\0".as_ptr() as *const _,
                transmute(page_reordered_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"reorder-tab\0".as_ptr() as *const _,
                transmute(reorder_tab_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_reorder_tab(&self, object: DirectionType, p0: bool) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("reorder-tab", &[&object, &p0]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"select-page\0".as_ptr() as *const _,
                transmute(select_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_select_page(&self, object: bool) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("select-page", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"switch-page\0".as_ptr() as *const _,
                transmute(switch_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_enable_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::enable-popup\0".as_ptr() as *const _,
                transmute(notify_enable_popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_group_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::group-name\0".as_ptr() as *const _,
                transmute(notify_group_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::page\0".as_ptr() as *const _,
                transmute(notify_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scrollable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::scrollable\0".as_ptr() as *const _,
                transmute(notify_scrollable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::show-border\0".as_ptr() as *const _,
                transmute(notify_show_border_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::show-tabs\0".as_ptr() as *const _,
                transmute(notify_show_tabs_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tab_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::tab-pos\0".as_ptr() as *const _,
                transmute(notify_tab_pos_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn change_current_page_trampoline<P>(this: *mut ffi::GtkNotebook, object: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    let f: &&(Fn(&P, i32) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), object).to_glib()
}

unsafe extern "C" fn create_window_trampoline<P>(this: *mut ffi::GtkNotebook, page: *mut ffi::GtkWidget, x: libc::c_int, y: libc::c_int, f: glib_ffi::gpointer) -> *mut ffi::GtkNotebook
where P: IsA<Notebook> {
    let f: &&(Fn(&P, &Widget, i32, i32) -> Notebook + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(page), x, y)/*Not checked*/.to_glib_none().0
}

unsafe extern "C" fn focus_tab_trampoline<P>(this: *mut ffi::GtkNotebook, object: ffi::GtkNotebookTab, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    let f: &&(Fn(&P, NotebookTab) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), from_glib(object)).to_glib()
}

unsafe extern "C" fn move_focus_out_trampoline<P>(this: *mut ffi::GtkNotebook, object: ffi::GtkDirectionType, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P, DirectionType) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), from_glib(object))
}

unsafe extern "C" fn page_added_trampoline<P>(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(child), page_num)
}

unsafe extern "C" fn page_removed_trampoline<P>(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(child), page_num)
}

unsafe extern "C" fn page_reordered_trampoline<P>(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(child), page_num)
}

unsafe extern "C" fn reorder_tab_trampoline<P>(this: *mut ffi::GtkNotebook, object: ffi::GtkDirectionType, p0: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    let f: &&(Fn(&P, DirectionType, bool) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), from_glib(object), from_glib(p0)).to_glib()
}

unsafe extern "C" fn select_page_trampoline<P>(this: *mut ffi::GtkNotebook, object: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    let f: &&(Fn(&P, bool) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), from_glib(object)).to_glib()
}

unsafe extern "C" fn switch_page_trampoline<P>(this: *mut ffi::GtkNotebook, page: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(page), page_num)
}

unsafe extern "C" fn notify_enable_popup_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_group_name_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_page_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scrollable_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_border_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_tabs_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tab_pos_trampoline<P>(this: *mut ffi::GtkNotebook, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Notebook::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notebook")
    }
}
