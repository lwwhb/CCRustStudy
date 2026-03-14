use glam::{Mat4, Quat, Vec3};
use std::f32::consts::PI;

// ============================================================================
// 相机系统
// ============================================================================

/// 透视相机
#[derive(Debug, Clone)]
pub struct PerspectiveCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl PerspectiveCamera {
    pub fn new(position: Vec3, target: Vec3, aspect: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    /// 获取视图矩阵
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// 获取投影矩阵
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// 获取视图投影矩阵
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// 获取前向向量
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// 获取右向向量
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }

    /// 更新宽高比
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

/// 正交相机
#[derive(Debug, Clone)]
pub struct OrthographicCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl OrthographicCamera {
    pub fn new(position: Vec3, target: Vec3, width: f32, height: f32) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        Self {
            position,
            target,
            up: Vec3::Y,
            left: -half_width,
            right: half_width,
            bottom: -half_height,
            top: half_height,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

// ============================================================================
// 第一人称相机控制器
// ============================================================================

/// 第一人称相机控制器
#[derive(Debug, Clone)]
pub struct FirstPersonController {
    pub position: Vec3,
    pub yaw: f32,   // 偏航角（左右）
    pub pitch: f32, // 俯仰角（上下）
    pub speed: f32,
    pub sensitivity: f32,
}

impl FirstPersonController {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: -90.0_f32.to_radians(), // 初始朝向 -Z
            pitch: 0.0,
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    /// 获取前向向量
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    /// 获取右向向量
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    /// 获取上向向量
    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize()
    }

    /// 移动前后
    pub fn move_forward(&mut self, delta: f32) {
        self.position += self.forward() * delta * self.speed;
    }

    /// 移动左右
    pub fn move_right(&mut self, delta: f32) {
        self.position += self.right() * delta * self.speed;
    }

    /// 移动上下
    pub fn move_up(&mut self, delta: f32) {
        self.position += Vec3::Y * delta * self.speed;
    }

    /// 旋转视角
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw * self.sensitivity * 0.01;
        self.pitch += delta_pitch * self.sensitivity * 0.01;

        // 限制俯仰角
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }

    /// 转换为透视相机
    pub fn to_camera(&self, aspect: f32) -> PerspectiveCamera {
        PerspectiveCamera::new(self.position, self.position + self.forward(), aspect)
    }
}

// ============================================================================
// 第三人称相机控制器
// ============================================================================

/// 第三人称相机控制器
#[derive(Debug, Clone)]
pub struct ThirdPersonController {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub sensitivity: f32,
}

impl ThirdPersonController {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance,
            yaw: 0.0,
            pitch: 30.0_f32.to_radians(),
            min_distance: 2.0,
            max_distance: 20.0,
            sensitivity: 0.1,
        }
    }

    /// 获取相机位置
    pub fn position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.sin();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.cos();

        self.target + Vec3::new(x, y, z)
    }

    /// 旋转
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw * self.sensitivity * 0.01;
        self.pitch += delta_pitch * self.sensitivity * 0.01;

        // 限制俯仰角
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }

    /// 缩放（改变距离）
    pub fn zoom(&mut self, delta: f32) {
        self.distance -= delta;
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }

    /// 移动目标
    pub fn move_target(&mut self, delta: Vec3) {
        self.target += delta;
    }

    /// 转换为透视相机
    pub fn to_camera(&self, aspect: f32) -> PerspectiveCamera {
        PerspectiveCamera::new(self.position(), self.target, aspect)
    }
}

// ============================================================================
// 输入状态
// ============================================================================

/// 键盘按键状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

/// 输入管理器
#[derive(Debug, Clone)]
pub struct InputManager {
    keys: std::collections::HashMap<String, KeyState>,
    mouse_position: (f32, f32),
    mouse_delta: (f32, f32),
    scroll_delta: f32,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys: std::collections::HashMap::new(),
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            scroll_delta: 0.0,
        }
    }

    /// 设置按键状态
    pub fn set_key(&mut self, key: String, state: KeyState) {
        self.keys.insert(key, state);
    }

    /// 检查按键是否按下
    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.keys.get(key) == Some(&KeyState::Pressed)
    }

    /// 更新鼠标位置
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        let old_pos = self.mouse_position;
        self.mouse_position = (x, y);
        self.mouse_delta = (x - old_pos.0, y - old_pos.1);
    }

    /// 获取鼠标增量
    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    /// 设置滚轮增量
    pub fn set_scroll_delta(&mut self, delta: f32) {
        self.scroll_delta = delta;
    }

    /// 获取滚轮增量
    pub fn scroll_delta(&self) -> f32 {
        self.scroll_delta
    }

    /// 重置每帧数据
    pub fn reset_frame_data(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.scroll_delta = 0.0;
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 相机动画
// ============================================================================

/// 相机关键帧
#[derive(Debug, Clone)]
pub struct CameraKeyframe {
    pub time: f32,
    pub position: Vec3,
    pub target: Vec3,
}

/// 相机动画
#[derive(Debug, Clone)]
pub struct CameraAnimation {
    pub keyframes: Vec<CameraKeyframe>,
    pub duration: f32,
}

impl CameraAnimation {
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
            duration: 0.0,
        }
    }

    pub fn add_keyframe(&mut self, keyframe: CameraKeyframe) {
        self.duration = self.duration.max(keyframe.time);
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// 在指定时间插值相机状态
    pub fn sample(&self, time: f32) -> Option<(Vec3, Vec3)> {
        if self.keyframes.is_empty() {
            return None;
        }

        let t = time % self.duration;

        // 找到前后两个关键帧
        let mut prev_idx = 0;
        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.time > t {
                break;
            }
            prev_idx = i;
        }

        let next_idx = (prev_idx + 1) % self.keyframes.len();
        let prev = &self.keyframes[prev_idx];
        let next = &self.keyframes[next_idx];

        // 线性插值
        let alpha = if next.time > prev.time {
            (t - prev.time) / (next.time - prev.time)
        } else {
            0.0
        };

        let position = prev.position.lerp(next.position, alpha);
        let target = prev.target.lerp(next.target, alpha);

        Some((position, target))
    }
}

