// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Display, Rectangle, SubpixelLayout};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkMonitor")]
    pub struct Monitor(Object<ffi::GdkMonitor, ffi::GdkMonitorClass>);

    match fn {
        type_ => || ffi::gdk_monitor_get_type(),
    }
}

impl Monitor {
    pub const NONE: Option<&'static Monitor> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Monitor>> Sealed for T {}
}

pub trait MonitorExt: IsA<Monitor> + sealed::Sealed + 'static {
    #[doc(alias = "gdk_monitor_get_connector")]
    #[doc(alias = "get_connector")]
    fn connector(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_connector(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gdk_monitor_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_monitor_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_monitor_get_display(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_geometry")]
    #[doc(alias = "get_geometry")]
    fn geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(
                self.as_ref().to_glib_none().0,
                geometry.to_glib_none_mut().0,
            );
            geometry
        }
    }

    #[doc(alias = "gdk_monitor_get_height_mm")]
    #[doc(alias = "get_height_mm")]
    fn height_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_height_mm(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_manufacturer")]
    #[doc(alias = "get_manufacturer")]
    fn manufacturer(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_manufacturer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_monitor_get_model")]
    #[doc(alias = "get_model")]
    fn model(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_model(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_refresh_rate")]
    #[doc(alias = "get_refresh_rate")]
    fn refresh_rate(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_refresh_rate(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gdk_monitor_get_scale")]
    #[doc(alias = "get_scale")]
    fn scale(&self) -> f64 {
        unsafe { ffi::gdk_monitor_get_scale(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_scale_factor")]
    #[doc(alias = "get_scale_factor")]
    fn scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_scale_factor(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_subpixel_layout")]
    #[doc(alias = "get_subpixel_layout")]
    fn subpixel_layout(&self) -> SubpixelLayout {
        unsafe {
            from_glib(ffi::gdk_monitor_get_subpixel_layout(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_monitor_get_width_mm")]
    #[doc(alias = "get_width_mm")]
    fn width_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_width_mm(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_is_valid")]
    fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gdk_monitor_is_valid(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "invalidate")]
    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn invalidate_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"invalidate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    invalidate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "connector")]
    fn connect_connector_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connector_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::connector\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_connector_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "description")]
    fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::description\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "geometry")]
    fn connect_geometry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_geometry_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::geometry\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_geometry_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "height-mm")]
    fn connect_height_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_mm_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height-mm\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_height_mm_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "manufacturer")]
    fn connect_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_manufacturer_trampoline<
            P: IsA<Monitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::manufacturer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_manufacturer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "refresh-rate")]
    fn connect_refresh_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_refresh_rate_trampoline<
            P: IsA<Monitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::refresh-rate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_refresh_rate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "scale")]
    fn connect_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scale-factor")]
    fn connect_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<
            P: IsA<Monitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-factor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subpixel-layout")]
    fn connect_subpixel_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subpixel_layout_trampoline<
            P: IsA<Monitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subpixel-layout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_subpixel_layout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "valid")]
    fn connect_valid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_valid_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::valid\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_valid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "width-mm")]
    fn connect_width_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_mm_trampoline<P: IsA<Monitor>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width-mm\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_width_mm_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Monitor>> MonitorExt for O {}
