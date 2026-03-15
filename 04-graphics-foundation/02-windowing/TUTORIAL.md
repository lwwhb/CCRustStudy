# 模块 4.2：窗口与事件处理 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 使用 winit 创建窗口
2. 处理事件循环
3. 响应键盘和鼠标输入
4. 实现跨平台窗口管理
5. 处理窗口生命周期

## 🎯 为什么需要窗口库？

### 原生 API vs 跨平台库

**直接使用原生 API（困难）**：
```
Windows: Win32 API
- CreateWindowEx
- GetMessage
- DispatchMessage

macOS: Cocoa/AppKit
- NSWindow
- NSApplication
- NSEvent

Linux: X11/Wayland
- XCreateWindow
- XNextEvent

问题：
- 每个平台不同的 API
- 需要写三份代码
- 维护困难
```

**使用 winit（简单）**：
```rust
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

// 一份代码，三个平台
let event_loop = EventLoop::new();
let window = WindowBuilder::new()
    .with_title("My App")
    .build(&event_loop)
    .unwrap();

优势：
- 统一的 API
- 跨平台
- 维护简单
- 社区支持
```

### 事件驱动编程

```
传统循环（游戏）：
loop {
    update();
    render();
}

事件驱动（GUI）：
loop {
    event = wait_for_event();
    match event {
        KeyPress => handle_key(),
        MouseMove => handle_mouse(),
        WindowClose => break,
    }
}
```

## 📖 核心概念详解

### 1. 创建窗口

#### 基础窗口

```rust
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // 创建事件循环
    let event_loop = EventLoop::new().unwrap();

    // 创建窗口
    let window = WindowBuilder::new()
        .with_title("Hello Window")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    // 运行事件循环
    event_loop.run(move |event, elwt| {
        // 处理事件
    }).unwrap();
}
```

#### 窗口配置

```rust
use winit::window::{WindowBuilder, Fullscreen};
use winit::dpi::{LogicalSize, PhysicalSize};

let window = WindowBuilder::new()
    // 标题
    .with_title("My Application")
    
    // 大小（逻辑像素）
    .with_inner_size(LogicalSize::new(1024, 768))
    
    // 最小大小
    .with_min_inner_size(LogicalSize::new(640, 480))
    
    // 最大大小
    .with_max_inner_size(LogicalSize::new(1920, 1080))
    
    // 是否可调整大小
    .with_resizable(true)
    
    // 是否装饰（标题栏、边框）
    .with_decorations(true)
    
    // 是否透明
    .with_transparent(false)
    
    // 全屏模式
    // .with_fullscreen(Some(Fullscreen::Borderless(None)))
    
    .build(&event_loop)
    .unwrap();
```

**逻辑像素 vs 物理像素**：
```
逻辑像素：
- 与 DPI 无关
- 800x600 在所有屏幕上看起来一样大

物理像素：
- 实际的屏幕像素
- 高 DPI 屏幕：1 逻辑像素 = 2 物理像素

缩放因子（Scale Factor）：
物理像素 = 逻辑像素 × 缩放因子
```

### 2. 事件循环

#### 事件类型

```rust
use winit::event::{Event, WindowEvent};

event_loop.run(move |event, elwt| {
    match event {
        // 窗口事件
        Event::WindowEvent { event, window_id } => {
            match event {
                WindowEvent::CloseRequested => {
                    println!("关闭窗口");
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    println!("窗口大小: {}x{}", size.width, size.height);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    println!("键盘输入");
                }
                WindowEvent::CursorMoved { position, .. } => {
                    println!("鼠标位置: ({}, {})", position.x, position.y);
                }
                _ => {}
            }
        }

        // 设备事件（原始输入）
        Event::DeviceEvent { event, .. } => {
            // 处理设备事件
        }

        // 用户事件（自定义）
        Event::UserEvent(user_event) => {
            // 处理自定义事件
        }

        // 挂起/恢复
        Event::Suspended => {
            println!("应用挂起");
        }
        Event::Resumed => {
            println!("应用恢复");
        }

        // 关于退出
        Event::AboutToWait => {
            // 所有事件处理完毕，可以更新和渲染
            window.request_redraw();
        }

        // 重绘请求
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            // 渲染
        }

        _ => {}
    }
}).unwrap();
```

