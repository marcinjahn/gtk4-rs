// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::MediaStream;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkMediaFile")]
    pub struct MediaFile(Object<ffi::GtkMediaFile, ffi::GtkMediaFileClass>) @extends MediaStream, @implements gdk::Paintable;

    match fn {
        type_ => || ffi::gtk_media_file_get_type(),
    }
}

impl MediaFile {
    pub const NONE: Option<&'static MediaFile> = None;

    #[doc(alias = "gtk_media_file_new")]
    pub fn new() -> MediaFile {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_media_file_new()) }
    }

    #[doc(alias = "gtk_media_file_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file(file: &impl IsA<gio::File>) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_file(
                file.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_new_for_filename")]
    #[doc(alias = "new_for_filename")]
    pub fn for_filename(filename: impl AsRef<std::path::Path>) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_filename(
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_new_for_input_stream")]
    #[doc(alias = "new_for_input_stream")]
    pub fn for_input_stream(stream: &impl IsA<gio::InputStream>) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_input_stream(
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_new_for_resource")]
    #[doc(alias = "new_for_resource")]
    pub fn for_resource(resource_path: &str) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }
}

impl Default for MediaFile {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::MediaFile>> Sealed for T {}
}

pub trait MediaFileExt: IsA<MediaFile> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_media_file_clear")]
    fn clear(&self) {
        unsafe {
            ffi::gtk_media_file_clear(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_media_file_get_file")]
    #[doc(alias = "get_file")]
    fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_media_file_get_file(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_media_file_get_input_stream")]
    #[doc(alias = "get_input_stream")]
    fn input_stream(&self) -> Option<gio::InputStream> {
        unsafe {
            from_glib_none(ffi::gtk_media_file_get_input_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_set_file")]
    fn set_file(&self, file: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_media_file_set_file(
                self.as_ref().to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_media_file_set_filename")]
    fn set_filename(&self, filename: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::gtk_media_file_set_filename(
                self.as_ref().to_glib_none().0,
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_media_file_set_input_stream")]
    fn set_input_stream(&self, stream: Option<&impl IsA<gio::InputStream>>) {
        unsafe {
            ffi::gtk_media_file_set_input_stream(
                self.as_ref().to_glib_none().0,
                stream.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_media_file_set_resource")]
    fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_media_file_set_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "file")]
    fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<P: IsA<MediaFile>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaFile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_file_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "input-stream")]
    fn connect_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_stream_trampoline<
            P: IsA<MediaFile>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaFile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-stream\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_input_stream_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<MediaFile>> MediaFileExt for O {}
