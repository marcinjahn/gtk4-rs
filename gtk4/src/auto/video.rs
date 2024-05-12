// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
use crate::GraphicsOffloadEnabled;
use crate::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, MediaStream,
    Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkVideo")]
    pub struct Video(Object<ffi::GtkVideo, ffi::GtkVideoClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_video_get_type(),
    }
}

impl Video {
    #[doc(alias = "gtk_video_new")]
    pub fn new() -> Video {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_video_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_video_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file(file: Option<&impl IsA<gio::File>>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_file(
                file.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_video_new_for_filename")]
    #[doc(alias = "new_for_filename")]
    pub fn for_filename(filename: Option<impl AsRef<std::path::Path>>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_filename(
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_video_new_for_media_stream")]
    #[doc(alias = "new_for_media_stream")]
    pub fn for_media_stream(stream: Option<&impl IsA<MediaStream>>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_media_stream(
                stream.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_video_new_for_resource")]
    #[doc(alias = "new_for_resource")]
    pub fn for_resource(resource_path: Option<&str>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Video`] objects.
    ///
    /// This method returns an instance of [`VideoBuilder`](crate::builders::VideoBuilder) which can be used to create [`Video`] objects.
    pub fn builder() -> VideoBuilder {
        VideoBuilder::new()
    }

    #[doc(alias = "gtk_video_get_autoplay")]
    #[doc(alias = "get_autoplay")]
    pub fn is_autoplay(&self) -> bool {
        unsafe { from_glib(ffi::gtk_video_get_autoplay(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_video_get_file(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_video_get_graphics_offload")]
    #[doc(alias = "get_graphics_offload")]
    pub fn graphics_offload(&self) -> GraphicsOffloadEnabled {
        unsafe { from_glib(ffi::gtk_video_get_graphics_offload(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_get_loop")]
    #[doc(alias = "get_loop")]
    pub fn is_loop(&self) -> bool {
        unsafe { from_glib(ffi::gtk_video_get_loop(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_get_media_stream")]
    #[doc(alias = "get_media_stream")]
    pub fn media_stream(&self) -> Option<MediaStream> {
        unsafe { from_glib_none(ffi::gtk_video_get_media_stream(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_set_autoplay")]
    pub fn set_autoplay(&self, autoplay: bool) {
        unsafe {
            ffi::gtk_video_set_autoplay(self.to_glib_none().0, autoplay.into_glib());
        }
    }

    #[doc(alias = "gtk_video_set_file")]
    pub fn set_file(&self, file: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_video_set_file(
                self.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_video_set_filename")]
    pub fn set_filename(&self, filename: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::gtk_video_set_filename(
                self.to_glib_none().0,
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_video_set_graphics_offload")]
    pub fn set_graphics_offload(&self, enabled: GraphicsOffloadEnabled) {
        unsafe {
            ffi::gtk_video_set_graphics_offload(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "gtk_video_set_loop")]
    pub fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gtk_video_set_loop(self.to_glib_none().0, loop_.into_glib());
        }
    }

    #[doc(alias = "gtk_video_set_media_stream")]
    pub fn set_media_stream(&self, stream: Option<&impl IsA<MediaStream>>) {
        unsafe {
            ffi::gtk_video_set_media_stream(
                self.to_glib_none().0,
                stream.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_video_set_resource")]
    pub fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_video_set_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[doc(alias = "autoplay")]
    pub fn connect_autoplay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoplay_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::autoplay\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_autoplay_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::file\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "graphics-offload")]
    pub fn connect_graphics_offload_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_graphics_offload_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::graphics-offload\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_graphics_offload_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "loop")]
    pub fn connect_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loop_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::loop\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_loop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "media-stream")]
    pub fn connect_media_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_stream_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::media-stream\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_media_stream_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Video {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Video`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct VideoBuilder {
    builder: glib::object::ObjectBuilder<'static, Video>,
}

impl VideoBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn autoplay(self, autoplay: bool) -> Self {
        Self {
            builder: self.builder.property("autoplay", autoplay),
        }
    }

    pub fn file(self, file: &impl IsA<gio::File>) -> Self {
        Self {
            builder: self.builder.property("file", file.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    pub fn graphics_offload(self, graphics_offload: GraphicsOffloadEnabled) -> Self {
        Self {
            builder: self.builder.property("graphics-offload", graphics_offload),
        }
    }

    pub fn loop_(self, loop_: bool) -> Self {
        Self {
            builder: self.builder.property("loop", loop_),
        }
    }

    pub fn media_stream(self, media_stream: &impl IsA<MediaStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("media-stream", media_stream.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Video`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Video {
        self.builder.build()
    }
}
