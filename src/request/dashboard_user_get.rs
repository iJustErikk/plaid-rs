use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DashboardUserGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub dashboard_user_id: String,
}
impl<'a> DashboardUserGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DashboardUserGetResponse> {
        let mut r = self.http_client.client.post("/dashboard_user/get");
        r = r.json(json!({ "dashboard_user_id" : self.dashboard_user_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DashboardUserGetRequest<'a> {
    type Output = httpclient::InMemoryResult<DashboardUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}