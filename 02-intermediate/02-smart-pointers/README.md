# 模块 2.2：智能指针

## 🎯 学习目标

- 理解 Box、Rc、Arc 的使用场景
- 掌握 RefCell 和内部可变性模式
- 学习 Weak 避免循环引用
- 实现自定义智能指针
- 理解 Deref 和 Drop trait

## 📚 核心概念

### 1. Box<T> - 堆分配

```rust
// 在堆上分配数据
let b = Box::new(5);
println!("b = {}", b);

// 递归类型
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### 2. Rc<T> - 引用计数

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);
let c = Rc::clone(&a);

println!("引用计数: {}", Rc::strong_count(&a)); // 3
```

### 3. RefCell<T> - 内部可变性

```rust
use std::cell::RefCell;

let value = RefCell::new(5);
*value.borrow_mut() += 1;
println!("{}", value.borrow()); // 6
```

### 4. Rc<RefCell<T>> - 共享可变数据

```rust
use std::rc::Rc;
use std::cell::RefCell;

let value = Rc::new(RefCell::new(5));
let a = Rc::clone(&value);
let b = Rc::clone(&value);

*a.borrow_mut() += 10;
println!("{}", b.borrow()); // 15
```

### 5. Weak<T> - 弱引用

```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(5);
let weak: Weak<_> = Rc::downgrade(&strong);

// 升级弱引用
if let Some(value) = weak.upgrade() {
    println!("{}", value);
}
```

### 6. Arc<T> - 原子引用计数

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    println!("{:?}", data_clone);
});
```

## 💻 实战项目：树形数据结构

使用智能指针实现二叉搜索树，演示 Rc、RefCell 和 Weak 的使用。

### 功能需求

1. 二叉搜索树（使用 Rc<RefCell<Node>>）
2. 双向链表（使用 Weak 避免循环引用）
3. 图结构（使用 Rc 和 Weak）
4. 自定义智能指针

### 项目结构

```
smart-pointers/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── tree.rs       # 二叉搜索树
│   ├── list.rs       # 双向链表
│   └── custom.rs     # 自定义智能指针
└── README.md
```

## 🧪 练习题

### 练习 1：使用 Box

```rust
// 实现递归类型的链表
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### 练习 2：使用 Rc 和 RefCell

```rust
// 实现共享可变的计数器
use std::rc::Rc;
use std::cell::RefCell;

struct Counter {
    count: Rc<RefCell<i32>>,
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust by Example - Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html)

## ✅ 检查清单

- [ ] 使用 Box 在堆上分配数据
- [ ] 使用 Rc 实现共享所有权
- [ ] 使用 RefCell 实现内部可变性
- [ ] 使用 Weak 避免循环引用
- [ ] 理解 Deref 和 Drop trait
- [ ] 实现自定义智能指针

## 🚀 下一步

完成本模块后，继续学习 [模块 2.3：闭包与函数式编程](../03-closures-functional/)。
