// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Fixed(Object<ffi::GtkFixed>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_fixed_get_type(),
    }
}

impl Fixed {
    pub fn new() -> Fixed {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_fixed_new()).downcast_unchecked()
        }
    }

    pub fn move_<T: IsA<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_move(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }

    pub fn put<T: IsA<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_put(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }
}
