// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::SectionModel;
use crate::Sorter;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
glib::wrapper! {
    #[doc(alias = "GtkSortListModel")]
    pub struct SortListModel(Object<ffi::GtkSortListModel, ffi::GtkSortListModelClass>) @implements gio::ListModel, SectionModel;

    match fn {
        type_ => || ffi::gtk_sort_list_model_get_type(),
    }
}

#[cfg(not(any(feature = "v4_12")))]
glib::wrapper! {
    #[doc(alias = "GtkSortListModel")]
    pub struct SortListModel(Object<ffi::GtkSortListModel, ffi::GtkSortListModelClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::gtk_sort_list_model_get_type(),
    }
}

impl SortListModel {
    #[doc(alias = "gtk_sort_list_model_new")]
    pub fn new(
        model: Option<impl IsA<gio::ListModel>>,
        sorter: Option<impl IsA<Sorter>>,
    ) -> SortListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_sort_list_model_new(
                model.map(|p| p.upcast()).into_glib_ptr(),
                sorter.map(|p| p.upcast()).into_glib_ptr(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SortListModel`] objects.
    ///
    /// This method returns an instance of [`SortListModelBuilder`](crate::builders::SortListModelBuilder) which can be used to create [`SortListModel`] objects.
    pub fn builder() -> SortListModelBuilder {
        SortListModelBuilder::new()
    }

    #[doc(alias = "gtk_sort_list_model_get_incremental")]
    #[doc(alias = "get_incremental")]
    pub fn is_incremental(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_sort_list_model_get_incremental(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_sort_list_model_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_sort_list_model_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_sort_list_model_get_pending")]
    #[doc(alias = "get_pending")]
    pub fn pending(&self) -> u32 {
        unsafe { ffi::gtk_sort_list_model_get_pending(self.to_glib_none().0) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_sort_list_model_get_section_sorter")]
    #[doc(alias = "get_section_sorter")]
    pub fn section_sorter(&self) -> Option<Sorter> {
        unsafe {
            from_glib_none(ffi::gtk_sort_list_model_get_section_sorter(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_sort_list_model_get_sorter")]
    #[doc(alias = "get_sorter")]
    pub fn sorter(&self) -> Option<Sorter> {
        unsafe { from_glib_none(ffi::gtk_sort_list_model_get_sorter(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_sort_list_model_set_incremental")]
    pub fn set_incremental(&self, incremental: bool) {
        unsafe {
            ffi::gtk_sort_list_model_set_incremental(
                self.to_glib_none().0,
                incremental.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_sort_list_model_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::gtk_sort_list_model_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_sort_list_model_set_section_sorter")]
    pub fn set_section_sorter(&self, sorter: Option<&impl IsA<Sorter>>) {
        unsafe {
            ffi::gtk_sort_list_model_set_section_sorter(
                self.to_glib_none().0,
                sorter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_sort_list_model_set_sorter")]
    pub fn set_sorter(&self, sorter: Option<&impl IsA<Sorter>>) {
        unsafe {
            ffi::gtk_sort_list_model_set_sorter(
                self.to_glib_none().0,
                sorter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "incremental")]
    pub fn connect_incremental_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_incremental_trampoline<F: Fn(&SortListModel) + 'static>(
            this: *mut ffi::GtkSortListModel,
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
                b"notify::incremental\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_incremental_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&SortListModel) + 'static>(
            this: *mut ffi::GtkSortListModel,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pending")]
    pub fn connect_pending_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pending_trampoline<F: Fn(&SortListModel) + 'static>(
            this: *mut ffi::GtkSortListModel,
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
                b"notify::pending\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pending_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "section-sorter")]
    pub fn connect_section_sorter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_section_sorter_trampoline<F: Fn(&SortListModel) + 'static>(
            this: *mut ffi::GtkSortListModel,
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
                b"notify::section-sorter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_section_sorter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sorter")]
    pub fn connect_sorter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sorter_trampoline<F: Fn(&SortListModel) + 'static>(
            this: *mut ffi::GtkSortListModel,
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
                b"notify::sorter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sorter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SortListModel {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SortListModel`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SortListModelBuilder {
    builder: glib::object::ObjectBuilder<'static, SortListModel>,
}

impl SortListModelBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn incremental(self, incremental: bool) -> Self {
        Self {
            builder: self.builder.property("incremental", incremental),
        }
    }

    pub fn model(self, model: &impl IsA<gio::ListModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn section_sorter(self, section_sorter: &impl IsA<Sorter>) -> Self {
        Self {
            builder: self
                .builder
                .property("section-sorter", section_sorter.clone().upcast()),
        }
    }

    pub fn sorter(self, sorter: &impl IsA<Sorter>) -> Self {
        Self {
            builder: self.builder.property("sorter", sorter.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SortListModel`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SortListModel {
        self.builder.build()
    }
}
