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
        let auth: MetadataValue<Ascii> = format!("Bearer {token}")
            .parse()
            .map_err(|err| ConnectorError::Authorization {
                message: "failed to encode T-Bank authorization token".into(),
                source: Some(Box::new(err)),
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
