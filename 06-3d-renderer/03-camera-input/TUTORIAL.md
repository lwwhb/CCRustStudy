# 模块 6.3：相机与输入系统 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 实现第一人称相机
2. 实现第三人称相机
3. 处理键盘和鼠标输入
4. 实现相机动画
5. 构建相机控制器

## 🎯 为什么需要相机系统？

### 固定视角 vs 交互式相机

**固定视角（受限）**：
```rust
// 固定的视图矩阵
let view = Matrix4::look_at_rh(
    &Point3::new(0.0, 0.0, 5.0),
    &Point3::new(0.0, 0.0, 0.0),
    &Vector3::new(0.0, 1.0, 0.0),
);

问题：
- 无法移动
- 无法旋转
- 无法交互
- 体验差
```

**交互式相机（灵活）**：
```rust
// 响应用户输入
camera.process_keyboard(input);
camera.process_mouse(delta_x, delta_y);
camera.update(delta_time);

let view = camera.view_matrix();

优势：
- 自由移动
- 自由旋转
- 交互体验好
- 支持多种模式
```

### 相机类型

```
1. 第一人称（FPS）
   - 玩家视角
   - WASD 移动
   - 鼠标旋转

2. 第三人称
   - 跟随目标
   - 环绕旋转
   - 距离调整

3. 轨道相机
   - 围绕中心旋转
   - 适合查看模型

4. 自由相机
   - 完全自由
   - 适合编辑器
```

## 📖 核心概念详解

### 1. 相机基础

#### 相机结构

```rust
use nalgebra as na;

pub struct Camera {
    // 位置和方向
    pub position: na::Point3<f32>,
    pub target: na::Point3<f32>,
    pub up: na::Vector3<f32>,

    // 投影参数
    pub fov: f32,           // 视野角度
    pub aspect: f32,        // 宽高比
    pub near: f32,          // 近裁剪面
    pub far: f32,           // 远裁剪面

    // 视图和投影矩阵
    view_matrix: na::Matrix4<f32>,
    proj_matrix: na::Matrix4<f32>,
}

impl Camera {
    pub fn new(
        position: na::Point3<f32>,
        target: na::Point3<f32>,
        aspect: f32,
    ) -> Self {
        let up = na::Vector3::y();
        let fov = std::f32::consts::PI / 4.0;  // 45 度
        let near = 0.1;
        let far = 100.0;

        let mut camera = Self {
            position,
            target,
            up,
            fov,
            aspect,
            near,
            far,
            view_matrix: na::Matrix4::identity(),
            proj_matrix: na::Matrix4::identity(),
        };

        camera.update_matrices();
        camera
    }

    pub fn update_matrices(&mut self) {
        // 视图矩阵
        self.view_matrix = na::Matrix4::look_at_rh(
            &self.position,
            &self.target,
            &self.up,
        );

        // 投影矩阵
        self.proj_matrix = na::Matrix4::new_perspective(
            self.aspect,
            self.fov,
            self.near,
            self.far,
        );
    }

    pub fn view_matrix(&self) -> &na::Matrix4<f32> {
        &self.view_matrix
    }

    pub fn proj_matrix(&self) -> &na::Matrix4<f32> {
        &self.proj_matrix
    }

    pub fn view_proj_matrix(&self) -> na::Matrix4<f32> {
        self.proj_matrix * self.view_matrix
    }
}
```

### 2. 第一人称相机

#### FPS 相机实现

```rust
pub struct FpsCamera {
    // 位置
    pub position: na::Point3<f32>,

    // 欧拉角
    pub yaw: f32,    // 偏航（左右）
    pub pitch: f32,  // 俯仰（上下）

    // 方向向量
    front: na::Vector3<f32>,
    right: na::Vector3<f32>,
    up: na::Vector3<f32>,
    world_up: na::Vector3<f32>,

    // 移动速度
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,

    // 投影参数
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl FpsCamera {
    pub fn new(position: na::Point3<f32>, aspect: f32) -> Self {
        let mut camera = Self {
            position,
            yaw: -90.0,  // 初始朝向 -Z
            pitch: 0.0,
            front: na::Vector3::zeros(),
            right: na::Vector3::zeros(),
            up: na::Vector3::zeros(),
            world_up: na::Vector3::y(),
            movement_speed: 5.0,
            mouse_sensitivity: 0.1,
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
        };

        camera.update_vectors();
        camera
    }

    fn update_vectors(&mut self) {
        // 计算前方向
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        self.front = na::Vector3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        ).normalize();

        // 计算右方向
        self.right = self.front.cross(&self.world_up).normalize();

        // 计算上方向
        self.up = self.right.cross(&self.front).normalize();
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        match direction {
            CameraMovement::Forward => {
                self.position += self.front * velocity;
            }
            CameraMovement::Backward => {
                self.position -= self.front * velocity;
            }
            CameraMovement::Left => {
                self.position -= self.right * velocity;
            }
            CameraMovement::Right => {
                self.position += self.right * velocity;
            }
            CameraMovement::Up => {
                self.position += self.world_up * velocity;
            }
            CameraMovement::Down => {
                self.position -= self.world_up * velocity;
            }
        }
    }

    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // 限制俯仰角
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_vectors();
    }

    pub fn process_scroll(&mut self, y_offset: f32) {
        self.fov -= y_offset.to_radians();
        self.fov = self.fov.clamp(1.0_f32.to_radians(), 45.0_f32.to_radians());
    }

    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(
            &self.position,
            &(self.position + self.front),
            &self.up,
        )
    }

    pub fn proj_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(
            self.aspect,
            self.fov,
            self.near,
            self.far,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
```

