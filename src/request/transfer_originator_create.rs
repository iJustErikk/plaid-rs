use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferOriginatorCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub company_name: String,
}
impl<'a> TransferOriginatorCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferOriginatorCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/originator/create");
        r = r.json(json!({ "company_name" : self.company_name }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for TransferOriginatorCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferOriginatorCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}