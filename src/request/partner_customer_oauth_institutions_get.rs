use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PartnerCustomerOauthInstitutionsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_id: Option<String>,
    pub end_customer_client_id: String,
    pub secret: Option<String>,
}
impl<'a> PartnerCustomerOauthInstitutionsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PartnerCustomerOAuthInstitutionsGetResponse> {
        let mut r = self
            .http_client
            .client
            .post("/partner/customer/oauth_institutions/get");
        if let Some(ref unwrapped) = self.client_id {
            r = r.json(json!({ "client_id" : unwrapped }));
        }
        r = r.json(json!({ "end_customer_client_id" : self.end_customer_client_id }));
        if let Some(ref unwrapped) = self.secret {
            r = r.json(json!({ "secret" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_owned());
        self
    }
    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = Some(secret.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for PartnerCustomerOauthInstitutionsGetRequest<'a> {
    type Output = httpclient::InMemoryResult<
        PartnerCustomerOAuthInstitutionsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}