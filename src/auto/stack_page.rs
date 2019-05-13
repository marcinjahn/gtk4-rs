// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

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
    pub struct StackPage(Object<ffi::GtkStackPage, ffi::GtkStackPageClass, StackPageClass>);

    match fn {
        get_type => || ffi::gtk_stack_page_get_type(),
    }
}

pub const NONE_STACK_PAGE: Option<&StackPage> = None;

pub trait StackPageExt: 'static {
    fn get_child(&self) -> Option<Widget>;

    fn get_property_icon_name(&self) -> Option<GString>;

    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn get_property_name(&self) -> Option<GString>;

    fn get_property_needs_attention(&self) -> bool;

    fn set_property_needs_attention(&self, needs_attention: bool);

    fn get_property_title(&self) -> Option<GString>;

    fn set_property_title(&self, title: Option<&str>);

    fn get_property_visible(&self) -> bool;

    fn set_property_visible(&self, visible: bool);

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_needs_attention_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StackPage>> StackPageExt for O {
    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_page_get_child(self.as_ref().to_glib_none().0))
        }
    }

    fn get_property_icon_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-name\0".as_ptr() as *const _, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_needs_attention(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"needs-attention\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_needs_attention(&self, needs_attention: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"needs-attention\0".as_ptr() as *const _, Value::from(&needs_attention).to_glib_none().0);
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

    fn get_property_visible(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"visible\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_visible(&self, visible: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"visible\0".as_ptr() as *const _, Value::from(&visible).to_glib_none().0);
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_needs_attention_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::needs-attention\0".as_ptr() as *const _,
                Some(transmute(notify_needs_attention_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible\0".as_ptr() as *const _,
                Some(transmute(notify_visible_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkStackPage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StackPage> {
    let f: &F = &*(f as *const F);
    f(&StackPage::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_needs_attention_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkStackPage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StackPage> {
    let f: &F = &*(f as *const F);
    f(&StackPage::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkStackPage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StackPage> {
    let f: &F = &*(f as *const F);
    f(&StackPage::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkStackPage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StackPage> {
    let f: &F = &*(f as *const F);
    f(&StackPage::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for StackPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StackPage")
    }
}
