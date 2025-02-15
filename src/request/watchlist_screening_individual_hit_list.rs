use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualHitListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub cursor: Option<String>,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualHitListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WatchlistScreeningIndividualHitListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/hit/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = r.json(json!({ "watchlist_screening_id" : self.watchlist_screening_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualHitListRequest<'a> {
    type Output = httpclient::InMemoryResult<
        WatchlistScreeningIndividualHitListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}