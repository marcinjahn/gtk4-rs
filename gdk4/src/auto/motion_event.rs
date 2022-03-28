// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkMotionEvent")]
    pub struct MotionEvent(Shared<ffi::GdkMotionEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for MotionEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_motion_event_get_type()) }
    }
}

impl MotionEvent {}

impl fmt::Display for MotionEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MotionEvent")
    }
}
