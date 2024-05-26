use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::{debug, info};
use ps_manager::data_fetcher::api_client::{ApiDataFetcher, FormatType};
use ps_manager::data_fetcher::{CustomersFetcher, OrderInvoicesFetcher, OrdersFetcher};

use crate::settings::{DebugConfProvider, Settings, SettingsProvider};

pub mod settings;

// Check this for some inspiration
// https://github.com/JaynewDee/portform/blob/866727ad3cd6ff499844d54b673472f646c95a40/src/generate.rs#L4

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new().unwrap();
    let logger = env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or(settings.debug_conf_provider().get_debug_lvl()),
    )
    .build();

    let multi = MultiProgress::new();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();

    info!("Logger initialized");

    let api_data_fetcher = ApiDataFetcher::new(settings.api_credential_provider())
        .format(FormatType::Json)
        .build();
    info!("api_data_fetcher: {:?}", api_data_fetcher);

    let clients = api_data_fetcher.retrieve_customers_id().await;
    for client_id in clients {
        debug!("client: {:?}", api_data_fetcher.retrieve_customer(client_id).await);
    }

    let orders = api_data_fetcher.retrieve_orders_id().await;
    for order_id in orders {
        debug!(
            "oder: {:?}",
            api_data_fetcher.retrieve_order(order_id).await
        );
    }

    let invoices = api_data_fetcher.retrieve_order_invoices_id().await;
    for invoice_id in invoices {
        debug!(
            "data: {:?}",
            api_data_fetcher.retrieve_order_invoice(invoice_id).await
        );
    }

    Ok(())
}
