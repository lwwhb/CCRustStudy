# 模块 2.2：智能指针 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解智能指针的概念和用途
2. 掌握 Box、Rc、Arc 的使用
3. 学习 RefCell 和内部可变性
4. 使用 Weak 避免循环引用
5. 实现树形数据结构

## 🎯 为什么需要智能指针？

### 普通引用的局限

**问题场景 1：多个所有者**
```rust
// 想要多个变量共享同一个数据
let data = vec![1, 2, 3];
let a = &data;  // 借用
let b = &data;  // 借用
// 但是 data 离开作用域后，a 和 b 都失效

// 如果想要多个所有者呢？
// 普通引用做不到！
```

**问题场景 2：递归数据结构**
```rust
// 想要定义链表
enum List {
    Cons(i32, List),  // 错误！大小未知
    Nil,
}

// 编译器不知道 List 的大小
// 因为它可能无限递归
```

**问题场景 3：内部可变性**
```rust
// 想要在不可变引用中修改数据
let x = 5;
let r = &x;
// *r = 6;  // 错误！不能通过不可变引用修改

// 有时候需要这种模式
// 普通引用做不到！
```

### 智能指针的解决方案

```rust
// Box - 堆分配，固定大小
enum List {
    Cons(i32, Box<List>),  // OK！Box 大小固定
    Nil,
}

// Rc - 引用计数，多个所有者
let data = Rc::new(vec![1, 2, 3]);
let a = Rc::clone(&data);  // 共享所有权
let b = Rc::clone(&data);  // 共享所有权

// RefCell - 内部可变性
let x = RefCell::new(5);
let r = &x;
*r.borrow_mut() = 6;  // OK！运行时检查
```

## 📖 核心概念详解

### 1. Box<T> - 堆分配

Box 是最简单的智能指针，用于在堆上分配数据。

#### 基础用法

```rust
// 在堆上分配一个整数
let b = Box::new(5);
println!("b = {}", b);  // 自动解引用

// Box 的大小是固定的（指针大小）
println!("Box 大小: {} 字节", std::mem::size_of::<Box<i32>>());
// 输出: 8 字节（64位系统）

// 数据在堆上
println!("数据大小: {} 字节", std::mem::size_of::<i32>());
// 输出: 4 字节
```

**内存布局**：
```
栈                堆
┌─────────┐      ┌─────┐
│ Box ptr │─────>│  5  │
└─────────┘      └─────┘
8 字节           4 字节
```

#### 递归类型

Box 最常用于定义递归数据结构。

```rust
// 链表定义
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// 创建链表: 1 -> 2 -> 3
let list = Cons(1,
    Box::new(Cons(2,
        Box::new(Cons(3,
            Box::new(Nil))))));

// 为什么需要 Box？
// 没有 Box：
enum List {
    Cons(i32, List),  // 大小未知！
    Nil,
}
// List 的大小 = i32 的大小 + List 的大小
// List 的大小 = 4 + List 的大小
// List 的大小 = 4 + 4 + List 的大小
// ... 无限递归！

// 有 Box：
enum List {
    Cons(i32, Box<List>),  // 大小已知！
    Nil,
}
// List 的大小 = max(4 + 8, 0) = 12 字节
```

#### 何时使用 Box

```rust
// ✅ 使用场景 1：递归类型
enum Tree {
    Node(i32, Box<Tree>, Box<Tree>),
    Leaf,
}

// ✅ 使用场景 2：大型数据转移所有权
let large_data = Box::new([0u8; 1_000_000]);
// 只移动指针，不复制数据

// ✅ 使用场景 3：trait 对象
let drawable: Box<dyn Draw> = Box::new(Circle { radius: 5.0 });

// ❌ 不需要 Box 的场景
let x = Box::new(5);  // 没必要，直接用 let x = 5;
```

### 2. Rc<T> - 引用计数

Rc（Reference Counting）允许多个所有者共享数据。

#### 基础用法

```rust
use std::rc::Rc;

// 创建 Rc
let a = Rc::new(5);
println!("引用计数: {}", Rc::strong_count(&a));  // 1

// 克隆 Rc（增加引用计数）
let b = Rc::clone(&a);
println!("引用计数: {}", Rc::strong_count(&a));  // 2

let c = Rc::clone(&a);
println!("引用计数: {}", Rc::strong_count(&a));  // 3

// 所有 Rc 都指向同一数据
println!("a = {}, b = {}, c = {}", a, b, c);

// 离开作用域时，引用计数递减
{
    let d = Rc::clone(&a);
    println!("引用计数: {}", Rc::strong_count(&a));  // 4
}
println!("引用计数: {}", Rc::strong_count(&a));  // 3
```

