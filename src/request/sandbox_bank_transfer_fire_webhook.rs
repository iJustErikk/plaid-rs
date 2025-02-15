use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxBankTransferFireWebhookRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub webhook: String,
}
impl<'a> SandboxBankTransferFireWebhookRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SandboxBankTransferFireWebhookResponse> {
        let mut r = self.http_client.client.post("/sandbox/bank_transfer/fire_webhook");
        r = r.json(json!({ "webhook" : self.webhook }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for SandboxBankTransferFireWebhookRequest<'a> {
    type Output = httpclient::InMemoryResult<SandboxBankTransferFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}