#### 事件循环模式

```rust
use winit::event_loop::ControlFlow;

event_loop.run(move |event, elwt| {
    // 设置控制流
    
    // 1. Poll - 持续运行（游戏）
    elwt.set_control_flow(ControlFlow::Poll);
    
    // 2. Wait - 等待事件（GUI）
    elwt.set_control_flow(ControlFlow::Wait);
    
    // 3. WaitUntil - 等待到指定时间
    use std::time::{Duration, Instant};
    let next_frame = Instant::now() + Duration::from_millis(16);
    elwt.set_control_flow(ControlFlow::WaitUntil(next_frame));
    
    // 处理事件...
}).unwrap();
```

**控制流对比**：
```
Poll:
- 持续运行，不等待
- CPU 使用率高
- 适合游戏

Wait:
- 等待事件
- CPU 使用率低
- 适合 GUI 应用

WaitUntil:
- 等待到指定时间
- 固定帧率
- 适合动画
```

### 3. 键盘输入

#### 键盘事件

```rust
use winit::event::{WindowEvent, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

match event {
    Event::WindowEvent {
        event: WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key,
                state,
                ..
            },
            ..
        },
        ..
    } => {
        // 检查按键
        if let PhysicalKey::Code(keycode) = physical_key {
            match state {
                winit::event::ElementState::Pressed => {
                    match keycode {
                        KeyCode::KeyW => println!("W 按下"),
                        KeyCode::KeyA => println!("A 按下"),
                        KeyCode::KeyS => println!("S 按下"),
                        KeyCode::KeyD => println!("D 按下"),
                        KeyCode::Space => println!("空格按下"),
                        KeyCode::Escape => elwt.exit(),
                        _ => {}
                    }
                }
                winit::event::ElementState::Released => {
                    println!("按键释放: {:?}", keycode);
                }
            }
        }
    }
    _ => {}
}
```

#### 键盘状态管理

```rust
use std::collections::HashSet;
use winit::keyboard::KeyCode;

struct InputState {
    pressed_keys: HashSet<KeyCode>,
}

impl InputState {
    fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    fn key_pressed(&mut self, keycode: KeyCode) {
        self.pressed_keys.insert(keycode);
    }

    fn key_released(&mut self, keycode: KeyCode) {
        self.pressed_keys.remove(&keycode);
    }

    fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode)
    }
}

// 使用
let mut input = InputState::new();

// 在事件循环中
match state {
    ElementState::Pressed => input.key_pressed(keycode),
    ElementState::Released => input.key_released(keycode),
}

// 检查状态
if input.is_key_pressed(KeyCode::KeyW) {
    // W 键被按下
}
```

### 4. 鼠标输入

#### 鼠标事件

```rust
use winit::event::{WindowEvent, MouseButton, MouseScrollDelta};

match event {
    Event::WindowEvent { event, .. } => {
        match event {
            // 鼠标移动
            WindowEvent::CursorMoved { position, .. } => {
                println!("鼠标: ({}, {})", position.x, position.y);
            }

            // 鼠标按钮
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => println!("左键"),
                    MouseButton::Right => println!("右键"),
                    MouseButton::Middle => println!("中键"),
                    _ => {}
                }
            }

            // 鼠标滚轮
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        println!("滚轮: ({}, {})", x, y);
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        println!("像素滚动: ({}, {})", pos.x, pos.y);
                    }
                }
            }

            // 鼠标进入/离开窗口
            WindowEvent::CursorEntered { .. } => {
                println!("鼠标进入窗口");
            }
            WindowEvent::CursorLeft { .. } => {
                println!("鼠标离开窗口");
            }

            _ => {}
        }
    }
    _ => {}
}
```

#### 鼠标状态管理

```rust
struct MouseState {
    position: (f64, f64),
    left_pressed: bool,
    right_pressed: bool,
    middle_pressed: bool,
}

impl MouseState {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            left_pressed: false,
            right_pressed: false,
            middle_pressed: false,
        }
    }

    fn update_position(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    fn button_pressed(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left_pressed = true,
            MouseButton::Right => self.right_pressed = true,
            MouseButton::Middle => self.middle_pressed = true,
            _ => {}
        }
    }

    fn button_released(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left_pressed = false,
            MouseButton::Right => self.right_pressed = false,
            MouseButton::Middle => self.middle_pressed = false,
            _ => {}
        }
    }
}
```