**欧拉角说明**：
```
Yaw（偏航）：
- 绕 Y 轴旋转
- 控制左右方向
- 0° = 朝向 +X
- 90° = 朝向 +Z

Pitch（俯仰）：
- 绕 X 轴旋转
- 控制上下方向
- 0° = 水平
- +90° = 朝上
- -90° = 朝下

Roll（翻滚）：
- 绕 Z 轴旋转
- FPS 相机通常不用
```

### 3. 第三人称相机

#### 跟随相机实现

```rust
pub struct ThirdPersonCamera {
    // 目标位置
    pub target: na::Point3<f32>,

    // 相机参数
    pub distance: f32,  // 距离目标的距离
    pub yaw: f32,       // 水平角度
    pub pitch: f32,     // 垂直角度

    // 限制
    pub min_distance: f32,
    pub max_distance: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,

    // 灵敏度
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,

    // 投影参数
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl ThirdPersonCamera {
    pub fn new(target: na::Point3<f32>, aspect: f32) -> Self {
        Self {
            target,
            distance: 10.0,
            yaw: 0.0,
            pitch: 20.0,
            min_distance: 2.0,
            max_distance: 50.0,
            min_pitch: -89.0,
            max_pitch: 89.0,
            rotate_sensitivity: 0.5,
            zoom_sensitivity: 1.0,
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn calculate_position(&self) -> na::Point3<f32> {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let x = self.distance * pitch_rad.cos() * yaw_rad.sin();
        let y = self.distance * pitch_rad.sin();
        let z = self.distance * pitch_rad.cos() * yaw_rad.cos();

        self.target + na::Vector3::new(x, y, z)
    }

    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw += x_offset * self.rotate_sensitivity;
        self.pitch -= y_offset * self.rotate_sensitivity;

        self.pitch = self.pitch.clamp(self.min_pitch, self.max_pitch);
    }

    pub fn process_scroll(&mut self, y_offset: f32) {
        self.distance -= y_offset * self.zoom_sensitivity;
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }

    pub fn set_target(&mut self, target: na::Point3<f32>) {
        self.target = target;
    }

    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        let position = self.calculate_position();
        na::Matrix4::look_at_rh(
            &position,
            &self.target,
            &na::Vector3::y(),
        )
    }

    pub fn proj_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(
            self.aspect,
            self.fov,
            self.near,
            self.far,
        )
    }
}
```

### 4. 输入处理

#### 输入管理器

```rust
use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputManager {
    // 键盘状态
    pressed_keys: HashSet<KeyCode>,

    // 鼠标状态
    mouse_buttons: HashSet<MouseButton>,
    mouse_position: (f64, f64),
    mouse_delta: (f32, f32),
    scroll_delta: f32,

    // 上一帧的鼠标位置
    last_mouse_position: Option<(f64, f64)>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            mouse_buttons: HashSet::new(),
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            scroll_delta: 0.0,
            last_mouse_position: None,
        }
    }

    pub fn process_keyboard(&mut self, event: &KeyEvent) {
        if let PhysicalKey::Code(keycode) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    self.pressed_keys.insert(keycode);
                }
                ElementState::Released => {
                    self.pressed_keys.remove(&keycode);
                }
            }
        }
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.mouse_buttons.insert(button);
            }
            ElementState::Released => {
                self.mouse_buttons.remove(&button);
            }
        }
    }

    pub fn process_mouse_motion(&mut self, position: (f64, f64)) {
        if let Some(last_pos) = self.last_mouse_position {
            self.mouse_delta = (
                (position.0 - last_pos.0) as f32,
                (position.1 - last_pos.1) as f32,
            );
        }

        self.mouse_position = position;
        self.last_mouse_position = Some(position);
    }

    pub fn process_mouse_wheel(&mut self, delta: f32) {
        self.scroll_delta = delta;
    }

    pub fn update(&mut self) {
        // 重置每帧的增量
        self.mouse_delta = (0.0, 0.0);
        self.scroll_delta = 0.0;
    }

    // 查询方法
    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn scroll_delta(&self) -> f32 {
        self.scroll_delta
    }
}
```

#### 相机控制器

