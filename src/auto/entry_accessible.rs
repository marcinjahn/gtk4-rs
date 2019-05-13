// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct EntryAccessible(Object<ffi::GtkEntryAccessible, ffi::GtkEntryAccessibleClass, EntryAccessibleClass>) @extends WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_entry_accessible_get_type(),
    }
}

impl EntryAccessible {}

pub const NONE_ENTRY_ACCESSIBLE: Option<&EntryAccessible> = None;

impl fmt::Display for EntryAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntryAccessible")
    }
}
