// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TextAccessible(Object<ffi::GtkTextAccessible, ffi::GtkTextAccessibleClass, TextAccessibleClass>) @extends WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_text_accessible_get_type(),
    }
}

impl TextAccessible {}

pub const NONE_TEXT_ACCESSIBLE: Option<&TextAccessible> = None;

impl fmt::Display for TextAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextAccessible")
    }
}
