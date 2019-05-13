// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use MenuItemAccessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct CheckMenuItemAccessible(Object<ffi::GtkCheckMenuItemAccessible, ffi::GtkCheckMenuItemAccessibleClass, CheckMenuItemAccessibleClass>) @extends MenuItemAccessible, ContainerAccessible, WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_check_menu_item_accessible_get_type(),
    }
}

impl CheckMenuItemAccessible {}

pub const NONE_CHECK_MENU_ITEM_ACCESSIBLE: Option<&CheckMenuItemAccessible> = None;

impl fmt::Display for CheckMenuItemAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CheckMenuItemAccessible")
    }
}