impl Default for CameraAnimation {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
    println!("相机与输入系统演示");

    // 创建第一人称相机
    let mut fps_controller = FirstPersonController::new(Vec3::new(0.0, 1.0, 5.0));
    fps_controller.move_forward(1.0);
    println!("第一人称相机位置: {:?}", fps_controller.position);

    // 创建第三人称相机
    let tps_controller = ThirdPersonController::new(Vec3::ZERO, 5.0);
    println!("第三人称相机位置: {:?}", tps_controller.position());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perspective_camera() {
        let camera = PerspectiveCamera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, 16.0 / 9.0);
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();
        assert!(!view.is_nan());
        assert!(!proj.is_nan());
    }

    #[test]
    fn test_orthographic_camera() {
        let camera = OrthographicCamera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, 10.0, 10.0);
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();
        assert!(!view.is_nan());
        assert!(!proj.is_nan());
    }

    #[test]
    fn test_first_person_controller() {
        let mut controller = FirstPersonController::new(Vec3::ZERO);
        let initial_pos = controller.position;

        controller.move_forward(1.0);
        assert_ne!(controller.position, initial_pos);

        controller.rotate(10.0, 5.0);
        assert_ne!(controller.yaw, -90.0_f32.to_radians());
    }

    #[test]
    fn test_third_person_controller() {
        let mut controller = ThirdPersonController::new(Vec3::ZERO, 5.0);
        let initial_pos = controller.position();

        controller.rotate(10.0, 5.0);
        assert_ne!(controller.position(), initial_pos);

        controller.zoom(1.0);
        assert_eq!(controller.distance, 4.0);
    }

    #[test]
    fn test_input_manager() {
        let mut input = InputManager::new();

        input.set_key("W".to_string(), KeyState::Pressed);
        assert!(input.is_key_pressed("W"));
        assert!(!input.is_key_pressed("S"));

        input.set_mouse_position(100.0, 50.0);
        input.set_mouse_position(110.0, 55.0);
        let delta = input.mouse_delta();
        assert_eq!(delta, (10.0, 5.0));
    }

    #[test]
    fn test_camera_animation() {
        let mut anim = CameraAnimation::new();

        anim.add_keyframe(CameraKeyframe {
            time: 0.0,
            position: Vec3::ZERO,
            target: Vec3::new(0.0, 0.0, -1.0),
        });

        anim.add_keyframe(CameraKeyframe {
            time: 1.0,
            position: Vec3::new(5.0, 0.0, 0.0),
            target: Vec3::new(5.0, 0.0, -1.0),
        });

        let sample = anim.sample(0.5);
        assert!(sample.is_some());

        let (pos, _) = sample.unwrap();
        assert!((pos.x - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_camera_forward_vector() {
        let camera = PerspectiveCamera::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1.0);
        let forward = camera.forward();
        assert!((forward.z + 1.0).abs() < 0.01);
    }

    #[test]
    fn test_fps_controller_movement() {
        let mut controller = FirstPersonController::new(Vec3::ZERO);

        controller.move_forward(1.0);
        controller.move_right(1.0);
        controller.move_up(1.0);

        assert_ne!(controller.position, Vec3::ZERO);
    }

    #[test]
    fn test_tps_controller_zoom() {
        let mut controller = ThirdPersonController::new(Vec3::ZERO, 10.0);

        controller.zoom(5.0);
        assert_eq!(controller.distance, 5.0);

        controller.zoom(-10.0);
        assert_eq!(controller.distance, 15.0);
    }

    #[test]
    fn test_input_reset() {
        let mut input = InputManager::new();

        input.set_mouse_position(100.0, 100.0);
        input.set_scroll_delta(5.0);

        input.reset_frame_data();

        assert_eq!(input.mouse_delta(), (0.0, 0.0));
        assert_eq!(input.scroll_delta(), 0.0);
    }
}
