// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ButtonAccessible;
use ContainerAccessible;
use ToggleButtonAccessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct MenuButtonAccessible(Object<ffi::GtkMenuButtonAccessible, ffi::GtkMenuButtonAccessibleClass, MenuButtonAccessibleClass>) @extends ToggleButtonAccessible, ButtonAccessible, ContainerAccessible, WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_menu_button_accessible_get_type(),
    }
}

impl MenuButtonAccessible {}

pub const NONE_MENU_BUTTON_ACCESSIBLE: Option<&MenuButtonAccessible> = None;

impl fmt::Display for MenuButtonAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuButtonAccessible")
    }
}
