use crate::{
    data::{
        customers::{Customer, CustomerData, CustomerId, Customers}, 
        order_details::{OrderDetail, OrderDetailData, OrderDetailId, OrderDetails}, 
        order_invoices::{OrderInvoice, OrderInvoiceData, OrderInvoiceId, OrderInvoices}, 
        orders::{Order, OrderData, OrderId, Orders},
        PsId,
    },
    data_fetcher::DataResource,
};

use super::{
    CustomersFetcher, DataFetcher, OrderDetailsFetcher, OrderInvoicesFetcher, OrdersFetcher, Resource,
};

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

pub trait PsApiCredentialsProvider {
    fn get_host(&self) -> String;
    fn get_raw_key(&self) -> String;
}

#[derive(Debug)]
pub struct ApiDataFetcher {
    host: Host,
    key: Key,
    format_type: FormatType,
    client: reqwest::Client,
}

impl ApiDataFetcher {
    pub fn new(credentials: &impl PsApiCredentialsProvider) -> ApiDataFetcherBuilder {
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

impl OrderDetailsFetcher for ApiDataFetcher {
    async fn retrieve_order_details_id(&self) -> Vec<OrderDetailId> {
        let body = self
            .retrieve_data(self.get_end_point(Resource::OrderDetails, None::<OrderDetailId>))
            .await;
        let data: OrderDetails = serde_json::from_str(&body).unwrap();
        data.order_details
    }

    async fn retrieve_order_detail(&self, id: OrderDetailId) -> OrderDetail {
        let body = self
            .retrieve_data(self.get_end_point(Resource::OrderDetails, Some(id)))
            .await;
        let data: OrderDetailData = serde_json::from_str(&body).unwrap();
        data.order_detail
    }
}

impl CustomersFetcher for ApiDataFetcher {
    async fn retrieve_customers_id(&self) -> Vec<CustomerId> {
        let body = self
            .retrieve_data(self.get_end_point(Resource::Customers, None::<CustomerId>))
            .await;
        let data: Customers = serde_json::from_str(&body).unwrap();
        data.customers
    }

    async fn retrieve_customer(&self, id: CustomerId) -> Customer {
        let body = self
            .retrieve_data(self.get_end_point(Resource::Customers, Some(id)))
            .await;
        let data: CustomerData = serde_json::from_str(&body).unwrap();
        data.customer
    }
}

impl OrderInvoicesFetcher for ApiDataFetcher {
    async fn retrieve_order_invoices_id(&self) -> Vec<OrderInvoiceId> {
        let body = self
            .retrieve_data(self.get_end_point(Resource::OrderInvoices, None::<OrderInvoiceId>))
            .await;
        let data: OrderInvoices = serde_json::from_str(&body).unwrap();
        data.order_invoices
    }
    async fn retrieve_order_invoice(&self, id: OrderInvoiceId) -> OrderInvoice {
        let body = self
            .retrieve_data(self.get_end_point(Resource::OrderInvoices, Some(id)))
            .await;
        let data: OrderInvoiceData = serde_json::from_str(&body).unwrap();
        data.order_invoice
    }
}

impl OrdersFetcher for ApiDataFetcher {
    async fn retrieve_orders_id(&self) -> Vec<OrderId> {
        let body = self
            .retrieve_data(self.get_end_point(Resource::Orders, None::<OrderId>))
            .await;
        let data: Orders = serde_json::from_str(&body).unwrap();
        data.orders
    }

    async fn retrieve_order(&self, id: OrderId) -> Order {
        let body = self
            .retrieve_data(self.get_end_point(Resource::Orders, Some(id)))
            .await;
        let data: OrderData = serde_json::from_str(&body).unwrap();
        data.order
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
