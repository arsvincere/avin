// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Debug, Clone)]
pub struct TBankInterceptor {
    authorization: MetadataValue<Ascii>,
    app_name: MetadataValue<Ascii>,
}

impl TBankInterceptor {
    pub fn new(
        token: &str,
    ) -> Result<Self, tonic::metadata::errors::InvalidMetadataValue> {
        let authorization = format!("Bearer {token}").parse()?;
        let app_name = MetadataValue::from_static("arsvincere.avin");

        Ok(Self {
            authorization,
            app_name,
        })
    }
}

impl Interceptor for TBankInterceptor {
    fn call(
        &mut self,
        mut request: Request<()>,
    ) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.authorization.clone());

        request
            .metadata_mut()
            .insert("x-app-name", self.app_name.clone());

        Ok(request)
    }
}
