use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentProfileGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub payment_profile_token: String,
}
impl<'a> PaymentProfileGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<PaymentProfileGetResponse> {
        let mut r = self.http_client.client.post("/payment_profile/get");
        r = r.json(json!({ "payment_profile_token" : self.payment_profile_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for PaymentProfileGetRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentProfileGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}