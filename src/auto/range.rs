// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use ScrollType;
use Widget;
use ffi;
use glib;
use glib::object::Cast;
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
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Range(Object<ffi::GtkRange, ffi::GtkRangeClass, RangeClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_range_get_type(),
    }
}

pub const NONE_RANGE: Option<&Range> = None;

pub trait RangeExt: 'static {
    fn get_adjustment(&self) -> Option<Adjustment>;

    fn get_fill_level(&self) -> f64;

    fn get_flippable(&self) -> bool;

    fn get_inverted(&self) -> bool;

    //fn get_range_rect(&self, range_rect: /*Ignored*/gdk::Rectangle);

    fn get_restrict_to_fill_level(&self) -> bool;

    fn get_round_digits(&self) -> i32;

    fn get_show_fill_level(&self) -> bool;

    fn get_slider_range(&self) -> (i32, i32);

    fn get_slider_size_fixed(&self) -> bool;

    fn get_value(&self) -> f64;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    fn set_fill_level(&self, fill_level: f64);

    fn set_flippable(&self, flippable: bool);

    fn set_increments(&self, step: f64, page: f64);

    fn set_inverted(&self, setting: bool);

    fn set_range(&self, min: f64, max: f64);

    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool);

    fn set_round_digits(&self, round_digits: i32);

    fn set_show_fill_level(&self, show_fill_level: bool);

    fn set_slider_size_fixed(&self, size_fixed: bool);

    fn set_value(&self, value: f64);

    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_slider(&self, step: ScrollType);

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_restrict_to_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_round_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Range>> RangeExt for O {
    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_range_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_fill_level(&self) -> f64 {
        unsafe {
            ffi::gtk_range_get_fill_level(self.as_ref().to_glib_none().0)
        }
    }

    fn get_flippable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_flippable(self.as_ref().to_glib_none().0))
        }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_inverted(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_range_rect(&self, range_rect: /*Ignored*/gdk::Rectangle) {
    //    unsafe { TODO: call ffi::gtk_range_get_range_rect() }
    //}

    fn get_restrict_to_fill_level(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_restrict_to_fill_level(self.as_ref().to_glib_none().0))
        }
    }

    fn get_round_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_range_get_round_digits(self.as_ref().to_glib_none().0)
        }
    }

    fn get_show_fill_level(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_show_fill_level(self.as_ref().to_glib_none().0))
        }
    }

    fn get_slider_range(&self) -> (i32, i32) {
        unsafe {
            let mut slider_start = mem::uninitialized();
            let mut slider_end = mem::uninitialized();
            ffi::gtk_range_get_slider_range(self.as_ref().to_glib_none().0, &mut slider_start, &mut slider_end);
            (slider_start, slider_end)
        }
    }

    fn get_slider_size_fixed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_slider_size_fixed(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_range_get_value(self.as_ref().to_glib_none().0)
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            ffi::gtk_range_set_adjustment(self.as_ref().to_glib_none().0, adjustment.as_ref().to_glib_none().0);
        }
    }

    fn set_fill_level(&self, fill_level: f64) {
        unsafe {
            ffi::gtk_range_set_fill_level(self.as_ref().to_glib_none().0, fill_level);
        }
    }

    fn set_flippable(&self, flippable: bool) {
        unsafe {
            ffi::gtk_range_set_flippable(self.as_ref().to_glib_none().0, flippable.to_glib());
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_range_set_increments(self.as_ref().to_glib_none().0, step, page);
        }
    }

    fn set_inverted(&self, setting: bool) {
        unsafe {
            ffi::gtk_range_set_inverted(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_range_set_range(self.as_ref().to_glib_none().0, min, max);
        }
    }

    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool) {
        unsafe {
            ffi::gtk_range_set_restrict_to_fill_level(self.as_ref().to_glib_none().0, restrict_to_fill_level.to_glib());
        }
    }

    fn set_round_digits(&self, round_digits: i32) {
        unsafe {
            ffi::gtk_range_set_round_digits(self.as_ref().to_glib_none().0, round_digits);
        }
    }

    fn set_show_fill_level(&self, show_fill_level: bool) {
        unsafe {
            ffi::gtk_range_set_show_fill_level(self.as_ref().to_glib_none().0, show_fill_level.to_glib());
        }
    }

    fn set_slider_size_fixed(&self, size_fixed: bool) {
        unsafe {
            ffi::gtk_range_set_slider_size_fixed(self.as_ref().to_glib_none().0, size_fixed.to_glib());
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_range_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"adjust-bounds\0".as_ptr() as *const _,
                Some(transmute(adjust_bounds_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"change-value\0".as_ptr() as *const _,
                Some(transmute(change_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-slider\0".as_ptr() as *const _,
                Some(transmute(move_slider_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_slider(&self, step: ScrollType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-slider", &[&step]).unwrap() };
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"value-changed\0".as_ptr() as *const _,
                Some(transmute(value_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::fill-level\0".as_ptr() as *const _,
                Some(transmute(notify_fill_level_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute(notify_inverted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_restrict_to_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::restrict-to-fill-level\0".as_ptr() as *const _,
                Some(transmute(notify_restrict_to_fill_level_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_round_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::round-digits\0".as_ptr() as *const _,
                Some(transmute(notify_round_digits_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-fill-level\0".as_ptr() as *const _,
                Some(transmute(notify_show_fill_level_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn adjust_bounds_trampoline<P, F: Fn(&P, f64) + 'static>(this: *mut ffi::GtkRange, value: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast(), value)
}

unsafe extern "C" fn change_value_trampoline<P, F: Fn(&P, ScrollType, f64) -> bool + 'static>(this: *mut ffi::GtkRange, scroll: ffi::GtkScrollType, value: libc::c_double, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast(), from_glib(scroll), value).to_glib()
}

unsafe extern "C" fn move_slider_trampoline<P, F: Fn(&P, ScrollType) + 'static>(this: *mut ffi::GtkRange, step: ffi::GtkScrollType, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast(), from_glib(step))
}

unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_fill_level_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_inverted_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_restrict_to_fill_level_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_round_digits_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_fill_level_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Range> {
    let f: &F = &*(f as *const F);
    f(&Range::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Range")
    }
}
