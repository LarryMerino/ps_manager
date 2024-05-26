pub mod api_client;

use ps_macro::DataResource;

use crate::data::{
    customers::{Customer, CustomerId},
    order_details::{OrderDetail, OrderDetailId},
    order_invoices::{OrderInvoice, OrderInvoiceId},
    orders::{Order, OrderId},
};

/// Data can be fetch from different sources and in this sense
/// the objective of this trait is provide an abstraction logic of each of the possible fetcher
pub trait DataFetcher {
    fn retrieve_data(&self, source: String) -> impl std::future::Future<Output = String> + Send;
}

pub trait CustomersFetcher: DataFetcher {
    fn retrieve_customers_id(&self) -> impl std::future::Future<Output = Vec<CustomerId>> + Send;
    fn retrieve_customer(&self, id: CustomerId) -> impl std::future::Future<Output = Customer> + Send;
}

pub trait OrderDetailsFetcher: DataFetcher {
    fn retrieve_order_details_id(
        &self,
    ) -> impl std::future::Future<Output = Vec<OrderDetailId>> + Send;
    fn retrieve_order_detail(
        &self,
        id: OrderDetailId,
    ) -> impl std::future::Future<Output = OrderDetail> + Send;
}


pub trait OrderInvoicesFetcher: DataFetcher {
    fn retrieve_order_invoices_id(
        &self,
    ) -> impl std::future::Future<Output = Vec<OrderInvoiceId>> + Send;
    fn retrieve_order_invoice(
        &self,
        id: OrderInvoiceId,
    ) -> impl std::future::Future<Output = OrderInvoice> + Send;
}

pub trait OrdersFetcher: DataFetcher {
    fn retrieve_orders_id(&self) -> impl std::future::Future<Output = Vec<OrderId>> + Send;
    fn retrieve_order(&self, id: OrderId) -> impl std::future::Future<Output = Order> + Send;
}

pub trait DataResource {
    fn as_api(&self) -> String;
}

#[derive(DataResource)]
pub enum Resource {
    Customers,
    OrderDetails,
    OrderInvoices,
    Orders,
}
