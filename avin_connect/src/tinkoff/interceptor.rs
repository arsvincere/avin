/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use tonic::Status;
use tonic::service::Interceptor;

#[derive(Debug, Clone)]
pub struct DefaultInterceptor {
    pub token: String,
}
impl Interceptor for DefaultInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, Status> {
        let mut req = request;
        req.metadata_mut().append(
            "authorization",
            format!("bearer {}", self.token).parse().unwrap(),
        );
        req.metadata_mut().append(
            "x-tracking-id",
            uuid::Uuid::new_v4().to_string().parse().unwrap(),
        );
        req.metadata_mut()
            .append("x-app-name", "arsvincere.avin".parse().unwrap());

        Ok(req)
    }
}
