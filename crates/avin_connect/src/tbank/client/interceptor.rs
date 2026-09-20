// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

use crate::ConnectorError;

#[derive(Debug, Clone)]
pub(super) struct TBankInterceptor {
    auth: MetadataValue<Ascii>,
    app_name: MetadataValue<Ascii>,
}

impl TBankInterceptor {
    pub(super) fn new(token: &str) -> Result<Self, ConnectorError> {
        let auth = format!("Bearer {token}")
            .parse::<MetadataValue<Ascii>>()
            .map_err(|err| {
                let msg = "failed to encode T-Bank authorization token";
                ConnectorError::authorization(msg, Some(err.into()))
            })?;

        let app_name = MetadataValue::from_static("arsvincere.avin");

        Ok(Self { auth, app_name })
    }
}

impl Interceptor for TBankInterceptor {
    fn call(
        &mut self,
        mut request: Request<()>,
    ) -> Result<Request<()>, Status> {
        let meta = request.metadata_mut();

        meta.insert("authorization", self.auth.clone());
        meta.insert("x-app-name", self.app_name.clone());

        Ok(request)
    }
}
