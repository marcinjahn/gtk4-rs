// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use ShortcutType;
use SizeGroup;
use Widget;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ShortcutsShortcut(Object<ffi::GtkShortcutsShortcut, ffi::GtkShortcutsShortcutClass, ShortcutsShortcutClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_shortcuts_shortcut_get_type(),
    }
}

pub const NONE_SHORTCUTS_SHORTCUT: Option<&ShortcutsShortcut> = None;

pub trait ShortcutsShortcutExt: 'static {
    fn set_property_accel_size_group(&self, accel_size_group: Option<&SizeGroup>);

    fn get_property_accelerator(&self) -> Option<GString>;

    fn set_property_accelerator(&self, accelerator: Option<&str>);

    fn get_property_action_name(&self) -> Option<GString>;

    fn set_property_action_name(&self, action_name: Option<&str>);

    //fn get_property_icon(&self) -> /*Ignored*/Option<gio::Icon>;

    //fn set_property_icon(&self, icon: /*Ignored*/Option<&gio::Icon>);

    fn get_property_icon_set(&self) -> bool;

    fn set_property_icon_set(&self, icon_set: bool);

    fn get_property_shortcut_type(&self) -> ShortcutType;

    fn set_property_shortcut_type(&self, shortcut_type: ShortcutType);

    fn get_property_subtitle(&self) -> Option<GString>;

    fn set_property_subtitle(&self, subtitle: Option<&str>);

    fn get_property_subtitle_set(&self) -> bool;

    fn set_property_subtitle_set(&self, subtitle_set: bool);

    fn get_property_title(&self) -> Option<GString>;

    fn set_property_title(&self, title: Option<&str>);

    fn set_property_title_size_group(&self, title_size_group: Option<&SizeGroup>);

    fn connect_property_accel_size_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accelerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shortcut_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_size_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ShortcutsShortcut>> ShortcutsShortcutExt for O {
    fn set_property_accel_size_group(&self, accel_size_group: Option<&SizeGroup>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"accel-size-group\0".as_ptr() as *const _, Value::from(accel_size_group).to_glib_none().0);
        }
    }

    fn get_property_accelerator(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"accelerator\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accelerator(&self, accelerator: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"accelerator\0".as_ptr() as *const _, Value::from(accelerator).to_glib_none().0);
        }
    }

    fn get_property_action_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"action-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_action_name(&self, action_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"action-name\0".as_ptr() as *const _, Value::from(action_name).to_glib_none().0);
        }
    }

    //fn get_property_icon(&self) -> /*Ignored*/Option<gio::Icon> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get()
    //    }
    //}

    //fn set_property_icon(&self, icon: /*Ignored*/Option<&gio::Icon>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon\0".as_ptr() as *const _, Value::from(icon).to_glib_none().0);
    //    }
    //}

    fn get_property_icon_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-set\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_icon_set(&self, icon_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-set\0".as_ptr() as *const _, Value::from(&icon_set).to_glib_none().0);
        }
    }

    fn get_property_shortcut_type(&self) -> ShortcutType {
        unsafe {
            let mut value = Value::from_type(<ShortcutType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"shortcut-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_shortcut_type(&self, shortcut_type: ShortcutType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"shortcut-type\0".as_ptr() as *const _, Value::from(&shortcut_type).to_glib_none().0);
        }
    }

    fn get_property_subtitle(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"subtitle\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"subtitle\0".as_ptr() as *const _, Value::from(subtitle).to_glib_none().0);
        }
    }

    fn get_property_subtitle_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"subtitle-set\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_subtitle_set(&self, subtitle_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"subtitle-set\0".as_ptr() as *const _, Value::from(&subtitle_set).to_glib_none().0);
        }
    }

    fn get_property_title(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"title\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_title(&self, title: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"title\0".as_ptr() as *const _, Value::from(title).to_glib_none().0);
        }
    }

    fn set_property_title_size_group(&self, title_size_group: Option<&SizeGroup>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"title-size-group\0".as_ptr() as *const _, Value::from(title_size_group).to_glib_none().0);
        }
    }

    fn connect_property_accel_size_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-size-group\0".as_ptr() as *const _,
                Some(transmute(notify_accel_size_group_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accelerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accelerator\0".as_ptr() as *const _,
                Some(transmute(notify_accelerator_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_action_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::action-name\0".as_ptr() as *const _,
                Some(transmute(notify_action_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::direction\0".as_ptr() as *const _,
                Some(transmute(notify_direction_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute(notify_icon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-set\0".as_ptr() as *const _,
                Some(transmute(notify_icon_set_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_shortcut_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shortcut-type\0".as_ptr() as *const _,
                Some(transmute(notify_shortcut_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute(notify_subtitle_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_subtitle_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle-set\0".as_ptr() as *const _,
                Some(transmute(notify_subtitle_set_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_title_size_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title-size-group\0".as_ptr() as *const _,
                Some(transmute(notify_title_size_group_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_accel_size_group_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accelerator_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_action_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_icon_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_icon_set_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_shortcut_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_subtitle_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_subtitle_set_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_title_size_group_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkShortcutsShortcut, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsShortcut> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsShortcut::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ShortcutsShortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShortcutsShortcut")
    }
}