```rust
use winit::keyboard::KeyCode;

pub struct CameraController {
    camera: FpsCamera,
    input: InputManager,
}

impl CameraController {
    pub fn new(camera: FpsCamera) -> Self {
        Self {
            camera,
            input: InputManager::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // 处理键盘输入
        if self.input.is_key_pressed(KeyCode::KeyW) {
            self.camera.process_keyboard(CameraMovement::Forward, delta_time);
        }
        if self.input.is_key_pressed(KeyCode::KeyS) {
            self.camera.process_keyboard(CameraMovement::Backward, delta_time);
        }
        if self.input.is_key_pressed(KeyCode::KeyA) {
            self.camera.process_keyboard(CameraMovement::Left, delta_time);
        }
        if self.input.is_key_pressed(KeyCode::KeyD) {
            self.camera.process_keyboard(CameraMovement::Right, delta_time);
        }
        if self.input.is_key_pressed(KeyCode::Space) {
            self.camera.process_keyboard(CameraMovement::Up, delta_time);
        }
        if self.input.is_key_pressed(KeyCode::ShiftLeft) {
            self.camera.process_keyboard(CameraMovement::Down, delta_time);
        }

        // 处理鼠标输入
        let (dx, dy) = self.input.mouse_delta();
        if dx != 0.0 || dy != 0.0 {
            self.camera.process_mouse(dx, dy);
        }

        // 处理滚轮
        let scroll = self.input.scroll_delta();
        if scroll != 0.0 {
            self.camera.process_scroll(scroll);
        }

        // 更新输入状态
        self.input.update();
    }

    pub fn camera(&self) -> &FpsCamera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut FpsCamera {
        &mut self.camera
    }

    pub fn input_mut(&mut self) -> &mut InputManager {
        &mut self.input
    }
}
```

### 5. 相机动画

#### 平滑移动

```rust
pub struct SmoothCamera {
    camera: FpsCamera,
    target_position: na::Point3<f32>,
    target_yaw: f32,
    target_pitch: f32,
    smoothness: f32,  // 0.0 = 瞬间，1.0 = 永不到达
}

impl SmoothCamera {
    pub fn new(camera: FpsCamera) -> Self {
        let target_position = camera.position;
        let target_yaw = camera.yaw;
        let target_pitch = camera.pitch;

        Self {
            camera,
            target_position,
            target_yaw,
            target_pitch,
            smoothness: 0.1,
        }
    }

    pub fn set_target_position(&mut self, position: na::Point3<f32>) {
        self.target_position = position;
    }

    pub fn set_target_rotation(&mut self, yaw: f32, pitch: f32) {
        self.target_yaw = yaw;
        self.target_pitch = pitch;
    }

    pub fn update(&mut self, delta_time: f32) {
        let t = 1.0 - self.smoothness.powf(delta_time);

        // 插值位置
        self.camera.position = na::Point3::from(
            self.camera.position.coords.lerp(&self.target_position.coords, t)
        );

        // 插值旋转
        self.camera.yaw = lerp(self.camera.yaw, self.target_yaw, t);
        self.camera.pitch = lerp(self.camera.pitch, self.target_pitch, t);

        self.camera.update_vectors();
    }

    pub fn camera(&self) -> &FpsCamera {
        &self.camera
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
```

## 💻 实战项目：交互式 3D 查看器

### 项目需求

构建一个 3D 模型查看器，支持：
1. FPS 相机控制
2. 轨道相机模式
3. 平滑切换
4. 键盘和鼠标输入
5. 相机状态显示

### 完整实现

```rust
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct App {
    renderer: Renderer,
    camera_controller: CameraController,
    last_frame_time: std::time::Instant,
}

impl App {
    async fn new(window: &winit::window::Window) -> Self {
        let renderer = Renderer::new(window).await;
        
        let camera = FpsCamera::new(
            na::Point3::new(0.0, 2.0, 5.0),
            renderer.aspect(),
        );
        
        let camera_controller = CameraController::new(camera);

        Self {
            renderer,
            camera_controller,
            last_frame_time: std::time::Instant::now(),
        }
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        let delta_time = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        self.camera_controller.update(delta_time);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let camera = self.camera_controller.camera();
        self.renderer.render(camera)
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.camera_controller.input_mut().process_keyboard(event);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.camera_controller.input_mut().process_mouse_button(*button, *state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.camera_controller.input_mut().process_mouse_motion((position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                };
                self.camera_controller.input_mut().process_mouse_wheel(scroll);
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("3D Viewer")
        .build(&event_loop)
        .unwrap();

    let mut app = pollster::block_on(App::new(&window));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => {
                        app.renderer.resize(size);
                    }
                    WindowEvent::RedrawRequested => {
                        app.update();
                        match app.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => app.renderer.resize(app.renderer.size()),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {
                        app.handle_event(&event);
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 实现第一人称相机
- [ ] 实现第三人称相机
- [ ] 处理键盘输入
- [ ] 处理鼠标输入
- [ ] 实现相机平滑移动
- [ ] 理解欧拉角和方向向量
- [ ] 构建相机控制器
- [ ] 实现相机切换

## 🔗 延伸阅读

- [Learn OpenGL - Camera](https://learnopengl.com/Getting-started/Camera)
- [Camera Systems in Games](https://docs.unity3d.com/Manual/CamerasOverview.html)
- [Euler Angles](https://en.wikipedia.org/wiki/Euler_angles)

## 🚀 下一步

完成本模块后，继续学习：
- 模块 6.4：光照与材质系统
- 模块 6.5：高级渲染特性

---

**掌握相机系统，创造沉浸式 3D 体验！** 🎮
