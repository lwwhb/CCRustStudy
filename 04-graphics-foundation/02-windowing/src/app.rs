/// 应用状态
///
/// 管理应用程序的状态和数据

use std::time::Instant;

/// 应用程序状态
pub struct AppState {
    /// 窗口标题
    pub title: String,
    /// 窗口宽度
    pub width: u32,
    /// 窗口高度
    pub height: u32,
    /// 是否全屏
    pub fullscreen: bool,
    /// 鼠标位置
    pub mouse_x: f64,
    pub mouse_y: f64,
    /// 鼠标按钮状态
    pub mouse_left_pressed: bool,
    pub mouse_right_pressed: bool,
    /// 键盘状态
    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,
    pub key_space: bool,
    pub key_shift: bool,
    /// FPS 计数
    pub frame_count: u64,
    pub last_fps_update: Instant,
    pub fps: f64,
    /// 玩家位置（用于演示）
    pub player_x: f32,
    pub player_y: f32,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new(title: String, width: u32, height: u32) -> Self {
        AppState {
            title,
            width,
            height,
            fullscreen: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_left_pressed: false,
            mouse_right_pressed: false,
            key_w: false,
            key_a: false,
            key_s: false,
            key_d: false,
            key_space: false,
            key_shift: false,
            frame_count: 0,
            last_fps_update: Instant::now(),
            fps: 0.0,
            player_x: 0.0,
            player_y: 0.0,
        }
    }

    /// 更新窗口大小
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        println!("窗口大小: {}x{}", width, height);
    }

    /// 更新鼠标位置
    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    /// 更新 FPS
    pub fn update_fps(&mut self) {
        self.frame_count += 1;

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_fps_update).as_secs_f64();

        if elapsed >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed;
            self.frame_count = 0;
            self.last_fps_update = now;
            println!("FPS: {:.2}", self.fps);
        }
    }

    /// 更新玩家位置（基于键盘输入）
    pub fn update_player(&mut self, delta_time: f32) {
        let speed = 100.0; // 像素/秒
        let movement = speed * delta_time;

        if self.key_w {
            self.player_y -= movement;
        }
        if self.key_s {
            self.player_y += movement;
        }
        if self.key_a {
            self.player_x -= movement;
        }
        if self.key_d {
            self.player_x += movement;
        }

        // 限制在窗口范围内
        self.player_x = self.player_x.clamp(0.0, self.width as f32);
        self.player_y = self.player_y.clamp(0.0, self.height as f32);
    }

    /// 打印当前状态
    pub fn print_status(&self) {
        println!("\n=== 应用状态 ===");
        println!("窗口: {}x{}", self.width, self.height);
        println!("全屏: {}", self.fullscreen);
        println!("鼠标: ({:.1}, {:.1})", self.mouse_x, self.mouse_y);
        println!("鼠标按钮: L={} R={}", self.mouse_left_pressed, self.mouse_right_pressed);
        println!("WASD: W={} A={} S={} D={}", self.key_w, self.key_a, self.key_s, self.key_d);
        println!("玩家位置: ({:.1}, {:.1})", self.player_x, self.player_y);
        println!("FPS: {:.2}", self.fps);
    }

    /// 重置状态
    pub fn reset(&mut self) {
        self.player_x = (self.width / 2) as f32;
        self.player_y = (self.height / 2) as f32;
        self.key_w = false;
        self.key_a = false;
        self.key_s = false;
        self.key_d = false;
        self.key_space = false;
        self.key_shift = false;
        println!("状态已重置");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new("Test".to_string(), 800, 600);
        assert_eq!(state.width, 800);
        assert_eq!(state.height, 600);
        assert_eq!(state.fullscreen, false);
    }

    #[test]
    fn test_resize() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        state.resize(1024, 768);
        assert_eq!(state.width, 1024);
        assert_eq!(state.height, 768);
    }

    #[test]
    fn test_mouse_position() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        state.update_mouse_position(100.0, 200.0);
        assert_eq!(state.mouse_x, 100.0);
        assert_eq!(state.mouse_y, 200.0);
    }

    #[test]
    fn test_player_movement() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        state.player_x = 400.0;
        state.player_y = 300.0;

        state.key_w = true;
        state.update_player(0.1); // 0.1 秒
        assert!(state.player_y < 300.0);

        state.key_w = false;
        state.key_d = true;
        state.update_player(0.1);
        assert!(state.player_x > 400.0);
    }

    #[test]
    fn test_player_bounds() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        state.player_x = 0.0;
        state.player_y = 0.0;

        state.key_a = true;
        state.update_player(1.0);
        assert_eq!(state.player_x, 0.0); // 不应该小于 0

        state.key_a = false;
        state.player_x = 800.0;
        state.key_d = true;
        state.update_player(1.0);
        assert_eq!(state.player_x, 800.0); // 不应该大于宽度
    }

    #[test]
    fn test_reset() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        state.key_w = true;
        state.key_a = true;
        state.player_x = 100.0;
        state.player_y = 100.0;

        state.reset();

        assert_eq!(state.key_w, false);
        assert_eq!(state.key_a, false);
        assert_eq!(state.player_x, 400.0);
        assert_eq!(state.player_y, 300.0);
    }
}
