use serde::{Deserialize, Serialize};

use super::PsId;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderDetails {
    pub order_details: Vec<OrderDetailId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderDetailId {
    id: usize,
}

impl PsId for OrderDetailId {
    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderDetailData {
    pub order_detail: OrderDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderDetail {
    pub id: usize,
    pub id_order: String,
    pub product_id: String,
    pub product_attribute_id: String,
    pub product_quantity_reinjected: String,
    pub group_reduction: String,
    pub discount_quantity_applied: String,
    pub download_hash: String,
    pub download_deadline: String,
    pub id_order_invoice: String,
    pub id_warehouse: String,
    pub id_shop: String,
    pub id_customization: String,
    pub product_name: String,
    pub product_quantity: String,
    pub product_quantity_in_stock: String,
    pub product_quantity_return: String,
    pub product_quantity_refunded: String,
    pub product_price: String,
    pub reduction_percent: String,
    pub reduction_amount: String,
    pub reduction_amount_tax_incl: String,
    pub reduction_amount_tax_excl: String,
    pub product_quantity_discount: String,
    pub product_ean13: String,
    pub product_isbn: String,
    pub product_upc: String,
    pub product_mpn: String,
    pub product_reference: String,
    pub product_supplier_reference: String,
    pub product_weight: String,
    pub tax_computation_method: String,
    pub id_tax_rules_group: String,
    pub ecotax: String,
    pub ecotax_tax_rate: String,
    pub download_nb: String,
    pub unit_price_tax_incl: String,
    pub unit_price_tax_excl: String,
    pub total_price_tax_incl: String,
    pub total_price_tax_excl: String,
    pub total_shipping_price_tax_excl: String,
    pub total_shipping_price_tax_incl: String,
    pub purchase_supplier_price: String,
    pub original_product_price: String,
    pub original_wholesale_price: String,
    pub total_refunded_tax_excl: String,
    pub total_refunded_tax_incl: String,
}
