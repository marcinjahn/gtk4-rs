// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use License;
use Root;
use Widget;
use Window;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct AboutDialog(Object<ffi::GtkAboutDialog, ffi::GtkAboutDialogClass, AboutDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, Root;

    match fn {
        get_type => || ffi::gtk_about_dialog_get_type(),
    }
}

impl AboutDialog {
    pub fn new() -> AboutDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_about_dialog_new()).unsafe_cast()
        }
    }
}

impl Default for AboutDialog {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ABOUT_DIALOG: Option<&AboutDialog> = None;

pub trait AboutDialogExt: 'static {
    fn add_credit_section(&self, section_name: &str, people: &[&str]);

    fn get_artists(&self) -> Vec<GString>;

    fn get_authors(&self) -> Vec<GString>;

    fn get_comments(&self) -> Option<GString>;

    fn get_copyright(&self) -> Option<GString>;

    fn get_documenters(&self) -> Vec<GString>;

    fn get_license(&self) -> Option<GString>;

    fn get_license_type(&self) -> License;

    //fn get_logo(&self) -> /*Ignored*/Option<gdk::Paintable>;

    fn get_logo_icon_name(&self) -> Option<GString>;

    fn get_program_name(&self) -> Option<GString>;

    fn get_system_information(&self) -> Option<GString>;

    fn get_translator_credits(&self) -> Option<GString>;

    fn get_version(&self) -> Option<GString>;

    fn get_website(&self) -> Option<GString>;

    fn get_website_label(&self) -> Option<GString>;

    fn get_wrap_license(&self) -> bool;

    fn set_artists(&self, artists: &[&str]);

    fn set_authors(&self, authors: &[&str]);

    fn set_comments(&self, comments: Option<&str>);

    fn set_copyright(&self, copyright: Option<&str>);

    fn set_documenters(&self, documenters: &[&str]);

    fn set_license(&self, license: Option<&str>);

    fn set_license_type(&self, license_type: License);

    //fn set_logo(&self, logo: /*Ignored*/Option<&gdk::Paintable>);

    fn set_logo_icon_name(&self, icon_name: Option<&str>);

    fn set_program_name(&self, name: &str);

    fn set_system_information(&self, system_information: Option<&str>);

    fn set_translator_credits(&self, translator_credits: Option<&str>);

    fn set_version(&self, version: Option<&str>);

    fn set_website(&self, website: Option<&str>);

    fn set_website_label(&self, website_label: &str);

    fn set_wrap_license(&self, wrap_license: bool);