### 5. 窗口操作

#### 窗口属性

```rust
// 获取窗口大小
let size = window.inner_size();
println!("窗口大小: {}x{}", size.width, size.height);

// 获取窗口位置
if let Some(position) = window.outer_position().ok() {
    println!("窗口位置: ({}, {})", position.x, position.y);
}

// 设置窗口标题
window.set_title("新标题");

// 设置窗口大小
window.set_inner_size(LogicalSize::new(1024, 768));

// 设置窗口位置
window.set_outer_position(PhysicalPosition::new(100, 100));

// 最小化
window.set_minimized(true);

// 最大化
window.set_maximized(true);

// 全屏
window.set_fullscreen(Some(Fullscreen::Borderless(None)));

// 退出全屏
window.set_fullscreen(None);

// 请求重绘
window.request_redraw();
```

## 💻 实战项目：交互式窗口应用

### 项目需求

创建一个交互式窗口应用，支持：
1. 窗口创建和配置
2. 键盘控制（WASD 移动）
3. 鼠标交互（点击、拖拽）
4. 简单的 2D 图形绘制
5. FPS 显示

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
winit = "0.29"
```

### 步骤 2：应用状态

```rust
use std::collections::HashSet;
use std::time::Instant;
use winit::keyboard::KeyCode;

struct AppState {
    // 输入状态
    pressed_keys: HashSet<KeyCode>,
    mouse_position: (f64, f64),
    mouse_pressed: bool,
    
    // 应用状态
    player_position: (f32, f32),
    player_speed: f32,
    
    // FPS 计数
    frame_count: u32,
    last_fps_update: Instant,
    current_fps: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            mouse_position: (0.0, 0.0),
            mouse_pressed: false,
            player_position: (400.0, 300.0),
            player_speed: 200.0,
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0,
        }
    }

    fn update(&mut self, delta_time: f32) {
        // 更新玩家位置
        let mut dx = 0.0;
        let mut dy = 0.0;

        if self.pressed_keys.contains(&KeyCode::KeyW) {
            dy -= 1.0;
        }
        if self.pressed_keys.contains(&KeyCode::KeyS) {
            dy += 1.0;
        }
        if self.pressed_keys.contains(&KeyCode::KeyA) {
            dx -= 1.0;
        }
        if self.pressed_keys.contains(&KeyCode::KeyD) {
            dx += 1.0;
        }

        // 归一化方向
        let length = (dx * dx + dy * dy).sqrt();
        if length > 0.0 {
            dx /= length;
            dy /= length;
        }

        // 应用移动
        self.player_position.0 += dx * self.player_speed * delta_time;
        self.player_position.1 += dy * self.player_speed * delta_time;

        // 更新 FPS
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_fps_update).as_secs_f32();
        if elapsed >= 1.0 {
            self.current_fps = self.frame_count;
            self.frame_count = 0;
            self.last_fps_update = now;
        }
    }
}
```

### 步骤 3：事件处理

```rust
use winit::{
    event::{Event, WindowEvent, KeyEvent, MouseButton, ElementState},
    event_loop::{EventLoop, ControlFlow},
    keyboard::PhysicalKey,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Interactive Window")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut state = AppState::new();
    let mut last_update = Instant::now();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }

                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            physical_key: PhysicalKey::Code(keycode),
                            state: key_state,
                            ..
                        },
                        ..
                    } => {
                        match key_state {
                            ElementState::Pressed => {
                                state.pressed_keys.insert(keycode);
                                
                                // ESC 退出
                                if keycode == KeyCode::Escape {
                                    elwt.exit();
                                }
                            }
                            ElementState::Released => {
                                state.pressed_keys.remove(&keycode);
                            }
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        state.mouse_position = (position.x, position.y);
                    }

                    WindowEvent::MouseInput { state: button_state, button, .. } => {
                        if button == MouseButton::Left {
                            state.mouse_pressed = button_state == ElementState::Pressed;
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        // 这里应该渲染
                        // 目前只打印状态
                        println!(
                            "Player: ({:.1}, {:.1}), Mouse: ({:.1}, {:.1}), FPS: {}",
                            state.player_position.0,
                            state.player_position.1,
                            state.mouse_position.0,
                            state.mouse_position.1,
                            state.current_fps
                        );
                    }

                    _ => {}
                }
            }

            Event::AboutToWait => {
                // 更新
                let now = Instant::now();
                let delta_time = now.duration_since(last_update).as_secs_f32();
                last_update = now;

                state.update(delta_time);

                // 请求重绘
                window.request_redraw();
            }

            _ => {}
        }
    }).unwrap();
}
```

### 步骤 4：增强功能

```rust
impl AppState {
    // 边界检查
    fn clamp_position(&mut self, width: f32, height: f32) {
        self.player_position.0 = self.player_position.0.clamp(0.0, width);
        self.player_position.1 = self.player_position.1.clamp(0.0, height);
    }

