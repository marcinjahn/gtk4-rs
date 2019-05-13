// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use WidgetAccessible;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct FlowBoxAccessible(Object<ffi::GtkFlowBoxAccessible, ffi::GtkFlowBoxAccessibleClass, FlowBoxAccessibleClass>) @extends ContainerAccessible, WidgetAccessible, Accessible;

    match fn {
        get_type => || ffi::gtk_flow_box_accessible_get_type(),
    }
}

impl FlowBoxAccessible {}

pub const NONE_FLOW_BOX_ACCESSIBLE: Option<&FlowBoxAccessible> = None;

impl fmt::Display for FlowBoxAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlowBoxAccessible")
    }
}
