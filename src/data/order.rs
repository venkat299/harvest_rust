/* COMPLETE, REJECTED, CANCELLED, and OPEN

placing order parameter	
=======================
tradingsymbol	Tradingsymbol of the instrument
exchange	Name of the exchange
transaction_type	BUY or SELL
order_type	Order type (MARKET, LIMIT etc.)
quantity	Quantity to transact
product	Margin product applied to the order (margin is blocked based on this). More on margin products here.
price	For LIMIT orders
trigger_price	For SL, SL-M etc.
disclosed_quantity	Quantity to disclose publicly (for equity trades)
validity	Order validity
tag	An optional tag to apply to an order to identify it (alphanumeric, max 8 chars)

*/
extern crate rustc_serialize;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Order {
  // pub tradingsymbol:i32,
  pub exchange: super::Exchange,
  pub transaction_type: super::TransactionType,
  pub order_type: super::OrderType,
  pub quantity: i32,
  pub product: super::Product,
  // pub trigger_price: String,
  // pub disclosed_quantity : String,
  // pub validity : String,
  // pub tag:String,
  // pub status: super::OrderStatus
}