    // 鼠标点击传送
    fn teleport_to_mouse(&mut self) {
        if self.mouse_pressed {
            self.player_position = (
                self.mouse_position.0 as f32,
                self.mouse_position.1 as f32,
            );
        }
    }

    // 重置位置
    fn reset(&mut self) {
        self.player_position = (400.0, 300.0);
    }
}

// 在事件处理中添加
WindowEvent::KeyboardInput {
    event: KeyEvent {
        physical_key: PhysicalKey::Code(KeyCode::KeyR),
        state: ElementState::Pressed,
        ..
    },
    ..
} => {
    state.reset();
}
```

## 🔍 深入理解

### 事件循环的工作原理

```
1. 初始化
   ↓
2. 等待事件
   ↓
3. 分发事件
   ↓
4. 处理事件
   ↓
5. AboutToWait
   ↓
6. 更新状态
   ↓
7. 请求重绘
   ↓
8. RedrawRequested
   ↓
9. 渲染
   ↓
10. 返回步骤 2
```

### DPI 处理

```rust
// 获取缩放因子
let scale_factor = window.scale_factor();

// 逻辑大小 -> 物理大小
let logical_size = LogicalSize::new(800, 600);
let physical_size: PhysicalSize<u32> = logical_size.to_physical(scale_factor);

// 物理大小 -> 逻辑大小
let logical_size: LogicalSize<f64> = physical_size.to_logical(scale_factor);
```

### 性能优化

```rust
// 1. 只在需要时重绘
Event::AboutToWait => {
    if state.needs_redraw() {
        window.request_redraw();
    }
}

// 2. 限制帧率
use std::time::{Duration, Instant};

let target_fps = 60;
let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
let next_frame = Instant::now() + frame_duration;

elwt.set_control_flow(ControlFlow::WaitUntil(next_frame));

// 3. 批处理事件
let mut events_buffer = Vec::new();

// 收集事件
events_buffer.push(event);

// 批量处理
for event in events_buffer.drain(..) {
    process_event(event);
}
```

## 📝 练习题

### 练习 1：全屏切换
实现按 F11 切换全屏模式。

### 练习 2：窗口拖拽
实现鼠标拖拽移动窗口内的对象。

### 练习 3：多按键组合
实现 Ctrl+S 保存，Ctrl+Q 退出等组合键。

### 练习 4：帧率限制
实现可配置的帧率限制（30/60/120 FPS）。

## 🎯 学习检查清单

- [ ] 创建和配置窗口
- [ ] 处理窗口事件
- [ ] 响应键盘输入
- [ ] 响应鼠标输入
- [ ] 管理输入状态
- [ ] 实现事件循环
- [ ] 处理 DPI 缩放
- [ ] 实现帧率控制
- [ ] 优雅退出应用

## 🔗 延伸阅读

- [winit 文档](https://docs.rs/winit/)
- [事件驱动编程](https://en.wikipedia.org/wiki/Event-driven_programming)
- [游戏循环模式](https://gameprogrammingpatterns.com/game-loop.html)

## 🚀 下一步

完成本模块后，继续学习：
- 模块 4.3：wgpu 基础
- 模块 4.4：着色器编程

---

**掌握窗口管理，为图形渲染打下基础！** 🚀
