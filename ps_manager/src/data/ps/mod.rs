pub mod order_invoice;

pub trait PsId {
    fn get_id(&self) -> usize;
}
