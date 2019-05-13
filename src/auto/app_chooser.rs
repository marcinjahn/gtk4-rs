// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct AppChooser(Interface<ffi::GtkAppChooser>) @requires Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_app_chooser_get_type(),
    }
}

pub const NONE_APP_CHOOSER: Option<&AppChooser> = None;

pub trait AppChooserExt: 'static {
    //fn get_app_info(&self) -> /*Ignored*/Option<gio::AppInfo>;

    fn get_content_type(&self) -> Option<GString>;

    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    //fn get_app_info(&self) -> /*Ignored*/Option<gio::AppInfo> {
    //    unsafe { TODO: call ffi::gtk_app_chooser_get_app_info() }
    //}

    fn get_content_type(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_content_type(self.as_ref().to_glib_none().0))
        }
    }

    fn refresh(&self) {
        unsafe {
            ffi::gtk_app_chooser_refresh(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for AppChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppChooser")
    }
}
