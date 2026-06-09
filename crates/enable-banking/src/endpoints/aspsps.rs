use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};

use crate::{
    endpoints::Endpoint,
    schemas::{
        aspsp_data::ASPSPData, payment_type::PaymentType, psu_type::PSUType, service::Service,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetAspsps {
    pub country: Option<String>,
    pub psu_type: Option<PSUType>,
    pub service: Option<Service>,
    pub payment_type: Option<PaymentType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAspspsResponse {
    pub aspsps: Vec<ASPSPData>,
}

impl Endpoint for GetAspsps {
    const METHOD: Method = Method::GET;

    type Response = GetAspspsResponse;

    fn build_url(&self, mut base_url: Url) -> Url {
        base_url.set_path("aspsps");

        {
            let query_map = serde_json::to_value(self).unwrap();
            let query_map = query_map.as_object().unwrap();
            let mut query = base_url.query_pairs_mut();

            for (key, value) in query_map {
                if !value.is_null() {
                    query.append_pair(key, value.to_string().trim_matches('"'));
                }
            }

            query.finish();
        }

        base_url
    }
}