**内存布局**：
```
栈                    堆
┌─────┐             ┌───────────┐
│ a   │────────────>│ count: 3  │
└─────┘             │ data: 5   │
┌─────┐             └───────────┘
│ b   │────────────>      ↑
└─────┘                   │
┌─────┐                   │
│ c   │───────────────────┘
└─────┘
```

#### 共享数据结构

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

// 创建共享的尾部
let tail = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

// 两个列表共享同一个尾部
let a = Cons(3, Rc::clone(&tail));
let b = Cons(4, Rc::clone(&tail));

// 内存布局：
//     a: 3 ─┐
//           ├─> 5 -> 10 -> Nil
//     b: 4 ─┘
```

#### Rc 的限制

```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);

// ❌ 不能修改 Rc 中的数据
// data.push(4);  // 错误！Rc 只提供不可变访问

// ❌ 不能在多线程中使用
// std::thread::spawn(move || {
//     println!("{:?}", data);  // 错误！Rc 不是线程安全的
// });

// 解决方案：
// 1. 修改数据 -> 使用 Rc<RefCell<T>>
// 2. 多线程 -> 使用 Arc<T>
```

### 3. RefCell<T> - 内部可变性

RefCell 允许在运行时检查借用规则，实现内部可变性。

#### 借用规则回顾

```rust
// 编译时借用规则：
let mut x = 5;
let r1 = &x;      // OK
let r2 = &x;      // OK
// let r3 = &mut x;  // 错误！不能同时有不可变和可变引用

// RefCell 将检查推迟到运行时
use std::cell::RefCell;

let x = RefCell::new(5);
let r1 = x.borrow();      // 运行时借用
let r2 = x.borrow();      // OK
// let r3 = x.borrow_mut();  // panic！违反借用规则
```

#### 基础用法

```rust
use std::cell::RefCell;

let value = RefCell::new(5);

// 不可变借用
{
    let borrowed = value.borrow();
    println!("值: {}", borrowed);
}  // borrowed 离开作用域

// 可变借用
{
    let mut borrowed_mut = value.borrow_mut();
    *borrowed_mut += 10;
}  // borrowed_mut 离开作用域

println!("修改后: {}", value.borrow());  // 15
```

#### 内部可变性模式

```rust
// 场景：在不可变方法中修改内部状态
struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: RefCell::new(0),
        }
    }
    
    // 不可变方法，但可以修改内部状态
    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }
    
    fn get(&self) -> i32 {
        *self.count.borrow()
    }
}

let counter = Counter::new();
counter.increment();  // &self，但修改了内部状态
counter.increment();
println!("计数: {}", counter.get());  // 2
```

### 4. Rc<RefCell<T>> - 共享可变数据

组合 Rc 和 RefCell 实现多个所有者共享可变数据。

```rust
use std::rc::Rc;
use std::cell::RefCell;

// 创建共享的可变数据
let value = Rc::new(RefCell::new(5));

// 多个所有者
let a = Rc::clone(&value);
let b = Rc::clone(&value);

// 通过任何一个所有者修改数据
*a.borrow_mut() += 10;
println!("通过 a 修改后: {}", b.borrow());  // 15

*b.borrow_mut() += 5;
println!("通过 b 修改后: {}", value.borrow());  // 20
```

**使用场景**：
```rust
// 图结构：节点之间相互引用
struct Node {
    value: i32,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

// 树结构：父节点和子节点相互引用
struct TreeNode {
    value: i32,
    children: Vec<Rc<RefCell<TreeNode>>>,
}
```

### 5. Weak<T> - 弱引用

Weak 用于避免循环引用导致的内存泄漏。

#### 循环引用问题

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

// 创建循环引用
let a = Rc::new(RefCell::new(Node { value: 1, next: None }));
let b = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&a)) }));

// 形成循环
a.borrow_mut().next = Some(Rc::clone(&b));

// 内存泄漏！
// a 引用 b，b 引用 a
// 引用计数永远不会变成 0
// 内存永远不会被释放
```

#### 使用 Weak 解决

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,  // 使用 Weak
}

let a = Rc::new(RefCell::new(Node {
    value: 1,
    next: None,
    prev: None,
}));

let b = Rc::new(RefCell::new(Node {
    value: 2,
    next: Some(Rc::clone(&a)),
    prev: None,
}));

// 使用 Weak 引用
a.borrow_mut().prev = Some(Rc::downgrade(&b));

// 不会内存泄漏！
// b 强引用 a（引用计数 +1）
// a 弱引用 b（不增加引用计数）
// b 离开作用域时，引用计数变为 0，被释放
// a 也随之被释放
```

#### Weak 的使用

```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(5);
println!("强引用计数: {}", Rc::strong_count(&strong));  // 1

