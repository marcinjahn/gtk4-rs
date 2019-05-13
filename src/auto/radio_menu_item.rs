// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use CheckMenuItem;
use Container;
use MenuItem;
use Widget;
use ffi;
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
    pub struct RadioMenuItem(Object<ffi::GtkRadioMenuItem, ffi::GtkRadioMenuItemClass, RadioMenuItemClass>) @extends CheckMenuItem, MenuItem, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_menu_item_get_type(),
    }
}

impl RadioMenuItem {
    pub fn new_from_widget<P: IsA<RadioMenuItem>>(group: Option<&P>) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_from_widget(group.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_label_from_widget<P: IsA<RadioMenuItem>>(group: Option<&P>, label: Option<&str>) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_label_from_widget(group.map(|p| p.as_ref()).to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic_from_widget<P: IsA<RadioMenuItem>>(group: Option<&P>, label: Option<&str>) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_mnemonic_from_widget(group.map(|p| p.as_ref()).to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_RADIO_MENU_ITEM: Option<&RadioMenuItem> = None;

pub trait RadioMenuItemExt: 'static {
    fn get_group(&self) -> Vec<RadioMenuItem>;

    fn join_group<P: IsA<RadioMenuItem>>(&self, group_source: Option<&P>);

    fn set_property_group(&self, group: Option<&RadioMenuItem>);

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RadioMenuItem>> RadioMenuItemExt for O {
    fn get_group(&self) -> Vec<RadioMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_menu_item_get_group(self.as_ref().to_glib_none().0))
        }
    }

    fn join_group<P: IsA<RadioMenuItem>>(&self, group_source: Option<&P>) {
        unsafe {
            ffi::gtk_radio_menu_item_join_group(self.as_ref().to_glib_none().0, group_source.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_property_group(&self, group: Option<&RadioMenuItem>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"group\0".as_ptr() as *const _, Value::from(group).to_glib_none().0);
        }
    }

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"group-changed\0".as_ptr() as *const _,
                Some(transmute(group_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::group\0".as_ptr() as *const _,
                Some(transmute(notify_group_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn group_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRadioMenuItem, f: glib_ffi::gpointer)
where P: IsA<RadioMenuItem> {
    let f: &F = &*(f as *const F);
    f(&RadioMenuItem::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_group_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRadioMenuItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RadioMenuItem> {
    let f: &F = &*(f as *const F);
    f(&RadioMenuItem::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for RadioMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RadioMenuItem")
    }
}
