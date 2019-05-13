// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ScrolledWindowAccessible(Object<ffi::GtkScrolledWindowAccessible, ffi::GtkScrolledWindowAccessibleClass, ScrolledWindowAccessibleClass>) @extends ContainerAccessible, WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_scrolled_window_accessible_get_type(),
    }
}

impl ScrolledWindowAccessible {}

pub const NONE_SCROLLED_WINDOW_ACCESSIBLE: Option<&ScrolledWindowAccessible> = None;

impl fmt::Display for ScrolledWindowAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrolledWindowAccessible")
    }
}
