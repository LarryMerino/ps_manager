pub mod ps;
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
