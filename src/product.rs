// product.rs

use crate::traits::{DisplayDetail, ProductInfo, ProductTrait, Purchasable};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProductCategory {
    Book,
    Electronic,
    Food,
}

pub struct Product {
    pub name: String,
    pub unit_price: f32,
    pub inventory: u32,
    pub category: ProductCategory,
}

impl Purchasable for Product {
    fn unit_price(&self) -> f32 {
        self.unit_price
    }

    fn inventory(&self) -> u32 {
        self.inventory
    }
}

impl DisplayDetail for Product {
    fn show_detail(&self) {
        println!("商品名称: {}", self.name);
        println!("单价: {}", self.unit_price);
        println!("库存数量: {}", self.inventory);
        println!("类别: {:?}", self.category);
    }
}

impl ProductInfo for Product {
    fn category(&self) -> &ProductCategory {
        &self.category
    }
}

impl ProductTrait for Product {}

pub struct Book {
    pub product: Product,
    pub author: String,
    pub isbn: String,
}

impl Purchasable for Book {
    fn unit_price(&self) -> f32 {
        self.product.unit_price
    }

    fn inventory(&self) -> u32 {
        self.product.inventory
    }
}

impl DisplayDetail for Book {
    fn show_detail(&self) {
        self.product.show_detail();
        println!("作者: {}", self.author);
        println!("ISBN: {}", self.isbn);
    }
}

impl ProductInfo for Book {
    fn category(&self) -> &ProductCategory {
        &ProductCategory::Book
    }
}

impl ProductTrait for Book {}
