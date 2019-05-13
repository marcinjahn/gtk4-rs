// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SortListModel(Object<ffi::GtkSortListModel, ffi::GtkSortListModelClass, SortListModelClass>);

    match fn {
        get_type => || ffi::gtk_sort_list_model_get_type(),
    }
}

impl SortListModel {
    //pub fn new(model: /*Ignored*/&gio::ListModel, sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer, /*Unimplemented*/Fundamental: Pointer) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> SortListModel {
    //    unsafe { TODO: call ffi::gtk_sort_list_model_new() }
    //}

    pub fn new_for_type(item_type: glib::types::Type) -> SortListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_sort_list_model_new_for_type(item_type.to_glib()))
        }
    }
}

pub const NONE_SORT_LIST_MODEL: Option<&SortListModel> = None;

pub trait SortListModelExt: 'static {
    //fn get_model(&self) -> /*Ignored*/Option<gio::ListModel>;

    fn has_sort(&self) -> bool;

    fn resort(&self);

    //fn set_model(&self, model: /*Ignored*/Option<&gio::ListModel>);

    //fn set_sort_func(&self, sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer, /*Unimplemented*/Fundamental: Pointer) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn get_property_has_sort(&self) -> bool;

    fn get_property_item_type(&self) -> glib::types::Type;

    fn connect_property_has_sort_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SortListModel>> SortListModelExt for O {
    //fn get_model(&self) -> /*Ignored*/Option<gio::ListModel> {
    //    unsafe { TODO: call ffi::gtk_sort_list_model_get_model() }
    //}

    fn has_sort(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_sort_list_model_has_sort(self.as_ref().to_glib_none().0))
        }
    }

    fn resort(&self) {
        unsafe {
            ffi::gtk_sort_list_model_resort(self.as_ref().to_glib_none().0);
        }
    }

    //fn set_model(&self, model: /*Ignored*/Option<&gio::ListModel>) {
    //    unsafe { TODO: call ffi::gtk_sort_list_model_set_model() }
    //}

    //fn set_sort_func(&self, sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer, /*Unimplemented*/Fundamental: Pointer) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_sort_list_model_set_sort_func() }
    //}

    fn get_property_has_sort(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"has-sort\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_item_type(&self) -> glib::types::Type {
        unsafe {
            let mut value = Value::from_type(<glib::types::Type as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"item-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_has_sort_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-sort\0".as_ptr() as *const _,
                Some(transmute(notify_has_sort_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_has_sort_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkSortListModel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SortListModel> {
    let f: &F = &*(f as *const F);
    f(&SortListModel::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SortListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SortListModel")
    }
}
