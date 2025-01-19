use crate::traits::{DisplayDetail, ProductTrait};

// src/customer.rs
pub struct Customer {
    pub name: String,
    pub email: String,
    pub vip_level: u8,
    pub shopping_cart: Vec<Box<dyn ProductTrait>>,
}

impl Customer {
    pub fn new(
        name: String,
        email: String,
        vip_level: u8,
        shopping_cart: Vec<Box<dyn ProductTrait>>,
    ) -> Self {
        Customer {
            name,
            email,
            vip_level,
            shopping_cart,
        }
    }
}

impl DisplayDetail for Customer {
    fn show_detail(&self) {
        println!("用户名称: {}", self.name);
        println!("邮箱: {}", self.email);
        println!("VIP等级: {}", self.vip_level);
        println!("购物车中的商品:");
        for product in &self.shopping_cart {
            product.show_detail();
            println!();
        }
    }
}
