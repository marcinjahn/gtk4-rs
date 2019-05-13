// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use MediaStream;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Video(Object<ffi::GtkVideo, ffi::GtkVideoClass, VideoClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_video_get_type(),
    }
}

impl Video {
    pub fn new() -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new()).unsafe_cast()
        }
    }

    //pub fn new_for_file(file: /*Ignored*/Option<&gio::File>) -> Video {
    //    unsafe { TODO: call ffi::gtk_video_new_for_file() }
    //}

    pub fn new_for_filename<P: AsRef<std::path::Path>>(filename: P) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_filename(filename.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_for_media_stream<P: IsA<MediaStream>>(stream: Option<&P>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_media_stream(stream.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_for_resource(resource_path: Option<&str>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_resource(resource_path.to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for Video {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_VIDEO: Option<&Video> = None;

pub trait VideoExt: 'static {
    fn get_autoplay(&self) -> bool;

    //fn get_file(&self) -> /*Ignored*/Option<gio::File>;

    fn get_loop(&self) -> bool;

    fn get_media_stream(&self) -> Option<MediaStream>;

    fn set_autoplay(&self, autoplay: bool);

    //fn set_file(&self, file: /*Ignored*/Option<&gio::File>);

    fn set_filename(&self, filename: Option<&str>);

    fn set_loop(&self, loop_: bool);

    fn set_media_stream<P: IsA<MediaStream>>(&self, stream: Option<&P>);

    fn set_resource(&self, resource_path: Option<&str>);

    fn connect_property_autoplay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_media_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Video>> VideoExt for O {
    fn get_autoplay(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_video_get_autoplay(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_video_get_file() }
    //}

    fn get_loop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_video_get_loop(self.as_ref().to_glib_none().0))
        }
    }

    fn get_media_stream(&self) -> Option<MediaStream> {
        unsafe {
            from_glib_none(ffi::gtk_video_get_media_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn set_autoplay(&self, autoplay: bool) {
        unsafe {
            ffi::gtk_video_set_autoplay(self.as_ref().to_glib_none().0, autoplay.to_glib());
        }
    }

    //fn set_file(&self, file: /*Ignored*/Option<&gio::File>) {
    //    unsafe { TODO: call ffi::gtk_video_set_file() }
    //}

    fn set_filename(&self, filename: Option<&str>) {
        unsafe {
            ffi::gtk_video_set_filename(self.as_ref().to_glib_none().0, filename.to_glib_none().0);
        }
    }

    fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gtk_video_set_loop(self.as_ref().to_glib_none().0, loop_.to_glib());
        }
    }

    fn set_media_stream<P: IsA<MediaStream>>(&self, stream: Option<&P>) {
        unsafe {
            ffi::gtk_video_set_media_stream(self.as_ref().to_glib_none().0, stream.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_video_set_resource(self.as_ref().to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    fn connect_property_autoplay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::autoplay\0".as_ptr() as *const _,
                Some(transmute(notify_autoplay_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::file\0".as_ptr() as *const _,
                Some(transmute(notify_file_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::loop\0".as_ptr() as *const _,
                Some(transmute(notify_loop_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_media_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::media-stream\0".as_ptr() as *const _,
                Some(transmute(notify_media_stream_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_autoplay_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkVideo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Video> {
    let f: &F = &*(f as *const F);
    f(&Video::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkVideo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Video> {
    let f: &F = &*(f as *const F);
    f(&Video::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_loop_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkVideo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Video> {
    let f: &F = &*(f as *const F);
    f(&Video::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_media_stream_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkVideo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Video> {
    let f: &F = &*(f as *const F);
    f(&Video::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Video {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Video")
    }
}
