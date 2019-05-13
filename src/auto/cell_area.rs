// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use CellAreaContext;
use CellEditable;
use CellLayout;
use CellRenderer;
use DirectionType;
use Orientation;
use SizeRequestMode;
use TreeIter;
use TreeModel;
use Widget;
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
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellArea(Object<ffi::GtkCellArea, ffi::GtkCellAreaClass, CellAreaClass>) @implements Buildable, CellLayout;

    match fn {
        get_type => || ffi::gtk_cell_area_get_type(),
    }
}

pub const NONE_CELL_AREA: Option<&CellArea> = None;

pub trait CellAreaExt: 'static {
    //fn activate<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState, edit_only: bool) -> bool;

    //fn activate_cell<P: IsA<Widget>, Q: IsA<CellRenderer>>(&self, widget: &P, renderer: &Q, event: /*Ignored*/&gdk::Event, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState) -> bool;

    fn add<P: IsA<CellRenderer>>(&self, renderer: &P);

    fn add_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(&self, renderer: &P, sibling: &Q);

    //fn add_with_properties<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn apply_attributes<P: IsA<TreeModel>>(&self, tree_model: &P, iter: &mut TreeIter, is_expander: bool, is_expanded: bool);

    fn attribute_connect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str, column: i32);

    fn attribute_disconnect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str);

    fn attribute_get_column<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str) -> i32;

    //fn cell_get<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn cell_get_property<P: IsA<CellRenderer>>(&self, renderer: &P, property_name: &str, value: /*Ignored*/&mut glib::Value);

    //fn cell_get_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn cell_set<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn cell_set_property<P: IsA<CellRenderer>>(&self, renderer: &P, property_name: &str, value: /*Ignored*/&glib::Value);

    //fn cell_set_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn copy_context<P: IsA<CellAreaContext>>(&self, context: &P) -> Option<CellAreaContext>;

    fn create_context(&self) -> Option<CellAreaContext>;

    //fn event<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, event: /*Ignored*/&gdk::Event, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState) -> i32;

    fn focus(&self, direction: DirectionType) -> bool;

    fn foreach<P: FnMut(&CellRenderer) -> bool>(&self, callback: P);

    //fn foreach_alloc<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, cell_area: /*Ignored*/&gdk::Rectangle, background_area: /*Ignored*/&gdk::Rectangle, callback: /*Unimplemented*/FnMut(&CellRenderer, /*Ignored*/gdk::Rectangle, /*Ignored*/gdk::Rectangle) -> bool, callback_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //fn get_cell_allocation<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: IsA<CellRenderer>>(&self, context: &P, widget: &Q, renderer: &R, cell_area: /*Ignored*/&gdk::Rectangle, allocation: /*Ignored*/gdk::Rectangle);

    //fn get_cell_at_position<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, cell_area: /*Ignored*/&gdk::Rectangle, x: i32, y: i32, alloc_area: /*Ignored*/gdk::Rectangle) -> Option<CellRenderer>;

    fn get_current_path_string(&self) -> Option<GString>;

    fn get_edit_widget(&self) -> Option<CellEditable>;

    fn get_edited_cell(&self) -> Option<CellRenderer>;

    fn get_focus_cell(&self) -> Option<CellRenderer>;

    fn get_focus_from_sibling<P: IsA<CellRenderer>>(&self, renderer: &P) -> Option<CellRenderer>;

    fn get_focus_siblings<P: IsA<CellRenderer>>(&self, renderer: &P) -> Vec<CellRenderer>;

    fn get_preferred_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q) -> (i32, i32);

    fn get_preferred_height_for_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, width: i32) -> (i32, i32);

    fn get_preferred_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q) -> (i32, i32);

    fn get_preferred_width_for_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, height: i32) -> (i32, i32);

    fn get_request_mode(&self) -> SizeRequestMode;

    fn has_renderer<P: IsA<CellRenderer>>(&self, renderer: &P) -> bool;

    //fn inner_cell_area<P: IsA<Widget>>(&self, widget: &P, cell_area: /*Ignored*/&gdk::Rectangle, inner_area: /*Ignored*/gdk::Rectangle);

    fn is_activatable(&self) -> bool;

    fn is_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(&self, renderer: &P, sibling: &Q) -> bool;

    fn remove<P: IsA<CellRenderer>>(&self, renderer: &P);

    fn remove_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(&self, renderer: &P, sibling: &Q);

    fn request_renderer<P: IsA<CellRenderer>, Q: IsA<Widget>>(&self, renderer: &P, orientation: Orientation, widget: &Q, for_size: i32) -> (i32, i32);

    fn set_focus_cell<P: IsA<CellRenderer>>(&self, renderer: &P);

    //fn snapshot<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: IsA<Snapshot>>(&self, context: &P, widget: &Q, snapshot: &R, background_area: /*Ignored*/&gdk::Rectangle, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState, paint_focus: bool);

    fn stop_editing(&self, canceled: bool);

    //fn connect_add_editable<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_apply_attributes<F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_focus_changed<F: Fn(&Self, &CellRenderer, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_remove_editable<F: Fn(&Self, &CellRenderer, &CellEditable) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_edit_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_edited_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_focus_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellArea>> CellAreaExt for O {
    //fn activate<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState, edit_only: bool) -> bool {
    //    unsafe { TODO: call ffi::gtk_cell_area_activate() }
    //}

    //fn activate_cell<P: IsA<Widget>, Q: IsA<CellRenderer>>(&self, widget: &P, renderer: &Q, event: /*Ignored*/&gdk::Event, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState) -> bool {
    //    unsafe { TODO: call ffi::gtk_cell_area_activate_cell() }
    //}

    fn add<P: IsA<CellRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_cell_area_add(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0);
        }
    }

    fn add_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(&self, renderer: &P, sibling: &Q) {
        unsafe {
            ffi::gtk_cell_area_add_focus_sibling(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, sibling.as_ref().to_glib_none().0);
        }
    }

    //fn add_with_properties<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_area_add_with_properties() }
    //}

    fn apply_attributes<P: IsA<TreeModel>>(&self, tree_model: &P, iter: &mut TreeIter, is_expander: bool, is_expanded: bool) {
        unsafe {
            ffi::gtk_cell_area_apply_attributes(self.as_ref().to_glib_none().0, tree_model.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, is_expander.to_glib(), is_expanded.to_glib());
        }
    }

    fn attribute_connect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_area_attribute_connect(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, attribute.to_glib_none().0, column);
        }
    }

    fn attribute_disconnect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str) {
        unsafe {
            ffi::gtk_cell_area_attribute_disconnect(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, attribute.to_glib_none().0);
        }
    }

    fn attribute_get_column<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str) -> i32 {
        unsafe {
            ffi::gtk_cell_area_attribute_get_column(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    //fn cell_get<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_get() }
    //}

    //fn cell_get_property<P: IsA<CellRenderer>>(&self, renderer: &P, property_name: &str, value: /*Ignored*/&mut glib::Value) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_get_property() }
    //}

    //fn cell_get_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_get_valist() }
    //}

    //fn cell_set<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_set() }
    //}

    //fn cell_set_property<P: IsA<CellRenderer>>(&self, renderer: &P, property_name: &str, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_set_property() }
    //}

    //fn cell_set_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_set_valist() }
    //}

    fn copy_context<P: IsA<CellAreaContext>>(&self, context: &P) -> Option<CellAreaContext> {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_copy_context(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0))
        }
    }

    fn create_context(&self) -> Option<CellAreaContext> {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_create_context(self.as_ref().to_glib_none().0))
        }
    }

    //fn event<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, event: /*Ignored*/&gdk::Event, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState) -> i32 {
    //    unsafe { TODO: call ffi::gtk_cell_area_event() }
    //}

    fn focus(&self, direction: DirectionType) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_focus(self.as_ref().to_glib_none().0, direction.to_glib()))
        }
    }

    fn foreach<P: FnMut(&CellRenderer) -> bool>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&CellRenderer) -> bool>(renderer: *mut ffi::GtkCellRenderer, data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let renderer = from_glib_borrow(renderer);
            let callback: *mut P = data as *const _ as usize as *mut P;
            let res = (*callback)(&renderer);
            res.to_glib()
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gtk_cell_area_foreach(self.as_ref().to_glib_none().0, callback, super_callback0 as *const _ as usize as *mut _);
        }
    }

    //fn foreach_alloc<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, cell_area: /*Ignored*/&gdk::Rectangle, background_area: /*Ignored*/&gdk::Rectangle, callback: /*Unimplemented*/FnMut(&CellRenderer, /*Ignored*/gdk::Rectangle, /*Ignored*/gdk::Rectangle) -> bool, callback_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_cell_area_foreach_alloc() }
    //}

    //fn get_cell_allocation<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: IsA<CellRenderer>>(&self, context: &P, widget: &Q, renderer: &R, cell_area: /*Ignored*/&gdk::Rectangle, allocation: /*Ignored*/gdk::Rectangle) {
    //    unsafe { TODO: call ffi::gtk_cell_area_get_cell_allocation() }
    //}

    //fn get_cell_at_position<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, cell_area: /*Ignored*/&gdk::Rectangle, x: i32, y: i32, alloc_area: /*Ignored*/gdk::Rectangle) -> Option<CellRenderer> {
    //    unsafe { TODO: call ffi::gtk_cell_area_get_cell_at_position() }
    //}

    fn get_current_path_string(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_current_path_string(self.as_ref().to_glib_none().0))
        }
    }

    fn get_edit_widget(&self) -> Option<CellEditable> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edit_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_edited_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edited_cell(self.as_ref().to_glib_none().0))
        }
    }

    fn get_focus_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_cell(self.as_ref().to_glib_none().0))
        }
    }

    fn get_focus_from_sibling<P: IsA<CellRenderer>>(&self, renderer: &P) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_from_sibling(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0))
        }
    }

    fn get_focus_siblings<P: IsA<CellRenderer>>(&self, renderer: &P) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_cell_area_get_focus_siblings(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0))
        }
    }

    fn get_preferred_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_height(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_height_for_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_height_for_width(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, width, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_width(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn get_preferred_width_for_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(&self, context: &P, widget: &Q, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_width_for_height(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, height, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn get_request_mode(&self) -> SizeRequestMode {
        unsafe {
            from_glib(ffi::gtk_cell_area_get_request_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn has_renderer<P: IsA<CellRenderer>>(&self, renderer: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_has_renderer(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0))
        }
    }

    //fn inner_cell_area<P: IsA<Widget>>(&self, widget: &P, cell_area: /*Ignored*/&gdk::Rectangle, inner_area: /*Ignored*/gdk::Rectangle) {
    //    unsafe { TODO: call ffi::gtk_cell_area_inner_cell_area() }
    //}

    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_activatable(self.as_ref().to_glib_none().0))
        }
    }

    fn is_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(&self, renderer: &P, sibling: &Q) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_focus_sibling(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, sibling.as_ref().to_glib_none().0))
        }
    }

    fn remove<P: IsA<CellRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_cell_area_remove(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0);
        }
    }

    fn remove_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(&self, renderer: &P, sibling: &Q) {
        unsafe {
            ffi::gtk_cell_area_remove_focus_sibling(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, sibling.as_ref().to_glib_none().0);
        }
    }

    fn request_renderer<P: IsA<CellRenderer>, Q: IsA<Widget>>(&self, renderer: &P, orientation: Orientation, widget: &Q, for_size: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::uninitialized();
            let mut natural_size = mem::uninitialized();
            ffi::gtk_cell_area_request_renderer(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, orientation.to_glib(), widget.as_ref().to_glib_none().0, for_size, &mut minimum_size, &mut natural_size);
            (minimum_size, natural_size)
        }
    }

    fn set_focus_cell<P: IsA<CellRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_cell_area_set_focus_cell(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0);
        }
    }

    //fn snapshot<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: IsA<Snapshot>>(&self, context: &P, widget: &Q, snapshot: &R, background_area: /*Ignored*/&gdk::Rectangle, cell_area: /*Ignored*/&gdk::Rectangle, flags: CellRendererState, paint_focus: bool) {
    //    unsafe { TODO: call ffi::gtk_cell_area_snapshot() }
    //}

    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_area_stop_editing(self.as_ref().to_glib_none().0, canceled.to_glib());
        }
    }

    //fn connect_add_editable<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored cell_area: Gdk.Rectangle
    //}

    fn connect_apply_attributes<F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"apply-attributes\0".as_ptr() as *const _,
                Some(transmute(apply_attributes_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_focus_changed<F: Fn(&Self, &CellRenderer, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"focus-changed\0".as_ptr() as *const _,
                Some(transmute(focus_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_remove_editable<F: Fn(&Self, &CellRenderer, &CellEditable) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"remove-editable\0".as_ptr() as *const _,
                Some(transmute(remove_editable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_edit_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::edit-widget\0".as_ptr() as *const _,
                Some(transmute(notify_edit_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_edited_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::edited-cell\0".as_ptr() as *const _,
                Some(transmute(notify_edited_cell_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_focus_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::focus-cell\0".as_ptr() as *const _,
                Some(transmute(notify_focus_cell_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn apply_attributes_trampoline<P, F: Fn(&P, &TreeModel, &TreeIter, bool, bool) + 'static>(this: *mut ffi::GtkCellArea, model: *mut ffi::GtkTreeModel, iter: *mut ffi::GtkTreeIter, is_expander: glib_ffi::gboolean, is_expanded: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<CellArea> {
    let f: &F = &*(f as *const F);
    f(&CellArea::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(model), &from_glib_borrow(iter), from_glib(is_expander), from_glib(is_expanded))
}

unsafe extern "C" fn focus_changed_trampoline<P, F: Fn(&P, &CellRenderer, &str) + 'static>(this: *mut ffi::GtkCellArea, renderer: *mut ffi::GtkCellRenderer, path: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<CellArea> {
    let f: &F = &*(f as *const F);
    f(&CellArea::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(renderer), &GString::from_glib_borrow(path))
}

unsafe extern "C" fn remove_editable_trampoline<P, F: Fn(&P, &CellRenderer, &CellEditable) + 'static>(this: *mut ffi::GtkCellArea, renderer: *mut ffi::GtkCellRenderer, editable: *mut ffi::GtkCellEditable, f: glib_ffi::gpointer)
where P: IsA<CellArea> {
    let f: &F = &*(f as *const F);
    f(&CellArea::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(renderer), &from_glib_borrow(editable))
}

unsafe extern "C" fn notify_edit_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellArea> {
    let f: &F = &*(f as *const F);
    f(&CellArea::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_edited_cell_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellArea> {
    let f: &F = &*(f as *const F);
    f(&CellArea::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_focus_cell_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellArea> {
    let f: &F = &*(f as *const F);
    f(&CellArea::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellArea")
    }
}
