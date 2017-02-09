// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use Action;
use SettingsBindFlags;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Settings(Object<ffi::GSettings>);

    match fn {
        get_type => || ffi::g_settings_get_type(),
    }
}

impl Settings {
    pub fn new(schema_id: &str) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new(schema_id.to_glib_none().0))
        }
    }

    //pub fn new_full<'a, T: Into<Option<&'a str>>>(schema: /*Ignored*/&SettingsSchema, backend: /*Ignored*/Option<&mut SettingsBackend>, path: T) -> Settings {
    //    unsafe { TODO: call ffi::g_settings_new_full() }
    //}

    //pub fn new_with_backend(schema_id: &str, backend: /*Ignored*/&mut SettingsBackend) -> Settings {
    //    unsafe { TODO: call ffi::g_settings_new_with_backend() }
    //}

    //pub fn new_with_backend_and_path(schema_id: &str, backend: /*Ignored*/&mut SettingsBackend, path: &str) -> Settings {
    //    unsafe { TODO: call ffi::g_settings_new_with_backend_and_path() }
    //}

    pub fn new_with_path(schema_id: &str, path: &str) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new_with_path(schema_id.to_glib_none().0, path.to_glib_none().0))
        }
    }

    pub fn apply(&self) {
        unsafe {
            ffi::g_settings_apply(self.to_glib_none().0);
        }
    }

    pub fn bind<T: IsA<glib::Object>>(&self, key: &str, object: &T, property: &str, flags: SettingsBindFlags) {
        unsafe {
            ffi::g_settings_bind(self.to_glib_none().0, key.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0, flags.to_glib());
        }
    }

    //pub fn bind_with_mapping<T: IsA<glib::Object>>(&self, key: &str, object: &T, property: &str, flags: SettingsBindFlags, get_mapping: /*Unknown conversion*//*Unimplemented*/SettingsBindGetMapping, set_mapping: /*Unknown conversion*//*Unimplemented*/SettingsBindSetMapping, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::g_settings_bind_with_mapping() }
    //}

    pub fn bind_writable<T: IsA<glib::Object>>(&self, key: &str, object: &T, property: &str, inverted: bool) {
        unsafe {
            ffi::g_settings_bind_writable(self.to_glib_none().0, key.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0, inverted.to_glib());
        }
    }

    pub fn create_action(&self, key: &str) -> Option<Action> {
        unsafe {
            from_glib_full(ffi::g_settings_create_action(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn delay(&self) {
        unsafe {
            ffi::g_settings_delay(self.to_glib_none().0);
        }
    }

    //pub fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_settings_get() }
    //}

    pub fn get_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_get_boolean(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_child(&self, name: &str) -> Option<Settings> {
        unsafe {
            from_glib_full(ffi::g_settings_get_child(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_40")]
    pub fn get_default_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_default_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        unsafe {
            ffi::g_settings_get_double(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn get_enum(&self, key: &str) -> i32 {
        unsafe {
            ffi::g_settings_get_enum(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn get_flags(&self, key: &str) -> u32 {
        unsafe {
            ffi::g_settings_get_flags(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn get_has_unapplied(&self) -> bool {
        unsafe {
            from_glib(ffi::g_settings_get_has_unapplied(self.to_glib_none().0))
        }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        unsafe {
            ffi::g_settings_get_int(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_50")]
    pub fn get_int64(&self, key: &str) -> i64 {
        unsafe {
            ffi::g_settings_get_int64(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    //pub fn get_mapped(&self, key: &str, mapping: /*Unknown conversion*//*Unimplemented*/SettingsGetMapping, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::g_settings_get_mapped() }
    //}

    pub fn get_range(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_range(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_settings_get_string(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_strv(&self, key: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_get_strv(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_uint(&self, key: &str) -> u32 {
        unsafe {
            ffi::g_settings_get_uint(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_50")]
    pub fn get_uint64(&self, key: &str) -> u64 {
        unsafe {
            ffi::g_settings_get_uint64(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_40")]
    pub fn get_user_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_user_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn is_writable(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_is_writable(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn list_children(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_list_children(self.to_glib_none().0))
        }
    }

    pub fn list_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_list_keys(self.to_glib_none().0))
        }
    }

    pub fn range_check(&self, key: &str, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_range_check(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    pub fn reset(&self, key: &str) {
        unsafe {
            ffi::g_settings_reset(self.to_glib_none().0, key.to_glib_none().0);
        }
    }

    pub fn revert(&self) {
        unsafe {
            ffi::g_settings_revert(self.to_glib_none().0);
        }
    }

    //pub fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::g_settings_set() }
    //}

    pub fn set_boolean(&self, key: &str, value: bool) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_boolean(self.to_glib_none().0, key.to_glib_none().0, value.to_glib()))
        }
    }

    pub fn set_double(&self, key: &str, value: f64) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_double(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    pub fn set_enum(&self, key: &str, value: i32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_enum(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    pub fn set_flags(&self, key: &str, value: u32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_flags(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    pub fn set_int(&self, key: &str, value: i32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    #[cfg(feature = "v2_50")]
    pub fn set_int64(&self, key: &str, value: i64) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_int64(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    pub fn set_string(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_string(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    pub fn set_strv(&self, key: &str, value: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_strv(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    pub fn set_uint(&self, key: &str, value: u32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_uint(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    #[cfg(feature = "v2_50")]
    pub fn set_uint64(&self, key: &str, value: u64) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_uint64(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    pub fn set_value(&self, key: &str, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_value(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    pub fn get_property_delay_apply(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "delay-apply".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_has_unapplied(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "has-unapplied".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_path(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "path".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_path(&self, path: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "path".to_glib_none().0, Value::from(path).to_glib_none().0);
        }
    }

    pub fn get_property_schema(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "schema".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_schema(&self, schema: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "schema".to_glib_none().0, Value::from(schema).to_glib_none().0);
        }
    }

    pub fn get_property_schema_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "schema-id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_schema_id(&self, schema_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "schema-id".to_glib_none().0, Value::from(schema_id).to_glib_none().0);
        }
    }

    //pub fn set_property_settings_schema(&self, settings_schema: /*Ignored*/Option<&SettingsSchema>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "settings-schema".to_glib_none().0, Value::from(settings_schema).to_glib_none().0);
    //    }
    //}

    pub fn list_relocatable_schemas() -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_settings_list_relocatable_schemas())
        }
    }

    pub fn list_schemas() -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_settings_list_schemas())
        }
    }

    pub fn sync() {
        unsafe {
            ffi::g_settings_sync();
        }
    }

    pub fn unbind<T: IsA<glib::Object>>(object: &T, property: &str) {
        unsafe {
            ffi::g_settings_unbind(object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    //pub fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unimplemented keys: *.CArray TypeId { ns_id: 2, id: 4 }
    //}

    pub fn connect_changed<F: Fn(&Settings, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Settings, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_writable_change_event<F: Fn(&Settings, u32) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Settings, u32) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "writable-change-event",
                transmute(writable_change_event_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_writable_changed<F: Fn(&Settings, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Settings, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "writable-changed",
                transmute(writable_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline(this: *mut ffi::GSettings, key: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Settings, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(key))
}

unsafe extern "C" fn writable_change_event_trampoline(this: *mut ffi::GSettings, key: libc::c_uint, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Settings, u32) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), key).to_glib()
}

unsafe extern "C" fn writable_changed_trampoline(this: *mut ffi::GSettings, key: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Settings, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(key))
}
