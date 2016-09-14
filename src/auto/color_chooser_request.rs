// This file was generated by gir (0d368d6) from gir-files (???)
// DO NOT EDIT

use ffi;
#[cfg(feature = "v2_8")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_8")]
use glib_ffi;
#[cfg(feature = "v2_8")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_8")]
use std::mem::transmute;

glib_wrapper! {
    pub struct ColorChooserRequest(Object<ffi::WebKitColorChooserRequest>);

    match fn {
        get_type => || ffi::webkit_color_chooser_request_get_type(),
    }
}

impl ColorChooserRequest {
    #[cfg(feature = "v2_8")]
    pub fn cancel(&self) {
        unsafe {
            ffi::webkit_color_chooser_request_cancel(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn finish(&self) {
        unsafe {
            ffi::webkit_color_chooser_request_finish(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_8")]
    //pub fn get_element_rectangle(&self, rect: /*Ignored*/gdk::Rectangle) {
    //    unsafe { TODO: call ffi::webkit_color_chooser_request_get_element_rectangle() }
    //}

    //#[cfg(feature = "v2_8")]
    //pub fn get_rgba(&self, rgba: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::webkit_color_chooser_request_get_rgba() }
    //}

    //#[cfg(feature = "v2_8")]
    //pub fn set_rgba(&self, rgba: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::webkit_color_chooser_request_set_rgba() }
    //}

    #[cfg(feature = "v2_8")]
    pub fn connect_finished<F: Fn(&ColorChooserRequest) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ColorChooserRequest) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn finished_trampoline(this: *mut ffi::WebKitColorChooserRequest, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ColorChooserRequest) + 'static> = transmute(f);
    f(&from_glib_none(this))
}