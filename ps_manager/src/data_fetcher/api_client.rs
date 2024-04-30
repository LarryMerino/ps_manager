use crate::{
    data::ps::{
        order_invoice::{OrderInvoice, OrderInvoiceData, OrderInvoiceId, OrderInvoices},
        PsId,
    },
    data_fetcher::DataResource,
    settings::ApiCredentialsProvider,
};

use super::{DataFetcher, InvoiceFetcher, Resource};

use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, AUTHORIZATION};

#[derive(Debug)]
struct Host {
    host: String,
}

impl Host {
    fn new(host: String) -> Self {
        Self { host }
    }
}

impl Into<String> for Host {
    fn into(self) -> String {
        self.host
    }
}

impl ToString for Host {
    fn to_string(&self) -> String {
        self.host.clone()
    }
}

#[derive(Debug)]
struct Key {
    raw_key: String,
}

impl Key {
    fn new(raw_key: String) -> Self {
        Self { raw_key }
    }
    #[inline]
    fn as_base64(&self) -> String {
        general_purpose::STANDARD_NO_PAD.encode(&self.raw_key)
    }
}

/// By default format_type is FormatType::Json
#[derive(Debug)]
pub struct ApiDataFetcherBuilder {
    host: String,
    key: String,
    format_type: FormatType,
}

impl ApiDataFetcherBuilder {
    pub fn format(mut self, format_type: FormatType) -> Self {
        self.format_type = format_type;
        self
    }
    pub fn build(self) -> ApiDataFetcher {
        ApiDataFetcher {
            host: Host::new(self.host),
            key: Key::new(self.key),
            format_type: self.format_type,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Debug)]
pub struct ApiDataFetcher {
    host: Host,
    key: Key,
    format_type: FormatType,
    client: reqwest::Client,
}

impl ApiDataFetcher {
    pub fn new(credentials: &impl ApiCredentialsProvider) -> ApiDataFetcherBuilder {
        ApiDataFetcherBuilder {
            host: credentials.get_host(),
            key: credentials.get_raw_key(),
            format_type: FormatType::Json,
        }
    }

    fn get_end_point(&self, resource: Resource, id: Option<impl PsId>) -> String {
        match id {
            Some(id) => format!(
                "{}/{}/{}/{}",
                self.host.to_string(),
                resource.as_api(),
                id.get_id(),
                self.format_type.to_string()
            ),
            None => format!(
                "{}/{}/{}",
                self.host.to_string(),
                resource.as_api(),
                self.format_type.to_string()
            ),
        }
    }

    #[inline]
    fn get_headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.get_auth_header_value().parse().unwrap());
        map
    }

    #[inline]
    fn get_auth_header_value(&self) -> String {
        format!("basic {}:", self.key.as_base64())
    }
}

impl DataFetcher for ApiDataFetcher {
    async fn retrieve_data(&self, source: String) -> String {
        let result = self
            .client
            .get(source)
            .headers(self.get_headers())
            .send()
            .await
            .unwrap();

        result.text().await.unwrap()
    }
}

impl InvoiceFetcher for ApiDataFetcher {
    async fn retrieve_invoices_ids(&self) -> Vec<OrderInvoiceId> {
        let body = self
            .retrieve_data(self.get_end_point(Resource::OrderInvoices, None::<OrderInvoiceId>))
            .await;
        let data: OrderInvoices = serde_json::from_str(&body).unwrap();
        data.order_invoices
    }
    async fn retrieve_invoice(&self, id: OrderInvoiceId) -> OrderInvoice {
        let body = self
            .retrieve_data(self.get_end_point(Resource::OrderInvoices, Some(id)))
            .await;
        let data: OrderInvoiceData = serde_json::from_str(&body).unwrap();
        data.order_invoice
    }
}

#[derive(Debug)]
pub enum FormatType {
    Json,
}

impl ToString for FormatType {
    fn to_string(&self) -> String {
        match self {
            FormatType::Json => String::from("?output_format=JSON"),
        }
    }
}

impl Into<String> for FormatType {
    fn into(self) -> String {
        match self {
            FormatType::Json => String::from("?output_format=JSON"),
        }
    }
}