// 创建弱引用
let weak: Weak<i32> = Rc::downgrade(&strong);
println!("强引用计数: {}", Rc::strong_count(&strong));  // 1
println!("弱引用计数: {}", Rc::weak_count(&strong));    // 1

// 升级弱引用
match weak.upgrade() {
    Some(rc) => println!("值: {}", rc),
    None => println!("值已被释放"),
}

// 释放强引用
drop(strong);

// 弱引用升级失败
match weak.upgrade() {
    Some(rc) => println!("值: {}", rc),
    None => println!("值已被释放"),  // 这里
}
```

### 6. Arc<T> - 原子引用计数

Arc（Atomic Reference Counting）是线程安全的 Rc。

```rust
use std::sync::Arc;
use std::thread;

// 创建 Arc
let data = Arc::new(vec![1, 2, 3, 4, 5]);

// 在多个线程中共享
let mut handles = vec![];

for i in 0..3 {
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("线程 {} 看到: {:?}", i, data_clone);
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**Rc vs Arc**：
```
Rc<T>:
- 单线程
- 性能更好（无原子操作）
- 不能跨线程

Arc<T>:
- 多线程
- 性能稍差（原子操作）
- 可以跨线程
```

## 💻 实战项目：二叉搜索树

使用智能指针实现二叉搜索树。

### 树节点定义

```rust
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    value: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}
```

### 二叉搜索树实现

```rust
pub struct BinarySearchTree {
    root: Link,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }
    
    pub fn insert(&mut self, value: i32) {
        match self.root {
            None => {
                self.root = Some(Node::new(value));
            }
            Some(ref node) => {
                Self::insert_node(node, value);
            }
        }
    }
    
    fn insert_node(node: &Rc<RefCell<Node>>, value: i32) {
        let mut node_ref = node.borrow_mut();
        
        if value < node_ref.value {
            match node_ref.left {
                None => {
                    node_ref.left = Some(Node::new(value));
                }
                Some(ref left) => {
                    Self::insert_node(left, value);
                }
            }
        } else {
            match node_ref.right {
                None => {
                    node_ref.right = Some(Node::new(value));
                }
                Some(ref right) => {
                    Self::insert_node(right, value);
                }
            }
        }
    }
    
    pub fn search(&self, value: &i32) -> bool {
        Self::search_node(&self.root, value)
    }
    
    fn search_node(node: &Link, value: &i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                let n_ref = n.borrow();
                if value == &n_ref.value {
                    true
                } else if value < &n_ref.value {
                    Self::search_node(&n_ref.left, value)
                } else {
                    Self::search_node(&n_ref.right, value)
                }
            }
        }
    }
}
```

## 🔍 深入理解

### 智能指针的选择

```
需要堆分配？
└─> Box<T>

需要多个所有者？
├─> 单线程？
│   └─> Rc<T>
└─> 多线程？
    └─> Arc<T>

需要内部可变性？
├─> 单线程？
│   └─> RefCell<T>
└─> 多线程？
    └─> Mutex<T> 或 RwLock<T>

需要避免循环引用？
└─> Weak<T>

组合使用：
- Rc<RefCell<T>> - 单线程共享可变数据
- Arc<Mutex<T>> - 多线程共享可变数据
```

### 性能考虑

```rust
// Box - 零成本
let b = Box::new(5);  // 只是指针

// Rc - 引用计数开销
let rc = Rc::new(5);  // 额外存储计数

// Arc - 原子操作开销
let arc = Arc::new(5);  // 原子操作更慢

// RefCell - 运行时检查
let cell = RefCell::new(5);  // 运行时借用检查
```

## 📝 练习题

### 练习 1：实现双向链表

使用 Rc 和 Weak 实现双向链表，避免循环引用。

### 练习 2：实现图结构

使用智能指针实现有向图，支持添加节点和边。

### 练习 3：线程安全的计数器

使用 Arc 和 Mutex 实现线程安全的计数器。

## ✅ 检查清单

- [ ] 理解 Box 的用途和使用场景
- [ ] 掌握 Rc 的引用计数机制
- [ ] 理解 RefCell 的内部可变性
- [ ] 会使用 Rc<RefCell<T>> 组合
- [ ] 理解 Weak 避免循环引用
- [ ] 知道何时使用 Arc
- [ ] 能实现树形数据结构
- [ ] 理解智能指针的性能影响

## 🚀 下一步

完成本模块后，继续学习 [模块 2.3：闭包与函数式编程](../03-closures-functional/)。
