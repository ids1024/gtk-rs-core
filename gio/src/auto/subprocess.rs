// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::InputStream;
use crate::OutputStream;
use crate::SubprocessFlags;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GSubprocess")]
    pub struct Subprocess(Object<ffi::GSubprocess>);

    match fn {
        type_ => || ffi::g_subprocess_get_type(),
    }
}

impl Subprocess {
    //#[doc(alias = "g_subprocess_new")]
    //pub fn new(flags: SubprocessFlags, error: Option<&mut glib::Error>, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Subprocess {
    //    unsafe { TODO: call ffi:g_subprocess_new() }
    //}

    #[doc(alias = "g_subprocess_newv")]
    pub fn newv(
        argv: &[&std::ffi::OsStr],
        flags: SubprocessFlags,
    ) -> Result<Subprocess, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_newv(argv.to_glib_none().0, flags.into_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_communicate")]
    pub fn communicate<P: IsA<Cancellable>>(
        &self,
        stdin_buf: Option<&glib::Bytes>,
        cancellable: Option<&P>,
    ) -> Result<(Option<glib::Bytes>, Option<glib::Bytes>), glib::Error> {
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate(
                self.to_glib_none().0,
                stdin_buf.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut stdout_buf,
                &mut stderr_buf,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_communicate_async")]
    pub fn communicate_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(Option<glib::Bytes>, Option<glib::Bytes>), glib::Error>) + Send + 'static,
    >(
        &self,
        stdin_buf: Option<&glib::Bytes>,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn communicate_async_trampoline<
            Q: FnOnce(Result<(Option<glib::Bytes>, Option<glib::Bytes>), glib::Error>)
                + Send
                + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_finish(
                _source_object as *mut _,
                res,
                &mut stdout_buf,
                &mut stderr_buf,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = communicate_async_trampoline::<Q>;
        unsafe {
            ffi::g_subprocess_communicate_async(
                self.to_glib_none().0,
                stdin_buf.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn communicate_async_future(
        &self,
        stdin_buf: Option<&glib::Bytes>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(Option<glib::Bytes>, Option<glib::Bytes>), glib::Error>,
                > + 'static,
        >,
    > {
        let stdin_buf = stdin_buf.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.communicate_async(
                    stdin_buf.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_subprocess_communicate_utf8")]
    pub fn communicate_utf8<P: IsA<Cancellable>>(
        &self,
        stdin_buf: Option<&str>,
        cancellable: Option<&P>,
    ) -> Result<(Option<glib::GString>, Option<glib::GString>), glib::Error> {
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_utf8(
                self.to_glib_none().0,
                stdin_buf.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut stdout_buf,
                &mut stderr_buf,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_force_exit")]
    pub fn force_exit(&self) {
        unsafe {
            ffi::g_subprocess_force_exit(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_subprocess_get_exit_status")]
    #[doc(alias = "get_exit_status")]
    pub fn exit_status(&self) -> i32 {
        unsafe { ffi::g_subprocess_get_exit_status(self.to_glib_none().0) }
    }

    #[doc(alias = "g_subprocess_get_identifier")]
    #[doc(alias = "get_identifier")]
    pub fn identifier(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_subprocess_get_identifier(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_if_exited")]
    #[doc(alias = "get_if_exited")]
    pub fn has_exited(&self) -> bool {
        unsafe { from_glib(ffi::g_subprocess_get_if_exited(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_if_signaled")]
    #[doc(alias = "get_if_signaled")]
    pub fn has_signaled(&self) -> bool {
        unsafe { from_glib(ffi::g_subprocess_get_if_signaled(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_status")]
    #[doc(alias = "get_status")]
    pub fn status(&self) -> i32 {
        unsafe { ffi::g_subprocess_get_status(self.to_glib_none().0) }
    }

    #[doc(alias = "g_subprocess_get_stderr_pipe")]
    #[doc(alias = "get_stderr_pipe")]
    pub fn stderr_pipe(&self) -> Option<InputStream> {
        unsafe { from_glib_none(ffi::g_subprocess_get_stderr_pipe(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_stdin_pipe")]
    #[doc(alias = "get_stdin_pipe")]
    pub fn stdin_pipe(&self) -> Option<OutputStream> {
        unsafe { from_glib_none(ffi::g_subprocess_get_stdin_pipe(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_stdout_pipe")]
    #[doc(alias = "get_stdout_pipe")]
    pub fn stdout_pipe(&self) -> Option<InputStream> {
        unsafe { from_glib_none(ffi::g_subprocess_get_stdout_pipe(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_successful")]
    #[doc(alias = "get_successful")]
    pub fn is_successful(&self) -> bool {
        unsafe { from_glib(ffi::g_subprocess_get_successful(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_subprocess_get_term_sig")]
    #[doc(alias = "get_term_sig")]
    pub fn term_sig(&self) -> i32 {
        unsafe { ffi::g_subprocess_get_term_sig(self.to_glib_none().0) }
    }

    #[cfg(any(not(windows), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(windows))))]
    #[doc(alias = "g_subprocess_send_signal")]
    pub fn send_signal(&self, signal_num: i32) {
        unsafe {
            ffi::g_subprocess_send_signal(self.to_glib_none().0, signal_num);
        }
    }

    #[doc(alias = "g_subprocess_wait")]
    pub fn wait<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_wait_async")]
    pub fn wait_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn wait_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_async_trampoline::<Q>;
        unsafe {
            ffi::g_subprocess_wait_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn wait_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.wait_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_subprocess_wait_check")]
    pub fn wait_check<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_check(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_wait_check_async")]
    pub fn wait_check_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn wait_check_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_check_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_check_async_trampoline::<Q>;
        unsafe {
            ffi::g_subprocess_wait_check_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn wait_check_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.wait_check_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl fmt::Display for Subprocess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Subprocess")
    }
}
