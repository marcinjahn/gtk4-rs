// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SelectionMode;
use TreeIter;
use TreeModel;
use TreePath;
use TreeView;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TreeSelection(Object<ffi::GtkTreeSelection, ffi::GtkTreeSelectionClass, TreeSelectionClass>);

    match fn {
        get_type => || ffi::gtk_tree_selection_get_type(),
    }
}

pub const NONE_TREE_SELECTION: Option<&TreeSelection> = None;

pub trait TreeSelectionExt: 'static {
    fn count_selected_rows(&self) -> i32;

    fn get_mode(&self) -> SelectionMode;

    //fn get_select_function(&self) -> Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>;

    fn get_selected(&self) -> Option<(TreeModel, TreeIter)>;

    fn get_selected_rows(&self) -> (Vec<TreePath>, TreeModel);

    fn get_tree_view(&self) -> Option<TreeView>;

    //fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn iter_is_selected(&self, iter: &mut TreeIter) -> bool;

    fn path_is_selected(&self, path: &mut TreePath) -> bool;

    fn select_all(&self);

    fn select_iter(&self, iter: &mut TreeIter);

    fn select_path(&self, path: &mut TreePath);

    fn select_range(&self, start_path: &mut TreePath, end_path: &mut TreePath);

    fn selected_foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter)>(&self, func: P);

    fn set_mode(&self, type_: SelectionMode);

    fn set_select_function(&self, func: Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>);

    fn unselect_all(&self);

    fn unselect_iter(&self, iter: &mut TreeIter);

    fn unselect_path(&self, path: &mut TreePath);

    fn unselect_range(&self, start_path: &mut TreePath, end_path: &mut TreePath);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeSelection>> TreeSelectionExt for O {
    fn count_selected_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_selection_count_selected_rows(self.as_ref().to_glib_none().0)
        }
    }

    fn get_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_tree_selection_get_mode(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_select_function(&self) -> Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>> {
    //    unsafe { TODO: call ffi::gtk_tree_selection_get_select_function() }
    //}

    fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_selection_get_selected(self.as_ref().to_glib_none().0, &mut model, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), iter)) } else { None }
        }
    }

    fn get_selected_rows(&self) -> (Vec<TreePath>, TreeModel) {
        unsafe {
            let mut model = ptr::null_mut();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::gtk_tree_selection_get_selected_rows(self.as_ref().to_glib_none().0, &mut model));
            (ret, from_glib_none(model))
        }
    }

    fn get_tree_view(&self) -> Option<TreeView> {
        unsafe {
            from_glib_none(ffi::gtk_tree_selection_get_tree_view(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gtk_tree_selection_get_user_data() }
    //}

    fn iter_is_selected(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_iter_is_selected(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn path_is_selected(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_path_is_selected(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0))
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_tree_selection_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_iter(&self, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_select_iter(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
        }
    }

    fn select_path(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_path(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0);
        }
    }

    fn select_range(&self, start_path: &mut TreePath, end_path: &mut TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_range(self.as_ref().to_glib_none().0, start_path.to_glib_none_mut().0, end_path.to_glib_none_mut().0);
        }
    }

    fn selected_foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&TreeModel, &TreePath, &TreeIter)>(model: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, data: glib_ffi::gpointer) {
            let model = from_glib_borrow(model);
            let path = from_glib_borrow(path);
            let iter = from_glib_borrow(iter);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&model, &path, &iter);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_tree_selection_selected_foreach(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn set_mode(&self, type_: SelectionMode) {
        unsafe {
            ffi::gtk_tree_selection_set_mode(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn set_select_function(&self, func: Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>) {
        let func_data: Box_<Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>> = Box::new(func);
        unsafe extern "C" fn func_func(selection: *mut ffi::GtkTreeSelection, model: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, path_currently_selected: glib_ffi::gboolean, data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let selection = from_glib_borrow(selection);
            let model = from_glib_borrow(model);
            let path = from_glib_borrow(path);
            let path_currently_selected = from_glib(path_currently_selected);
            let callback: &Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>> = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&selection, &model, &path, path_currently_selected)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let func = if func_data.is_some() { Some(func_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>> = func_data;
        unsafe {
            ffi::gtk_tree_selection_set_select_function(self.as_ref().to_glib_none().0, func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_tree_selection_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_iter(&self, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_unselect_iter(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
        }
    }

    fn unselect_path(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_path(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0);
        }
    }

    fn unselect_range(&self, start_path: &mut TreePath, end_path: &mut TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_range(self.as_ref().to_glib_none().0, start_path.to_glib_none_mut().0, end_path.to_glib_none_mut().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mode\0".as_ptr() as *const _,
                Some(transmute(notify_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkTreeSelection, f: glib_ffi::gpointer)
where P: IsA<TreeSelection> {
    let f: &F = &*(f as *const F);
    f(&TreeSelection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkTreeSelection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeSelection> {
    let f: &F = &*(f as *const F);
    f(&TreeSelection::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for TreeSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeSelection")
    }
}
