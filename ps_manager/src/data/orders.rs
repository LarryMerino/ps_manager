use serde::{Deserialize, Serialize};

use super::PsId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Orders {
    pub orders: Vec<OrderId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderId {
    id: usize,
}

impl PsId for OrderId {
    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderData {
    pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: usize,
    pub id_address_delivery: String,
    pub id_address_invoice: String,
    pub id_cart: String,
    pub id_currency: String,
    pub id_lang: String,
    pub id_customer: String,
    pub id_carrier: String,
    pub current_state: String,
    pub module: String,
    pub invoice_number: String,
    pub invoice_date: String,
    pub delivery_number: String,
    pub delivery_date: String,
    pub valid: String,
    pub date_add: String,
    pub date_upd: String,
    pub shipping_number: String,
    pub id_shop_group: String,
    pub id_shop: String,
    pub secure_key: String,
    pub payment: String,
    pub recyclable: String,
    pub gift: String,
    pub gift_message: String,
    pub mobile_theme: String,
    pub total_discounts: String,
    pub total_discounts_tax_incl: String,
    pub total_discounts_tax_excl: String,
    pub total_paid: String,
    pub total_paid_tax_incl: String,
    pub total_paid_tax_excl: String,
    pub total_paid_real: String,
    pub total_products: String,
    pub total_products_wt: String,
    pub total_shipping: String,
    pub total_shipping_tax_incl: String,
    pub total_shipping_tax_excl: String,
    pub carrier_tax_rate: String,
    pub total_wrapping: String,
    pub total_wrapping_tax_incl: String,
    pub total_wrapping_tax_excl: String,
    pub round_mode: String,
    pub round_type: String,
    pub conversion_rate: String,
    pub reference: String,
}