    fn connect_activate_link<F: Fn(&Self, &str) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_artists_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_authors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_comments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_documenters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_license_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_logo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_logo_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_program_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_system_information_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_translator_credits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_website_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_website_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AboutDialog>> AboutDialogExt for O {
    fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(self.as_ref().to_glib_none().0, section_name.to_glib_none().0, people.to_glib_none().0);
        }
    }

    fn get_artists(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_artists(self.as_ref().to_glib_none().0))
        }
    }

    fn get_authors(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_authors(self.as_ref().to_glib_none().0))
        }
    }

    fn get_comments(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_comments(self.as_ref().to_glib_none().0))
        }
    }

    fn get_copyright(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_copyright(self.as_ref().to_glib_none().0))
        }
    }

    fn get_documenters(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_documenters(self.as_ref().to_glib_none().0))
        }
    }

    fn get_license(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_license(self.as_ref().to_glib_none().0))
        }
    }

    fn get_license_type(&self) -> License {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_license_type(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_logo(&self) -> /*Ignored*/Option<gdk::Paintable> {
    //    unsafe { TODO: call ffi::gtk_about_dialog_get_logo() }
    //}

    fn get_logo_icon_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_program_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_program_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_system_information(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_system_information(self.as_ref().to_glib_none().0))
        }
    }

    fn get_translator_credits(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_translator_credits(self.as_ref().to_glib_none().0))
        }
    }

    fn get_version(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_version(self.as_ref().to_glib_none().0))
        }
    }

    fn get_website(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website(self.as_ref().to_glib_none().0))
        }
    }

    fn get_website_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_wrap_license(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_wrap_license(self.as_ref().to_glib_none().0))
        }
    }

    fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(self.as_ref().to_glib_none().0, artists.to_glib_none().0);
        }
    }

    fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(self.as_ref().to_glib_none().0, authors.to_glib_none().0);
        }
    }

    fn set_comments(&self, comments: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_comments(self.as_ref().to_glib_none().0, comments.to_glib_none().0);
        }
    }

    fn set_copyright(&self, copyright: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(self.as_ref().to_glib_none().0, copyright.to_glib_none().0);
        }
    }

    fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(self.as_ref().to_glib_none().0, documenters.to_glib_none().0);
        }
    }

    fn set_license(&self, license: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_license(self.as_ref().to_glib_none().0, license.to_glib_none().0);
        }
    }

    fn set_license_type(&self, license_type: License) {
        unsafe {
            ffi::gtk_about_dialog_set_license_type(self.as_ref().to_glib_none().0, license_type.to_glib());
        }
    }

    //fn set_logo(&self, logo: /*Ignored*/Option<&gdk::Paintable>) {
    //    unsafe { TODO: call ffi::gtk_about_dialog_set_logo() }
    //}

    fn set_logo_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_program_name(&self, name: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_system_information(&self, system_information: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_system_information(self.as_ref().to_glib_none().0, system_information.to_glib_none().0);
        }
    }

    fn set_translator_credits(&self, translator_credits: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(self.as_ref().to_glib_none().0, translator_credits.to_glib_none().0);
        }
    }

    fn set_version(&self, version: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_version(self.as_ref().to_glib_none().0, version.to_glib_none().0);
        }
    }

    fn set_website(&self, website: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_website(self.as_ref().to_glib_none().0, website.to_glib_none().0);
        }
    }

    fn set_website_label(&self, website_label: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(self.as_ref().to_glib_none().0, website_label.to_glib_none().0);
        }
    }

    fn set_wrap_license(&self, wrap_license: bool) {
        unsafe {
            ffi::gtk_about_dialog_set_wrap_license(self.as_ref().to_glib_none().0, wrap_license.to_glib());
        }
    }

    fn connect_activate_link<F: Fn(&Self, &str) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-link\0".as_ptr() as *const _,
                Some(transmute(activate_link_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_artists_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::artists\0".as_ptr() as *const _,
                Some(transmute(notify_artists_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_authors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::authors\0".as_ptr() as *const _,
                Some(transmute(notify_authors_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_comments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::comments\0".as_ptr() as *const _,
                Some(transmute(notify_comments_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::copyright\0".as_ptr() as *const _,
                Some(transmute(notify_copyright_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_documenters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::documenters\0".as_ptr() as *const _,
                Some(transmute(notify_documenters_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::license\0".as_ptr() as *const _,
                Some(transmute(notify_license_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_license_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::license-type\0".as_ptr() as *const _,
                Some(transmute(notify_license_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_logo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::logo\0".as_ptr() as *const _,
                Some(transmute(notify_logo_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_logo_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::logo-icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_logo_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_program_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::program-name\0".as_ptr() as *const _,
                Some(transmute(notify_program_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_system_information_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::system-information\0".as_ptr() as *const _,
                Some(transmute(notify_system_information_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_translator_credits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::translator-credits\0".as_ptr() as *const _,
                Some(transmute(notify_translator_credits_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::version\0".as_ptr() as *const _,
                Some(transmute(notify_version_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_website_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::website\0".as_ptr() as *const _,
                Some(transmute(notify_website_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_website_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::website-label\0".as_ptr() as *const _,
                Some(transmute(notify_website_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap-license\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_license_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_link_trampoline<P, F: Fn(&P, &str) -> bool + 'static>(this: *mut ffi::GtkAboutDialog, uri: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(uri)).to_glib()
}

unsafe extern "C" fn notify_artists_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_authors_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_comments_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_copyright_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_documenters_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_license_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_license_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_logo_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_logo_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_program_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_system_information_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_translator_credits_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_version_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_website_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_website_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wrap_license_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    let f: &F = &*(f as *const F);
    f(&AboutDialog::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AboutDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AboutDialog")
    }
}
