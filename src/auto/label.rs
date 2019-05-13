// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Justification;
use Menu;
use MovementStep;
use Widget;
use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Label(Object<ffi::GtkLabel, ffi::GtkLabelClass, LabelClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_label_get_type(),
    }
}

impl Label {
    pub fn new(str: Option<&str>) -> Label {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new(str.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic(str: Option<&str>) -> Label {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new_with_mnemonic(str.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_LABEL: Option<&Label> = None;

pub trait LabelExt: 'static {
    //fn get_attributes(&self) -> /*Ignored*/Option<pango::AttrList>;

    fn get_current_uri(&self) -> Option<GString>;

    //fn get_ellipsize(&self) -> /*Ignored*/pango::EllipsizeMode;

    fn get_justify(&self) -> Justification;

    fn get_label(&self) -> Option<GString>;

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout>;

    fn get_layout_offsets(&self) -> (i32, i32);

    fn get_line_wrap(&self) -> bool;

    //fn get_line_wrap_mode(&self) -> /*Ignored*/pango::WrapMode;

    fn get_lines(&self) -> i32;

    fn get_max_width_chars(&self) -> i32;

    fn get_mnemonic_keyval(&self) -> u32;

    fn get_mnemonic_widget(&self) -> Option<Widget>;

    fn get_selectable(&self) -> bool;

    fn get_selection_bounds(&self) -> Option<(i32, i32)>;

    fn get_single_line_mode(&self) -> bool;

    fn get_text(&self) -> Option<GString>;

    fn get_track_visited_links(&self) -> bool;

    fn get_use_markup(&self) -> bool;

    fn get_use_underline(&self) -> bool;

    fn get_width_chars(&self) -> i32;

    fn get_xalign(&self) -> f32;

    fn get_yalign(&self) -> f32;

    fn select_region(&self, start_offset: i32, end_offset: i32);

    //fn set_attributes(&self, attrs: /*Ignored*/Option<&pango::AttrList>);

    //fn set_ellipsize(&self, mode: /*Ignored*/pango::EllipsizeMode);

    fn set_justify(&self, jtype: Justification);

    fn set_label(&self, str: &str);

    fn set_line_wrap(&self, wrap: bool);

    //fn set_line_wrap_mode(&self, wrap_mode: /*Ignored*/pango::WrapMode);

    fn set_lines(&self, lines: i32);

    fn set_markup(&self, str: &str);

    fn set_markup_with_mnemonic(&self, str: &str);

    fn set_max_width_chars(&self, n_chars: i32);

    fn set_mnemonic_widget<P: IsA<Widget>>(&self, widget: Option<&P>);

    fn set_pattern(&self, pattern: &str);

    fn set_selectable(&self, setting: bool);

    fn set_single_line_mode(&self, single_line_mode: bool);

    fn set_text(&self, str: &str);

    fn set_text_with_mnemonic(&self, str: &str);

    fn set_track_visited_links(&self, track_links: bool);

    fn set_use_markup(&self, setting: bool);

    fn set_use_underline(&self, setting: bool);

    fn set_width_chars(&self, n_chars: i32);

    fn set_xalign(&self, xalign: f32);

    fn set_yalign(&self, yalign: f32);

    fn get_property_cursor_position(&self) -> i32;

    fn get_property_selection_bound(&self) -> i32;

    fn get_property_wrap(&self) -> bool;

    fn set_property_wrap(&self, wrap: bool);

    //fn get_property_wrap_mode(&self) -> /*Ignored*/pango::WrapMode;

    //fn set_property_wrap_mode(&self, wrap_mode: /*Ignored*/pango::WrapMode);

    fn connect_activate_current_link<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_current_link(&self);

    fn connect_activate_link<F: Fn(&Self, &str) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_copy_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_copy_clipboard(&self);

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_cursor(&self, step: MovementStep, count: i32, extend_selection: bool);

    fn connect_populate_popup<F: Fn(&Self, &Menu) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cursor_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_justify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mnemonic_keyval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mnemonic_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_bound_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_single_line_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_track_visited_links_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Label>> LabelExt for O {
    //fn get_attributes(&self) -> /*Ignored*/Option<pango::AttrList> {
    //    unsafe { TODO: call ffi::gtk_label_get_attributes() }
    //}

    fn get_current_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_current_uri(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_ellipsize(&self) -> /*Ignored*/pango::EllipsizeMode {
    //    unsafe { TODO: call ffi::gtk_label_get_ellipsize() }
    //}

    fn get_justify(&self) -> Justification {
        unsafe {
            from_glib(ffi::gtk_label_get_justify(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_label(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call ffi::gtk_label_get_layout() }
    //}

    fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gtk_label_get_layout_offsets(self.as_ref().to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_line_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_line_wrap(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_line_wrap_mode(&self) -> /*Ignored*/pango::WrapMode {
    //    unsafe { TODO: call ffi::gtk_label_get_line_wrap_mode() }
    //}

    fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(self.as_ref().to_glib_none().0)
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(self.as_ref().to_glib_none().0)
        }
    }

    fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(self.as_ref().to_glib_none().0)
        }
    }

    fn get_mnemonic_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_mnemonic_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_selectable(self.as_ref().to_glib_none().0))
        }
    }

    fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start = mem::uninitialized();
            let mut end = mem::uninitialized();
            let ret = from_glib(ffi::gtk_label_get_selection_bounds(self.as_ref().to_glib_none().0, &mut start, &mut end));
            if ret { Some((start, end)) } else { None }
        }
    }

    fn get_single_line_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_single_line_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_track_visited_links(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_track_visited_links(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_markup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_underline(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(self.as_ref().to_glib_none().0)
        }
    }

    fn get_xalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_xalign(self.as_ref().to_glib_none().0)
        }
    }

    fn get_yalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_yalign(self.as_ref().to_glib_none().0)
        }
    }

    fn select_region(&self, start_offset: i32, end_offset: i32) {
        unsafe {
            ffi::gtk_label_select_region(self.as_ref().to_glib_none().0, start_offset, end_offset);
        }
    }

    //fn set_attributes(&self, attrs: /*Ignored*/Option<&pango::AttrList>) {
    //    unsafe { TODO: call ffi::gtk_label_set_attributes() }
    //}

    //fn set_ellipsize(&self, mode: /*Ignored*/pango::EllipsizeMode) {
    //    unsafe { TODO: call ffi::gtk_label_set_ellipsize() }
    //}

    fn set_justify(&self, jtype: Justification) {
        unsafe {
            ffi::gtk_label_set_justify(self.as_ref().to_glib_none().0, jtype.to_glib());
        }
    }

    fn set_label(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_label(self.as_ref().to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_line_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_label_set_line_wrap(self.as_ref().to_glib_none().0, wrap.to_glib());
        }
    }

    //fn set_line_wrap_mode(&self, wrap_mode: /*Ignored*/pango::WrapMode) {
    //    unsafe { TODO: call ffi::gtk_label_set_line_wrap_mode() }
    //}

    fn set_lines(&self, lines: i32) {
        unsafe {
            ffi::gtk_label_set_lines(self.as_ref().to_glib_none().0, lines);
        }
    }

    fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup(self.as_ref().to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_markup_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup_with_mnemonic(self.as_ref().to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_max_width_chars(self.as_ref().to_glib_none().0, n_chars);
        }
    }

    fn set_mnemonic_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            ffi::gtk_label_set_mnemonic_widget(self.as_ref().to_glib_none().0, widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_label_set_pattern(self.as_ref().to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    fn set_selectable(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_selectable(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_single_line_mode(&self, single_line_mode: bool) {
        unsafe {
            ffi::gtk_label_set_single_line_mode(self.as_ref().to_glib_none().0, single_line_mode.to_glib());
        }
    }

    fn set_text(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text(self.as_ref().to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_text_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text_with_mnemonic(self.as_ref().to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_track_visited_links(&self, track_links: bool) {
        unsafe {
            ffi::gtk_label_set_track_visited_links(self.as_ref().to_glib_none().0, track_links.to_glib());
        }
    }

    fn set_use_markup(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_markup(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_underline(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_width_chars(self.as_ref().to_glib_none().0, n_chars);
        }
    }

    fn set_xalign(&self, xalign: f32) {
        unsafe {
            ffi::gtk_label_set_xalign(self.as_ref().to_glib_none().0, xalign);
        }
    }

    fn set_yalign(&self, yalign: f32) {
        unsafe {
            ffi::gtk_label_set_yalign(self.as_ref().to_glib_none().0, yalign);
        }
    }

    fn get_property_cursor_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"cursor-position\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_selection_bound(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"selection-bound\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_wrap(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"wrap\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_wrap(&self, wrap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"wrap\0".as_ptr() as *const _, Value::from(&wrap).to_glib_none().0);
        }
    }

    //fn get_property_wrap_mode(&self) -> /*Ignored*/pango::WrapMode {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"wrap-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    //fn set_property_wrap_mode(&self, wrap_mode: /*Ignored*/pango::WrapMode) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"wrap-mode\0".as_ptr() as *const _, Value::from(&wrap_mode).to_glib_none().0);
    //    }
    //}

    fn connect_activate_current_link<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-current-link\0".as_ptr() as *const _,
                Some(transmute(activate_current_link_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate_current_link(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate-current-link", &[]).unwrap() };
    }

    fn connect_activate_link<F: Fn(&Self, &str) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-link\0".as_ptr() as *const _,
                Some(transmute(activate_link_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_copy_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"copy-clipboard\0".as_ptr() as *const _,
                Some(transmute(copy_clipboard_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_copy_clipboard(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("copy-clipboard", &[]).unwrap() };
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-cursor\0".as_ptr() as *const _,
                Some(transmute(move_cursor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_cursor(&self, step: MovementStep, count: i32, extend_selection: bool) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-cursor", &[&step, &count, &extend_selection]).unwrap() };
    }

    fn connect_populate_popup<F: Fn(&Self, &Menu) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"populate-popup\0".as_ptr() as *const _,
                Some(transmute(populate_popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attributes\0".as_ptr() as *const _,
                Some(transmute(notify_attributes_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_cursor_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cursor-position\0".as_ptr() as *const _,
                Some(transmute(notify_cursor_position_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute(notify_ellipsize_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_justify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::justify\0".as_ptr() as *const _,
                Some(transmute(notify_justify_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::lines\0".as_ptr() as *const _,
                Some(transmute(notify_lines_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_max_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-width-chars\0".as_ptr() as *const _,
                Some(transmute(notify_max_width_chars_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mnemonic_keyval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mnemonic-keyval\0".as_ptr() as *const _,
                Some(transmute(notify_mnemonic_keyval_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mnemonic_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mnemonic-widget\0".as_ptr() as *const _,
                Some(transmute(notify_mnemonic_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pattern\0".as_ptr() as *const _,
                Some(transmute(notify_pattern_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute(notify_selectable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_selection_bound_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-bound\0".as_ptr() as *const _,
                Some(transmute(notify_selection_bound_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_single_line_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::single-line-mode\0".as_ptr() as *const _,
                Some(transmute(notify_single_line_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_track_visited_links_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::track-visited-links\0".as_ptr() as *const _,
                Some(transmute(notify_track_visited_links_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute(notify_use_markup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute(notify_use_underline_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width-chars\0".as_ptr() as *const _,
                Some(transmute(notify_width_chars_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap-mode\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute(notify_xalign_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute(notify_yalign_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_current_link_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn activate_link_trampoline<P, F: Fn(&P, &str) -> bool + 'static>(this: *mut ffi::GtkLabel, uri: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(uri)).to_glib()
}

unsafe extern "C" fn copy_clipboard_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn move_cursor_trampoline<P, F: Fn(&P, MovementStep, i32, bool) + 'static>(this: *mut ffi::GtkLabel, step: ffi::GtkMovementStep, count: libc::c_int, extend_selection: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast(), from_glib(step), count, from_glib(extend_selection))
}

unsafe extern "C" fn populate_popup_trampoline<P, F: Fn(&P, &Menu) + 'static>(this: *mut ffi::GtkLabel, menu: *mut ffi::GtkMenu, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(menu))
}

unsafe extern "C" fn notify_attributes_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_cursor_position_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_ellipsize_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_justify_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_lines_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_max_width_chars_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_mnemonic_keyval_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_mnemonic_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pattern_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selectable_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_bound_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_single_line_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_track_visited_links_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_markup_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_chars_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wrap_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wrap_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_xalign_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_yalign_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Label> {
    let f: &F = &*(f as *const F);
    f(&Label::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Label")
    }
}
