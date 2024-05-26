use serde::{Deserialize, Serialize};

use super::PsId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Customers {
    pub customers: Vec<CustomerId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerId {
    id: usize,
}

impl PsId for CustomerId {
    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerData {
    pub customer: Customer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    id: usize,
    id_default_group: String,
    id_lang: String,
    newsletter_date_add: String,
    ip_registration_newsletter: Option<String>,
    last_passwd_gen: String,
    secure_key: String,
    deleted: String,
    passwd: String,
    lastname: String,
    firstname: String,
    email: String,
    id_gender: String,
    birthday: String,
    newsletter: String,
    optin: String,
    website: Option<String>,
    company: Option<String>,
    siret: Option<String>,
    ape: Option<String>,
    outstanding_allow_amount: String,
    show_public_prices: String,
    id_risk: String,
    max_payment_days: String,
    active: String,
    note: Option<String>,
    is_guest: String,
    id_shop: String,
    id_shop_group: String,
    date_add: String,
    date_upd: String,
    reset_password_token: Option<String>,
    reset_password_validity: String,
}
