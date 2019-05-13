// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BaselinePosition;
use Buildable;
use Container;
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
    pub struct Box(Object<ffi::GtkBox, ffi::GtkBoxClass, BoxClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_box_get_type(),
    }
}

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_box_new(orientation.to_glib(), spacing)).unsafe_cast()
        }
    }
}

pub const NONE_BOX: Option<&Box> = None;

pub trait BoxExt: 'static {
    fn get_baseline_position(&self) -> BaselinePosition;

    fn get_homogeneous(&self) -> bool;

    fn get_spacing(&self) -> i32;

    fn insert_child_after<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, sibling: Option<&Q>);

    fn reorder_child_after<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, sibling: Option<&Q>);

    fn set_baseline_position(&self, position: BaselinePosition);

    fn set_homogeneous(&self, homogeneous: bool);

    fn set_spacing(&self, spacing: i32);

    fn connect_property_baseline_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Box>> BoxExt for O {
    fn get_baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_get_baseline_position(self.as_ref().to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_box_get_homogeneous(self.as_ref().to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn insert_child_after<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, sibling: Option<&Q>) {
        unsafe {
            ffi::gtk_box_insert_child_after(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, sibling.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn reorder_child_after<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, sibling: Option<&Q>) {
        unsafe {
            ffi::gtk_box_reorder_child_after(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, sibling.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_set_baseline_position(self.as_ref().to_glib_none().0, position.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_set_homogeneous(self.as_ref().to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_box_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn connect_property_baseline_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::baseline-position\0".as_ptr() as *const _,
                Some(transmute(notify_baseline_position_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute(notify_homogeneous_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute(notify_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_baseline_position_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Box> {
    let f: &F = &*(f as *const F);
    f(&Box::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_homogeneous_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Box> {
    let f: &F = &*(f as *const F);
    f(&Box::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Box> {
    let f: &F = &*(f as *const F);
    f(&Box::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Box")
    }
}
