// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Scrollbar(Object<ffi::GtkScrollbar, ffi::GtkScrollbarClass, ScrollbarClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    pub fn new<P: IsA<Adjustment>>(orientation: Orientation, adjustment: Option<&P>) -> Scrollbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrollbar_new(orientation.to_glib(), adjustment.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_SCROLLBAR: Option<&Scrollbar> = None;

pub trait ScrollbarExt: 'static {
    fn get_adjustment(&self) -> Option<Adjustment>;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: Option<&P>);

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Scrollbar>> ScrollbarExt for O {
    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollbar_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: Option<&P>) {
        unsafe {
            ffi::gtk_scrollbar_set_adjustment(self.as_ref().to_glib_none().0, adjustment.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScrollbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Scrollbar> {
    let f: &F = &*(f as *const F);
    f(&Scrollbar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Scrollbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scrollbar")
    }
}
