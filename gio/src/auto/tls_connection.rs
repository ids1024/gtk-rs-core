// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::IOStream;
use crate::TlsCertificate;
use crate::TlsCertificateFlags;
use crate::TlsDatabase;
use crate::TlsInteraction;
use crate::TlsRehandshakeMode;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GTlsConnection")]
    pub struct TlsConnection(Object<ffi::GTlsConnection, ffi::GTlsConnectionClass>) @extends IOStream;

    match fn {
        type_ => || ffi::g_tls_connection_get_type(),
    }
}

pub const NONE_TLS_CONNECTION: Option<&TlsConnection> = None;

pub trait TlsConnectionExt: 'static {
    #[doc(alias = "g_tls_connection_emit_accept_certificate")]
    fn emit_accept_certificate<P: IsA<TlsCertificate>>(
        &self,
        peer_cert: &P,
        errors: TlsCertificateFlags,
    ) -> bool;

    #[doc(alias = "g_tls_connection_get_certificate")]
    #[doc(alias = "get_certificate")]
    fn certificate(&self) -> Option<TlsCertificate>;

    #[doc(alias = "g_tls_connection_get_database")]
    #[doc(alias = "get_database")]
    fn database(&self) -> Option<TlsDatabase>;

    #[doc(alias = "g_tls_connection_get_interaction")]
    #[doc(alias = "get_interaction")]
    fn interaction(&self) -> Option<TlsInteraction>;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_tls_connection_get_negotiated_protocol")]
    #[doc(alias = "get_negotiated_protocol")]
    fn negotiated_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_tls_connection_get_peer_certificate")]
    #[doc(alias = "get_peer_certificate")]
    fn peer_certificate(&self) -> Option<TlsCertificate>;

    #[doc(alias = "g_tls_connection_get_peer_certificate_errors")]
    #[doc(alias = "get_peer_certificate_errors")]
    fn peer_certificate_errors(&self) -> TlsCertificateFlags;

    #[cfg_attr(feature = "v2_60", deprecated = "Since 2.60")]
    #[doc(alias = "g_tls_connection_get_rehandshake_mode")]
    #[doc(alias = "get_rehandshake_mode")]
    fn rehandshake_mode(&self) -> TlsRehandshakeMode;

    #[doc(alias = "g_tls_connection_get_require_close_notify")]
    #[doc(alias = "get_require_close_notify")]
    fn requires_close_notify(&self) -> bool;

    #[doc(alias = "g_tls_connection_handshake")]
    fn handshake<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    #[doc(alias = "g_tls_connection_handshake_async")]
    fn handshake_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn handshake_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_tls_connection_set_advertised_protocols")]
    fn set_advertised_protocols(&self, protocols: &[&str]);

    #[doc(alias = "g_tls_connection_set_certificate")]
    fn set_certificate<P: IsA<TlsCertificate>>(&self, certificate: &P);

    #[doc(alias = "g_tls_connection_set_database")]
    fn set_database<P: IsA<TlsDatabase>>(&self, database: Option<&P>);

    #[doc(alias = "g_tls_connection_set_interaction")]
    fn set_interaction<P: IsA<TlsInteraction>>(&self, interaction: Option<&P>);

    #[cfg_attr(feature = "v2_60", deprecated = "Since 2.60")]
    #[doc(alias = "g_tls_connection_set_rehandshake_mode")]
    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode);

    #[doc(alias = "g_tls_connection_set_require_close_notify")]
    fn set_require_close_notify(&self, require_close_notify: bool);

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "advertised-protocols")]
    fn advertised_protocols(&self) -> Vec<glib::GString>;

    #[doc(alias = "base-io-stream")]
    fn base_io_stream(&self) -> Option<IOStream>;

    #[doc(alias = "accept-certificate")]
    fn connect_accept_certificate<
        F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "advertised-protocols")]
    fn connect_advertised_protocols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "certificate")]
    fn connect_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "database")]
    fn connect_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "interaction")]
    fn connect_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "negotiated-protocol")]
    fn connect_negotiated_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "peer-certificate")]
    fn connect_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "peer-certificate-errors")]
    fn connect_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg_attr(feature = "v2_60", deprecated = "Since 2.60")]
    #[doc(alias = "rehandshake-mode")]
    fn connect_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "require-close-notify")]
    fn connect_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsConnection>> TlsConnectionExt for O {
    fn emit_accept_certificate<P: IsA<TlsCertificate>>(
        &self,
        peer_cert: &P,
        errors: TlsCertificateFlags,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_tls_connection_emit_accept_certificate(
                self.as_ref().to_glib_none().0,
                peer_cert.as_ref().to_glib_none().0,
                errors.into_glib(),
            ))
        }
    }

    fn certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn database(&self) -> Option<TlsDatabase> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn interaction(&self) -> Option<TlsInteraction> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_interaction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn negotiated_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_negotiated_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peer_certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_peer_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peer_certificate_errors(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_tls_connection_get_peer_certificate_errors(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn rehandshake_mode(&self) -> TlsRehandshakeMode {
        unsafe {
            from_glib(ffi::g_tls_connection_get_rehandshake_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn requires_close_notify(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_connection_get_require_close_notify(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn handshake<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_tls_connection_handshake(
                self.as_ref().to_glib_none().0,
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

    fn handshake_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn handshake_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_tls_connection_handshake_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = handshake_async_trampoline::<Q>;
        unsafe {
            ffi::g_tls_connection_handshake_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn handshake_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.handshake_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn set_advertised_protocols(&self, protocols: &[&str]) {
        unsafe {
            ffi::g_tls_connection_set_advertised_protocols(
                self.as_ref().to_glib_none().0,
                protocols.to_glib_none().0,
            );
        }
    }

    fn set_certificate<P: IsA<TlsCertificate>>(&self, certificate: &P) {
        unsafe {
            ffi::g_tls_connection_set_certificate(
                self.as_ref().to_glib_none().0,
                certificate.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_database<P: IsA<TlsDatabase>>(&self, database: Option<&P>) {
        unsafe {
            ffi::g_tls_connection_set_database(
                self.as_ref().to_glib_none().0,
                database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_interaction<P: IsA<TlsInteraction>>(&self, interaction: Option<&P>) {
        unsafe {
            ffi::g_tls_connection_set_interaction(
                self.as_ref().to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode) {
        unsafe {
            ffi::g_tls_connection_set_rehandshake_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    fn set_require_close_notify(&self, require_close_notify: bool) {
        unsafe {
            ffi::g_tls_connection_set_require_close_notify(
                self.as_ref().to_glib_none().0,
                require_close_notify.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn advertised_protocols(&self) -> Vec<glib::GString> {
        unsafe {
            let mut value =
                glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"advertised-protocols\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `advertised-protocols` getter")
        }
    }

    fn base_io_stream(&self) -> Option<IOStream> {
        unsafe {
            let mut value = glib::Value::from_type(<IOStream as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"base-io-stream\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `base-io-stream` getter")
        }
    }

    #[doc(alias = "accept-certificate")]
    fn connect_accept_certificate<
        F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_certificate_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P, &TlsCertificate, TlsCertificateFlags) -> bool + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            peer_cert: *mut ffi::GTlsCertificate,
            errors: ffi::GTlsCertificateFlags,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &TlsConnection::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(peer_cert),
                from_glib(errors),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-certificate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "advertised-protocols")]
    fn connect_advertised_protocols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_advertised_protocols_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::advertised-protocols\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_advertised_protocols_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "certificate")]
    fn connect_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_certificate_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::certificate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "database")]
    fn connect_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_database_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::database\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_database_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "interaction")]
    fn connect_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interaction_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interaction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interaction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "negotiated-protocol")]
    fn connect_negotiated_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_negotiated_protocol_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::negotiated-protocol\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_negotiated_protocol_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "peer-certificate")]
    fn connect_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_certificate_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::peer-certificate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peer_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "peer-certificate-errors")]
    fn connect_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_certificate_errors_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::peer-certificate-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peer_certificate_errors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "rehandshake-mode")]
    fn connect_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rehandshake_mode_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rehandshake-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rehandshake_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "require-close-notify")]
    fn connect_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_require_close_notify_trampoline<
            P: IsA<TlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::require-close-notify\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_require_close_notify_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TlsConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsConnection")
    }
}
