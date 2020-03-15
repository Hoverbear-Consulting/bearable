mod line_item;
pub use line_item::LineItem;

mod money;
pub use money::Money;

mod currency;
pub use currency::Currency;

mod billing_method;
pub use billing_method::BillingMethod;

mod foreign_key;
pub use foreign_key::ForeignKey;

trait HasVariants {
    fn variants() -> Vec<String>;
}
