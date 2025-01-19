# RustCart

**RustCart** 是一个基于 Rust 的购物车系统，支持多态商品管理、VIP 折扣和书籍满减优惠。它展示了如何使用 Rust 的 trait 和动态分发来实现灵活的商品管理和折扣计算。

---

## 功能特性

- **多态商品管理**：支持多种商品类型（如书籍、电子产品、食品等），通过 trait 实现统一管理。
- **VIP 折扣**：根据用户的 VIP 等级（0-2）提供不同的折扣率。
- **书籍满减优惠**：对书籍类商品提供“每满 100 减 20”的优惠。
- **订单总价计算**：自动计算订单总价，并应用相应的折扣和优惠。
- **商品详情展示**：支持展示商品和订单的详细信息。

---

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/pyreymo/rust-cart.git
cd rust-cart
```

### 2. 运行项目

确保已安装 Rust 工具链（如 `rustc` 和 `cargo`），然后运行以下命令：

```bash
cargo run
```

### 3. 示例输出

运行后，程序将输出订单详情和总价，例如：

```
订单详情:
用户名称: 李四
邮箱: lisi@example.com
VIP等级: 1
购物车中的商品:
商品名称: 苹果
单价: 5
库存数量: 10
类别: Food

商品名称: Rust编程
单价: 50
库存数量: 2
类别: Book
作者: 张三
ISBN: 123456789

订单总价: 123.5
```

---

## 项目结构

```
rust-cart/
├── src/
│   ├── main.rs          # 程序入口
│   ├── customer.rs      # 客户和购物车逻辑
│   ├── order.rs         # 订单和折扣计算逻辑
│   ├── product.rs       # 商品和书籍逻辑
│   └── traits.rs        # 定义 trait（Purchasable, DisplayDetail, ProductInfo）
├── Cargo.toml           # 项目配置和依赖
└── README.md            # 项目文档
```

---

## 核心代码示例

### 1. 定义商品和书籍

```rust
// src/product.rs
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

pub struct Book {
    pub product: Product,
    pub author: String,
    pub isbn: String,
}
```

### 2. 实现 trait

```rust
// src/traits.rs
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
```

### 3. 计算订单总价

```rust
// src/order.rs
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
```

---

## 如何贡献

欢迎贡献代码！以下是贡献步骤：

1. Fork 本项目。
2. 创建一个新分支 (`git checkout -b feature/YourFeature`)。
3. 提交你的更改 (`git commit -am 'Add some feature'`)。
4. 推送到分支 (`git push origin feature/YourFeature`)。
5. 提交 Pull Request。

---

## 许可证

本项目基于 [MIT 许可证](LICENSE) 开源。

---

## 联系

如有问题或建议，请联系：  
- 邮箱：pyreymo@example.com  
- GitHub: [pyreymo](https://github.com/pyreymo)
