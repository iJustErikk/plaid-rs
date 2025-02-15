use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditSessionsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditSessionsGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreditSessionsGetResponse> {
        let mut r = self.http_client.client.post("/credit/sessions/get");
        r = r.json(json!({ "user_token" : self.user_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreditSessionsGetRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditSessionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}