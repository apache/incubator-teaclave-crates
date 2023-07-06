use crate::transport::{
    service::TlsAcceptor,
    tls::{Certificate, Identity},
};
use std::fmt;

/// Configures TLS settings for servers.
#[derive(Clone, Default)]
pub struct ServerTlsConfig {
    identity: Option<Identity>,
    client_ca_root: Option<Certificate>,
    client_auth_optional: bool,
    rustls_raw: Option<tokio_rustls::rustls::ServerConfig>,
}

impl fmt::Debug for ServerTlsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerTlsConfig").finish()
    }
}

impl ServerTlsConfig {
    /// Creates a new `ServerTlsConfig`.
    pub fn new() -> Self {
        ServerTlsConfig {
            identity: None,
            client_ca_root: None,
            client_auth_optional: false,
            rustls_raw: None,
        }
    }

    /// Sets the [`Identity`] of the server.
    pub fn identity(self, identity: Identity) -> Self {
        ServerTlsConfig {
            identity: Some(identity),
            ..self
        }
    }

    /// Sets a certificate against which to validate client TLS certificates.
    pub fn client_ca_root(self, cert: Certificate) -> Self {
        ServerTlsConfig {
            client_ca_root: Some(cert),
            ..self
        }
    }

    /// Sets whether client certificate verification is optional.
    ///
    /// This option has effect only if CA certificate is set.
    ///
    /// # Default
    /// By default, this option is set to `false`.
    pub fn client_auth_optional(self, optional: bool) -> Self {
        ServerTlsConfig {
            client_auth_optional: optional,
            ..self
        }
    }

    /// Use options specified by the given `ServerConfig` to configure TLS.
    ///
    /// This overrides all other TLS options set via other means.
    pub fn rustls_server_config(
        &mut self,
        config: tokio_rustls::rustls::ServerConfig,
    ) -> &mut Self {
        self.rustls_raw = Some(config);
        self
    }

    pub(crate) fn tls_acceptor(&self) -> Result<TlsAcceptor, crate::Error> {

        match &self.rustls_raw {
            None => TlsAcceptor::new(
                self.identity.clone().unwrap(),
                self.client_ca_root.clone(),
                self.client_auth_optional,
            ),
            Some(config) => TlsAcceptor::new_with_rustls_raw(config.clone()),
        }

        
    }
}
