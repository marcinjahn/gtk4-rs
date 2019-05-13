// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TreeIter;
use TreeModelFlags;
use TreePath;
use ffi;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct TreeModel(Interface<ffi::GtkTreeModel>);

    match fn {
        get_type => || ffi::gtk_tree_model_get_type(),
    }
}

pub const NONE_TREE_MODEL: Option<&TreeModel> = None;

pub trait TreeModelExt: 'static {
    fn foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter) -> bool>(&self, func: P);

    //fn get(&self, iter: &mut TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_column_type(&self, index_: i32) -> glib::types::Type;

    fn get_flags(&self) -> TreeModelFlags;

    fn get_iter(&self, path: &mut TreePath) -> Option<TreeIter>;

    fn get_iter_first(&self) -> Option<TreeIter>;

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter>;

    fn get_n_columns(&self) -> i32;

    fn get_path(&self, iter: &mut TreeIter) -> Option<TreePath>;

    fn get_string_from_iter(&self, iter: &mut TreeIter) -> Option<GString>;

    //fn get_valist(&self, iter: &mut TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn get_value(&self, iter: &mut TreeIter, column: i32, value: /*Ignored*/glib::Value);

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter>;

    fn iter_has_child(&self, iter: &mut TreeIter) -> bool;

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;

    fn iter_next(&self, iter: &mut TreeIter) -> bool;

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter>;

    fn iter_parent(&self, child: &mut TreeIter) -> Option<TreeIter>;

    fn iter_previous(&self, iter: &mut TreeIter) -> bool;

    fn ref_node(&self, iter: &mut TreeIter);

    fn row_changed(&self, path: &mut TreePath, iter: &mut TreeIter);

    fn row_deleted(&self, path: &mut TreePath);

    fn row_has_child_toggled(&self, path: &mut TreePath, iter: &mut TreeIter);

    fn row_inserted(&self, path: &mut TreePath, iter: &mut TreeIter);

    fn sort_new_with_model(&self) -> Option<TreeModel>;

    fn unref_node(&self, iter: &mut TreeIter);

    fn connect_row_changed<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_deleted<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_has_child_toggled<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_inserted<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_rows_reordered<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeModel>> TreeModelExt for O {
    fn foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter) -> bool>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&TreeModel, &TreePath, &TreeIter) -> bool>(model: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let model = from_glib_borrow(model);
            let path = from_glib_borrow(path);
            let iter = from_glib_borrow(iter);
            let callback: *mut P = data as *const _ as usize as *mut P;
            let res = (*callback)(&model, &path, &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_tree_model_foreach(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }

    //fn get(&self, iter: &mut TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get() }
    //}

    fn get_column_type(&self, index_: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_column_type(self.as_ref().to_glib_none().0, index_))
        }
    }

    fn get_flags(&self) -> TreeModelFlags {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_flags(self.as_ref().to_glib_none().0))
        }
    }

    fn get_iter(&self, path: &mut TreePath) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, path.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_iter_first(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_first(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_from_string(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, path_string.to_glib_none().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_n_columns(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_model_get_n_columns(self.as_ref().to_glib_none().0)
        }
    }

    fn get_path(&self, iter: &mut TreeIter) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_path(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn get_string_from_iter(&self, iter: &mut TreeIter) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_string_from_iter(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    //fn get_valist(&self, iter: &mut TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get_valist() }
    //}

    //fn get_value(&self, iter: &mut TreeIter, column: i32, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get_value() }
    //}

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_children(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_has_child(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_has_child(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0))
        }
    }

    fn iter_next(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_next(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_nth_child(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), n));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_parent(&self, child: &mut TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_parent(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, child.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_previous(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn ref_node(&self, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_model_ref_node(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
        }
    }

    fn row_changed(&self, path: &mut TreePath, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_changed(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0, iter.to_glib_none_mut().0);
        }
    }

    fn row_deleted(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_tree_model_row_deleted(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0);
        }
    }

    fn row_has_child_toggled(&self, path: &mut TreePath, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_has_child_toggled(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0, iter.to_glib_none_mut().0);
        }
    }

    fn row_inserted(&self, path: &mut TreePath, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_inserted(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0, iter.to_glib_none_mut().0);
        }
    }

    fn sort_new_with_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_sort_new_with_model(self.as_ref().to_glib_none().0))
        }
    }

    fn unref_node(&self, iter: &mut TreeIter) {
        unsafe {
            ffi::gtk_tree_model_unref_node(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
        }
    }

    fn connect_row_changed<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-changed\0".as_ptr() as *const _,
                Some(transmute(row_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_deleted<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-deleted\0".as_ptr() as *const _,
                Some(transmute(row_deleted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_has_child_toggled<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-has-child-toggled\0".as_ptr() as *const _,
                Some(transmute(row_has_child_toggled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_inserted<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-inserted\0".as_ptr() as *const _,
                Some(transmute(row_inserted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    //fn connect_rows_reordered<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented new_order: *.Pointer
    //}
}

unsafe extern "C" fn row_changed_trampoline<P, F: Fn(&P, &TreePath, &TreeIter) + 'static>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where P: IsA<TreeModel> {
    let f: &F = &*(f as *const F);
    f(&TreeModel::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(path), &from_glib_borrow(iter))
}

unsafe extern "C" fn row_deleted_trampoline<P, F: Fn(&P, &TreePath) + 'static>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer)
where P: IsA<TreeModel> {
    let f: &F = &*(f as *const F);
    f(&TreeModel::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(path))
}

unsafe extern "C" fn row_has_child_toggled_trampoline<P, F: Fn(&P, &TreePath, &TreeIter) + 'static>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where P: IsA<TreeModel> {
    let f: &F = &*(f as *const F);
    f(&TreeModel::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(path), &from_glib_borrow(iter))
}

unsafe extern "C" fn row_inserted_trampoline<P, F: Fn(&P, &TreePath, &TreeIter) + 'static>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where P: IsA<TreeModel> {
    let f: &F = &*(f as *const F);
    f(&TreeModel::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(path), &from_glib_borrow(iter))
}

impl fmt::Display for TreeModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeModel")
    }
}
