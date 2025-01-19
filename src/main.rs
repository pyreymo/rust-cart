use traits::{DisplayDetail, ProductTrait};

mod customer;
mod order;
mod product;
mod traits;

fn main() {
    // 创建商品
    let product = product::Product {
        name: "苹果".to_string(),
        unit_price: 5.0,
        inventory: 10,
        category: product::ProductCategory::Food,
    };

    let book = product::Book {
        product: product::Product {
            name: "Rust编程".to_string(),
            unit_price: 50.0,
            inventory: 2,
            category: product::ProductCategory::Book,
        },
        author: "张三".to_string(),
        isbn: "123456789".to_string(),
    };

    // 创建购物车
    let mut shopping_cart: Vec<Box<dyn ProductTrait>> = Vec::new();
    shopping_cart.push(Box::new(product));
    shopping_cart.push(Box::new(book));

    // 创建客户
    let customer = customer::Customer::new(
        "李四".to_string(),
        "lisi@example.com".to_string(),
        1,
        shopping_cart,
    );

    // 创建订单
    let order = order::Order { customer };

    // 显示订单详情
    order.show_detail();

    // 计算并显示订单总价
    let total = order.calculate_total();
    println!("订单总价: {}", total);
}
