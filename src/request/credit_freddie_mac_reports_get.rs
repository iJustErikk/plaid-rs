use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditFreddieMacReportsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> CreditFreddieMacReportsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreditFreddieMacReportsGetResponse> {
        let mut r = self.http_client.client.post("/credit/freddie_mac/reports/get");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreditFreddieMacReportsGetRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditFreddieMacReportsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}