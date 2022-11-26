// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkAlertDialog")]
    pub struct AlertDialog(Object<ffi::GtkAlertDialog, ffi::GtkAlertDialogClass>);

    match fn {
        type_ => || ffi::gtk_alert_dialog_get_type(),
    }
}

impl AlertDialog {
    //#[doc(alias = "gtk_alert_dialog_new")]
    //pub fn new(format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> AlertDialog {
    //    unsafe { TODO: call ffi:gtk_alert_dialog_new() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AlertDialog`] objects.
    ///
    /// This method returns an instance of [`AlertDialogBuilder`](crate::builders::AlertDialogBuilder) which can be used to create [`AlertDialog`] objects.
    pub fn builder() -> AlertDialogBuilder {
        AlertDialogBuilder::default()
    }

    #[doc(alias = "gtk_alert_dialog_get_buttons")]
    #[doc(alias = "get_buttons")]
    pub fn buttons(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_alert_dialog_get_buttons(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_alert_dialog_get_cancel_button")]
    #[doc(alias = "get_cancel_button")]
    pub fn cancel_button(&self) -> i32 {
        unsafe { ffi::gtk_alert_dialog_get_cancel_button(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_alert_dialog_get_default_button")]
    #[doc(alias = "get_default_button")]
    pub fn default_button(&self) -> i32 {
        unsafe { ffi::gtk_alert_dialog_get_default_button(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_alert_dialog_get_detail")]
    #[doc(alias = "get_detail")]
    pub fn detail(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_alert_dialog_get_detail(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_alert_dialog_get_message")]
    #[doc(alias = "get_message")]
    pub fn message(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_alert_dialog_get_message(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_alert_dialog_get_modal")]
    #[doc(alias = "get_modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_alert_dialog_get_modal(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_alert_dialog_set_buttons")]
    pub fn set_buttons(&self, labels: &[&str]) {
        unsafe {
            ffi::gtk_alert_dialog_set_buttons(self.to_glib_none().0, labels.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_alert_dialog_set_cancel_button")]
    pub fn set_cancel_button(&self, button: i32) {
        unsafe {
            ffi::gtk_alert_dialog_set_cancel_button(self.to_glib_none().0, button);
        }
    }

    #[doc(alias = "gtk_alert_dialog_set_default_button")]
    pub fn set_default_button(&self, button: i32) {
        unsafe {
            ffi::gtk_alert_dialog_set_default_button(self.to_glib_none().0, button);
        }
    }

    #[doc(alias = "gtk_alert_dialog_set_detail")]
    pub fn set_detail(&self, detail: &str) {
        unsafe {
            ffi::gtk_alert_dialog_set_detail(self.to_glib_none().0, detail.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_alert_dialog_set_message")]
    pub fn set_message(&self, message: &str) {
        unsafe {
            ffi::gtk_alert_dialog_set_message(self.to_glib_none().0, message.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_alert_dialog_set_modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_alert_dialog_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "gtk_alert_dialog_show")]
    pub fn show(&self, parent: Option<&impl IsA<Window>>) {
        unsafe {
            ffi::gtk_alert_dialog_show(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "buttons")]
    pub fn connect_buttons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buttons_trampoline<F: Fn(&AlertDialog) + 'static>(
            this: *mut ffi::GtkAlertDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buttons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buttons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "cancel-button")]
    pub fn connect_cancel_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cancel_button_trampoline<F: Fn(&AlertDialog) + 'static>(
            this: *mut ffi::GtkAlertDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cancel-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cancel_button_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "default-button")]
    pub fn connect_default_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_button_trampoline<F: Fn(&AlertDialog) + 'static>(
            this: *mut ffi::GtkAlertDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_button_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "detail")]
    pub fn connect_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detail_trampoline<F: Fn(&AlertDialog) + 'static>(
            this: *mut ffi::GtkAlertDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::detail\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_detail_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "message")]
    pub fn connect_message_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_trampoline<F: Fn(&AlertDialog) + 'static>(
            this: *mut ffi::GtkAlertDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&AlertDialog) + 'static>(
            this: *mut ffi::GtkAlertDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
impl Default for AlertDialog {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AlertDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AlertDialogBuilder {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    buttons: Option<Vec<String>>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    cancel_button: Option<i32>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    default_button: Option<i32>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    detail: Option<String>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    message: Option<String>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    modal: Option<bool>,
}

impl AlertDialogBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AlertDialogBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AlertDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AlertDialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref buttons) = self.buttons {
            properties.push(("buttons", buttons));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref cancel_button) = self.cancel_button {
            properties.push(("cancel-button", cancel_button));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref default_button) = self.default_button {
            properties.push(("default-button", default_button));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref detail) = self.detail {
            properties.push(("detail", detail));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref message) = self.message {
            properties.push(("message", message));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        glib::Object::new::<AlertDialog>(&properties)
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn buttons(mut self, buttons: Vec<String>) -> Self {
        self.buttons = Some(buttons);
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn cancel_button(mut self, cancel_button: i32) -> Self {
        self.cancel_button = Some(cancel_button);
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn default_button(mut self, default_button: i32) -> Self {
        self.default_button = Some(default_button);
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.to_string());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }
}

impl fmt::Display for AlertDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AlertDialog")
    }
}
