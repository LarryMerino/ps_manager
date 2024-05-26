pub mod customers;
pub mod order_details;
pub mod order_invoices;
pub mod orders;

pub trait PsId {
    fn get_id(&self) -> usize;
}

pub struct Invoice {
    header: Header,
    shipping_address: ShippingAddress,
}

impl Invoice {
    pub fn new() -> InvoiceBuilder {
        InvoiceBuilder {}
    }
}

pub struct InvoiceBuilder {}

struct Header {}
struct ShippingAddress {}
