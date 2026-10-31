//! Shop-scoped e-commerce APIs. Mutations are never automatically retried.
mod catalog;
mod commerce;
mod lists;
pub mod params;
mod reports;
mod scope;
mod shops;

pub use catalog::{Categories, Products, Variants};
pub use commerce::{CartItems, Carts, Customers, Orders};
pub use lists::{List, Sort};
pub use shops::{Ecommerce, ShopScope};

use scope::Scope;
