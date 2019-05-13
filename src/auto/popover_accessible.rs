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
    pub struct PopoverAccessible(Object<ffi::GtkPopoverAccessible, ffi::GtkPopoverAccessibleClass, PopoverAccessibleClass>) @extends ContainerAccessible, WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_popover_accessible_get_type(),
    }
}

impl PopoverAccessible {}

pub const NONE_POPOVER_ACCESSIBLE: Option<&PopoverAccessible> = None;

impl fmt::Display for PopoverAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PopoverAccessible")
    }
}
