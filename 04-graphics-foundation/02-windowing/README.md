# 模块 4.2：窗口与事件处理

## 🎯 学习目标

- 使用 winit 创建跨平台窗口
- 理解事件循环机制
- 处理键盘和鼠标输入
- 管理窗口生命周期
- 处理窗口调整大小
- 实现基本的用户交互

## 📚 核心概念

### 1. 创建窗口

```rust
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("My Window")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, elwt| {
        // 处理事件
    }).unwrap();
}
```

### 2. 事件处理

```rust
use winit::event::{Event, WindowEvent};

event_loop.run(move |event, elwt| {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
            WindowEvent::Resized(size) => {
                println!("Window resized: {:?}", size);
            }
            _ => {}
        }
        _ => {}
    }
}).unwrap();
```

### 3. 键盘输入

```rust
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

WindowEvent::KeyboardInput { event, .. } => {
    if event.state == ElementState::Pressed {
        if let PhysicalKey::Code(keycode) = event.physical_key {
            match keycode {
                KeyCode::Escape => elwt.exit(),
                KeyCode::KeyW => println!("W pressed"),
                _ => {}
            }
        }
    }
}
```

### 4. 鼠标输入

```rust
use winit::event::MouseButton;

WindowEvent::MouseInput { state, button, .. } => {
    if state == ElementState::Pressed {
        match button {
            MouseButton::Left => println!("Left click"),
            MouseButton::Right => println!("Right click"),
            _ => {}
        }
    }
}

WindowEvent::CursorMoved { position, .. } => {
    println!("Mouse at: {:?}", position);
}
```

## 💻 实战项目：交互式窗口应用

创建一个支持用户交互的窗口应用。

### 功能需求

1. 创建可调整大小的窗口
2. 处理键盘输入（WASD 移动）
3. 处理鼠标输入（点击和移动）
4. 显示 FPS 计数器
5. 支持全屏切换

### 项目结构

```
windowing/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── window.rs       # 窗口管理
│   ├── input.rs        # 输入处理
│   └── app.rs          # 应用状态
└── README.md
```

## 🧪 练习题

### 练习 1：窗口控制

```rust
// 实现窗口最小化、最大化功能
// 使用快捷键控制窗口状态
```

### 练习 2：输入映射

```rust
// 创建一个输入映射系统
// 将键盘按键映射到游戏动作
```

### 练习 3：鼠标拖拽

```rust
// 实现鼠标拖拽功能
// 跟踪鼠标按下和移动
```

## 📖 深入阅读

- [winit Documentation](https://docs.rs/winit/)
- [winit Examples](https://github.com/rust-windowing/winit/tree/master/examples)
- [Game Input Handling](https://gameprogrammingpatterns.com/command.html)

## ✅ 检查清单

- [ ] 创建和配置窗口
- [ ] 实现事件循环
- [ ] 处理键盘输入
- [ ] 处理鼠标输入
- [ ] 管理窗口生命周期
- [ ] 处理窗口调整大小
- [ ] 实现全屏切换

## 🚀 下一步

完成本模块后，继续学习 [模块 4.3：wgpu 基础](../03-wgpu-basics/)。
