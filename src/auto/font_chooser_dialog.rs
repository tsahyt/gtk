// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use FontChooser;
use Widget;
use Window;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct FontChooserDialog(Object<ffi::GtkFontChooserDialog>): Dialog, Window, Bin, Container, Widget, FontChooser;

    match fn {
        get_type => || ffi::gtk_font_chooser_dialog_get_type(),
    }
}

impl FontChooserDialog {
    pub fn new<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>) -> FontChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_chooser_dialog_new(title.to_glib_none().0, parent.to_glib_none().0)).downcast_unchecked()
        }
    }
}
