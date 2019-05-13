// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use CheckMenuItemAccessible;
use ContainerAccessible;
use MenuItemAccessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct RadioMenuItemAccessible(Object<ffi::GtkRadioMenuItemAccessible, ffi::GtkRadioMenuItemAccessibleClass, RadioMenuItemAccessibleClass>) @extends CheckMenuItemAccessible, MenuItemAccessible, ContainerAccessible, WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_radio_menu_item_accessible_get_type(),
    }
}

impl RadioMenuItemAccessible {}

pub const NONE_RADIO_MENU_ITEM_ACCESSIBLE: Option<&RadioMenuItemAccessible> = None;

impl fmt::Display for RadioMenuItemAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RadioMenuItemAccessible")
    }
}
