//! The Veracruz proxy attestation server
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Licensing and copyright notice
//!
//! See the `LICENSE_MIT.markdown` file in the Veracruz root directory for
//! information on licensing and copyright.

use crate::attestation;
#[cfg(feature = "nitro")]
use crate::attestation::nitro;
#[cfg(any(feature = "linux", feature = "icecap"))]
use crate::attestation::psa;
use crate::error::*;
use actix_web::{dev::Server, middleware, web, App, HttpServer};
use lazy_static::lazy_static;
use std::{
    net::ToSocketAddrs,
    path,
    sync::atomic::{AtomicBool, Ordering},
};

lazy_static! {
    pub static ref DEBUG_MODE: AtomicBool = AtomicBool::new(false);
}

#[allow(unused)]
async fn psa_router(
    psa_request: web::Path<String>,
    input_data: String,
) -> ProxyAttestationServerResponder {
    #[cfg(any(feature = "linux", feature = "icecap"))]
    if psa_request.into_inner().as_str() == "AttestationToken" {
        psa::attestation_token(input_data)
    } else {
        Err(ProxyAttestationServerError::UnsupportedRequestError)
    }
    #[cfg(not(any(feature = "linux", feature = "icecap")))]
    Err(ProxyAttestationServerError::UnimplementedRequestError)
}

#[allow(unused)]
async fn nitro_router(
    nitro_request: web::Path<String>,
    input_data: String,
) -> ProxyAttestationServerResponder {
    #[cfg(feature = "nitro")]
    {
        let inner = nitro_request.into_inner();
        if inner.as_str() == "AttestationToken" {
            nitro::attestation_token(input_data)
        } else {
            println!(
                "proxy-attestation-server::nitro_router returning unsupported with into_inner:{:?}",
                inner.as_str()
            );
            Err(ProxyAttestationServerError::UnsupportedRequestError)
        }
    }
    #[cfg(not(feature = "nitro"))]
    Err(ProxyAttestationServerError::UnimplementedRequestError)
}

pub fn server<U, P1, P2>(
    url: U,
    ca_cert_path: P1,
    ca_key_path: P2,
    debug: bool,
) -> Result<Server, String>
where
    U: ToSocketAddrs,
    P1: AsRef<path::Path>,
    P2: AsRef<path::Path>,
{
    if debug {
        DEBUG_MODE.store(true, Ordering::SeqCst);
    }
    crate::attestation::load_ca_certificate(ca_cert_path).map_err(|err| {
        format!(
            "proxy-attestation-server::server::server load_ca_certificate returned an error:{:?}",
            err
        )
    })?;
    crate::attestation::load_ca_key(ca_key_path).map_err(|err| {
        format!(
            "proxy-attestation-server::server::server load_ca_key returned an error:{:?}",
            err
        )
    })?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/Start", web::post().to(attestation::start))
            .route("/PSA/{psa_request}", web::post().to(psa_router))
            .route("/Nitro/{nitro_request}", web::post().to(nitro_router))
    })
    .bind(url)
    .map_err(|err| format!("binding error: {:?}", err))?
    .run();
    Ok(server)
}
