// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AppChooser;
use Buildable;
use Widget;
use ffi;
use glib::GString;
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
    pub struct AppChooserWidget(Object<ffi::GtkAppChooserWidget, ffi::GtkAppChooserWidgetClass, AppChooserWidgetClass>) @extends Widget, @implements Buildable, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_widget_get_type(),
    }
}

impl AppChooserWidget {
    pub fn new(content_type: &str) -> AppChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_widget_new(content_type.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_APP_CHOOSER_WIDGET: Option<&AppChooserWidget> = None;

pub trait AppChooserWidgetExt: 'static {
    fn get_default_text(&self) -> Option<GString>;

    fn get_show_all(&self) -> bool;

    fn get_show_default(&self) -> bool;

    fn get_show_fallback(&self) -> bool;

    fn get_show_other(&self) -> bool;

    fn get_show_recommended(&self) -> bool;

    fn set_default_text(&self, text: &str);

    fn set_show_all(&self, setting: bool);

    fn set_show_default(&self, setting: bool);

    fn set_show_fallback(&self, setting: bool);

    fn set_show_other(&self, setting: bool);

    fn set_show_recommended(&self, setting: bool);

    //fn connect_application_activated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_application_selected<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_populate_popup<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_other_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_recommended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserWidget>> AppChooserWidgetExt for O {
    fn get_default_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_widget_get_default_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_all(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_default(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_fallback(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_other(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_other(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_recommended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_recommended(self.as_ref().to_glib_none().0))
        }
    }

    fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_show_all(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_all(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_default(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_fallback(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_fallback(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_other(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_other(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_recommended(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_recommended(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    //fn connect_application_activated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored application: Gio.AppInfo
    //}

    //fn connect_application_selected<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored application: Gio.AppInfo
    //}

    //fn connect_populate_popup<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored application: Gio.AppInfo
    //}

    fn connect_property_default_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-text\0".as_ptr() as *const _,
                Some(transmute(notify_default_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-all\0".as_ptr() as *const _,
                Some(transmute(notify_show_all_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-default\0".as_ptr() as *const _,
                Some(transmute(notify_show_default_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-fallback\0".as_ptr() as *const _,
                Some(transmute(notify_show_fallback_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_other_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-other\0".as_ptr() as *const _,
                Some(transmute(notify_show_other_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_recommended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-recommended\0".as_ptr() as *const _,
                Some(transmute(notify_show_recommended_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_default_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &F = &*(f as *const F);
    f(&AppChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_all_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &F = &*(f as *const F);
    f(&AppChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_default_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &F = &*(f as *const F);
    f(&AppChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_fallback_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &F = &*(f as *const F);
    f(&AppChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_other_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &F = &*(f as *const F);
    f(&AppChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_recommended_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &F = &*(f as *const F);
    f(&AppChooserWidget::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AppChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppChooserWidget")
    }
}
