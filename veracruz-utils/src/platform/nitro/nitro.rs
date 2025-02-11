//! Structs needed for AWS Nitro Enclaves, both inside and outside of the
//! enclave
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Licensing and copyright notice
//!
//! See the `LICENSE_MIT.markdown` file in the Veracruz root directory for
//! information on licensing and copyright.

use serde::{Deserialize, Serialize};

use crate::runtime_manager_message::Status;

/// An enumerated type describing messages passed between to/from the Runtime
/// Manager enclave (These originate from the Untrusted Pass-through (Veracruz
/// server)
/// These messages are inteded to be serialized using bincode before transport,
/// and deserialized using bincode after transport
#[derive(Serialize, Deserialize, Debug)]
pub enum NitroRootEnclaveMessage {
    /// A message generated by an operation that did not return data, but did
    /// return a status.
    /// Most operations return data, but if they fail, they will return a
    /// status set to `Status::Fail` (or `Status::Unimplemented` if
    /// it is not implmeneted).
    /// Parameters:
    /// Status - the Status
    Status(Status),
    /// A request to fetch the firmware version from the Nitro Root Enclave
    FetchFirmwareVersion,
    /// A response to the `FetchFirmwareVersion` message, it contains the
    /// firmware version of the Nitro Root Enclave, as a string
    FirmwareVersion(String),
    /// A request to set the certificate chain for the Root Enclave
    SetCertChain(Vec<u8>, Vec<u8>),
    /// A request to start the native attestation process.
    /// This is usually initiated from the Proxy Attestation Service
    /// The values:
    /// Vec<u8> - The 128-bit challenge value generated by the caller
    /// i32     - A device ID set by the caller. Will be used by the enclave
    ///           in future operations
    NativeAttestation(Vec<u8>, i32),
    /// A response to the NativeAttestation message. This is generated by the
    /// enclave.
    /// The parameters:
    /// Vec<u8> - The native attestation token generated by the enclave
    /// Vec<u8> - The Certificate Signing Request (CSR), generated by the root
    ///           enclave, to be used by the proxy service to generate the
    ///           Root Enclave Certificate
    TokenData(Vec<u8>, Vec<u8>),
    /// A request to start the proxy attestation process for the caller. This
    /// request will result in a `ChallengeData` response.
    StartProxy,
    /// A response to the `StartProxy` message.
    /// Vec<u8> - The 128-bit challenge value generated by the root enclave
    /// i32     - The challenge ID generated by the root enclave to match the
    ///           challenge to future requests
    ChallengeData(Vec<u8>, i32),
    /// A request (initiated by the Runtime Manager enclave) to start the
    /// proxy attestation process.
    /// The parameters:
    /// Vec<u8> - The native attestation document value, generated by the
    ///           caller.
    /// i32     - The challenge ID value received in the `ChallengeData`
    ///           message letting the root enclave know which challenge value
    ///           to check for in the token
    ProxyAttestation(Vec<u8>, i32),
    /// A response to the ProxyAttestation message. This is the certificate that
    /// the compute enclave will send to it's clients.
    /// The parameters:
    /// Vec<u8> - the compute enclave certificate
    /// Vec<u8> - The root enclave certificate
    /// Vec<u8> - the CA root certificate
    CertChain(Vec<Vec<u8>>),
    /// A successful response to a request that just contains a status
    /// (for example, a response to a SetCertChain request)
    Success,
}
