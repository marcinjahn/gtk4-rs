// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EventController, Gesture, GestureSingle, PropagationLimit, PropagationPhase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkGestureLongPress")]
    pub struct GestureLongPress(Object<ffi::GtkGestureLongPress, ffi::GtkGestureLongPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    #[doc(alias = "gtk_gesture_long_press_new")]
    pub fn new() -> GestureLongPress {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_long_press_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureLongPress`] objects.
    ///
    /// This method returns an instance of [`GestureLongPressBuilder`](crate::builders::GestureLongPressBuilder) which can be used to create [`GestureLongPress`] objects.
    pub fn builder() -> GestureLongPressBuilder {
        GestureLongPressBuilder::new()
    }

    #[doc(alias = "gtk_gesture_long_press_get_delay_factor")]
    #[doc(alias = "get_delay_factor")]
    pub fn delay_factor(&self) -> f64 {
        unsafe { ffi::gtk_gesture_long_press_get_delay_factor(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_gesture_long_press_set_delay_factor")]
    pub fn set_delay_factor(&self, delay_factor: f64) {
        unsafe {
            ffi::gtk_gesture_long_press_set_delay_factor(self.to_glib_none().0, delay_factor);
        }
    }

    #[doc(alias = "cancelled")]
    pub fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut ffi::GtkGestureLongPress,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    cancelled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pressed")]
    pub fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pressed_trampoline<F: Fn(&GestureLongPress, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureLongPress,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pressed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "delay-factor")]
    pub fn connect_delay_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_factor_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut ffi::GtkGestureLongPress,
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
                b"notify::delay-factor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_delay_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureLongPress {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureLongPress`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureLongPressBuilder {
    builder: glib::object::ObjectBuilder<'static, GestureLongPress>,
}

impl GestureLongPressBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn delay_factor(self, delay_factor: f64) -> Self {
        Self {
            builder: self.builder.property("delay-factor", delay_factor),
        }
    }

    pub fn button(self, button: u32) -> Self {
        Self {
            builder: self.builder.property("button", button),
        }
    }

    pub fn exclusive(self, exclusive: bool) -> Self {
        Self {
            builder: self.builder.property("exclusive", exclusive),
        }
    }

    pub fn touch_only(self, touch_only: bool) -> Self {
        Self {
            builder: self.builder.property("touch-only", touch_only),
        }
    }

    pub fn n_points(self, n_points: u32) -> Self {
        Self {
            builder: self.builder.property("n-points", n_points),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureLongPress`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureLongPress {
        self.builder.build()
    }
}
