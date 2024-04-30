pub mod api_client;

use ps_macro::DataResource;

use crate::data::ps::order_invoice::{OrderInvoice, OrderInvoiceId};

/// Data can be fetch from different sources and in this sense
/// the objective of this trait is provide an abstraction logic of each of the possible fetcher
pub trait DataFetcher {
    fn retrieve_data(&self, source: String) -> impl std::future::Future<Output = String> + Send;
}

pub trait InvoiceFetcher: DataFetcher {
    fn retrieve_invoices_ids(
        &self,
    ) -> impl std::future::Future<Output = Vec<OrderInvoiceId>> + Send;
    fn retrieve_invoice(
        &self,
        id: OrderInvoiceId,
    ) -> impl std::future::Future<Output = OrderInvoice> + Send;
}

pub trait DataResource {
    fn as_api(&self) -> String;
}

#[derive(DataResource)]
pub enum Resource {
    OrderInvoices,
    Orders,
    Customer,
    CustomerOrderId,
}
