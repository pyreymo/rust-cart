use crate::product::ProductCategory;

pub trait Purchasable {
    fn unit_price(&self) -> f32;
    fn inventory(&self) -> u32;
}

pub trait DisplayDetail {
    fn show_detail(&self);
}

pub trait ProductInfo {
    fn category(&self) -> &ProductCategory;
}

pub trait ProductTrait: Purchasable + DisplayDetail + ProductInfo {}
