use crate::customer::Customer;
use crate::product::ProductCategory;
use crate::traits::DisplayDetail;

pub struct Order {
    pub customer: Customer,
}

impl DisplayDetail for Order {
    fn show_detail(&self) {
        println!("订单详情:");
        self.customer.show_detail();
    }
}

impl Order {
    pub fn calculate_total(&self) -> f32 {
        let mut subtotal: f32 = 0.0;
        let mut book_total: f32 = 0.0;

        for product in &self.customer.shopping_cart {
            if *product.category() == ProductCategory::Book {
                book_total += product.unit_price() * product.inventory() as f32;
            } else {
                subtotal += product.unit_price() * product.inventory() as f32;
            }
        }

        // Apply book discount: 每满100减20
        let book_discount = (book_total / 100.0).floor() * 20.0;
        let book_price = book_total - book_discount;

        subtotal += book_price;

        // Apply VIP discount based on customer's VIP level
        let vip_level = self.customer.vip_level;
        let vip_discount_rate = match vip_level {
            0 => 0.98,
            1 => 0.95,
            2 => 0.85,
            _ => 1.0, // default no discount
        };

        let final_total = subtotal * vip_discount_rate;

        final_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::customer::Customer;
    use crate::product::{Book, Product, ProductCategory};
    use crate::traits::ProductTrait;

    #[test]
    fn test_calculate_total() {
        // 创建书本
        let book_product = Product {
            name: "Rust编程".to_string(),
            unit_price: 50.0,
            inventory: 2,
            category: ProductCategory::Book,
        };
        let book = Book {
            product: book_product,
            author: "Alice".to_string(),
            isbn: "123456".to_string(),
        };
        // 创建电子产品
        let electronic_product = Product {
            name: "智能手机".to_string(),
            unit_price: 3000.0,
            inventory: 1,
            category: ProductCategory::Electronic,
        };
        // 将商品装箱成动态trait对象
        let mut shopping_cart: Vec<Box<dyn ProductTrait>> = Vec::new();
        shopping_cart.push(Box::new(book));
        shopping_cart.push(Box::new(electronic_product));
        // 创建客户
        let customer = Customer::new(
            "张三".to_string(),
            "zhangsan@example.com".to_string(),
            1,
            shopping_cart,
        );
        // 创建订单
        let order = Order { customer };
        // 计算总价
        let total = order.calculate_total();
        // 预期总价为2926.0
        assert_eq!(total, 2926.0);
    }

    #[test]
    fn test_calculate_total_empty_cart() {
        let shopping_cart: Vec<Box<dyn ProductTrait>> = Vec::new();
        let customer = Customer::new(
            "李四".to_string(),
            "lisi@example.com".to_string(),
            0,
            shopping_cart,
        );
        let order = Order { customer };
        let total = order.calculate_total();
        assert_eq!(total, 0.0);
    }

    #[test]
    fn test_calculate_total_only_books_less_100() {
        let book_product = Product {
            name: "简短故事".to_string(),
            unit_price: 40.0,
            inventory: 1,
            category: ProductCategory::Book,
        };
        let book = Book {
            product: book_product,
            author: "Bob".to_string(),
            isbn: "789012".to_string(),
        };
        let mut shopping_cart: Vec<Box<dyn ProductTrait>> = Vec::new();
        shopping_cart.push(Box::new(book));
        let customer = Customer::new(
            "王五".to_string(),
            "wangwu@example.com".to_string(),
            2,
            shopping_cart,
        );
        let order = Order { customer };
        let total = order.calculate_total();
        // 书本总价 = 40.0，不满100，没有折扣
        // VIP等级2，折扣率0.85
        // total = 40.0 * 0.85 = 34.0
        assert_eq!(total, 34.0);
    }

    #[test]
    fn test_calculate_total_only_electronics() {
        let electronic_product = Product {
            name: "笔记本电脑".to_string(),
            unit_price: 8000.0,
            inventory: 1,
            category: ProductCategory::Electronic,
        };
        let mut shopping_cart: Vec<Box<dyn ProductTrait>> = Vec::new();
        shopping_cart.push(Box::new(electronic_product));
        let customer = Customer::new(
            "赵六".to_string(),
            "zhaoliu@example.com".to_string(),
            0,
            shopping_cart,
        );
        let order = Order { customer };
        let total = order.calculate_total();
        // 电子产品总价 = 8000.0
        // 没有书籍折扣
        // VIP等级0，折扣率0.98
        // total = 8000.0 * 0.98 = 7840.0
        assert_eq!(total, 7840.0);
    }

    #[test]
    fn test_calculate_total_invalid_vip_level() {
        let book_product = Product {
            name: "Rust编程".to_string(),
            unit_price: 50.0,
            inventory: 2,
            category: ProductCategory::Book,
        };
        let book = Book {
            product: book_product,
            author: "Alice".to_string(),
            isbn: "123456".to_string(),
        };
        let mut shopping_cart: Vec<Box<dyn ProductTrait>> = Vec::new();
        shopping_cart.push(Box::new(book));
        let customer = Customer::new(
            "孙七".to_string(),
            "sunqi@example.com".to_string(),
            3,
            shopping_cart,
        );
        let order = Order { customer };
        let total = order.calculate_total();
        // 书本总价 = 100.0，折扣后 = 80.0
        // VIP等级3，没有折扣，折扣率1.0
        // total = 80.0 * 1.0 = 80.0
        assert_eq!(total, 80.0);
    }
}
