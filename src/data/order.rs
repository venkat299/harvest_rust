// COMPLETE, REJECTED, CANCELLED, and OPEN
//
// placing order parameter
// =======================
// tradingsymbol  Tradingsymbol of the instrument
// exchange Name of the exchange
// transaction_type BUY or SELL
// order_type Order type (MARKET, LIMIT etc.)
// quantity Quantity to transact
// product  Margin product applied to the order (margin is blocked based on this).
// More on margin products here.
// price  For LIMIT orders
// trigger_price  For SL, SL-M etc.
// disclosed_quantity Quantity to disclose publicly (for equity trades)
// validity Order validity
// tag  An optional tag to apply to an order to identify it (alphanumeric, max 8 chars)
//
//
extern crate rustc_serialize;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct RegularOrder {
    pub tradingsymbol: super::Symbol,
    pub exchange: super::Exchange,
    pub transaction_type: super::TransactionType,
    pub order_type: super::OrderType,
    pub quantity: u32,
    pub product: super::Product,
    pub price: f32,
    pub trigger_price: f32,
    pub disclosed_quantity: u32,
    pub validity: super::Validity,
    pub tag: String,
}
