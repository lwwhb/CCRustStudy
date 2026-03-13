/// 输入处理
///
/// 处理键盘和鼠标输入事件

use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::AppState;

/// 处理窗口事件
pub fn handle_window_event(event: &WindowEvent, state: &mut AppState) -> bool {
    match event {
        WindowEvent::Resized(size) => {
            state.resize(size.width, size.height);
            false
        }
        WindowEvent::CloseRequested => {
            println!("关闭窗口");
            true
        }
        WindowEvent::KeyboardInput { event, .. } => {
            handle_keyboard_input(event, state)
        }
        WindowEvent::MouseInput { state: button_state, button, .. } => {
            handle_mouse_button(*button_state, *button, state);
            false
        }
        WindowEvent::CursorMoved { position, .. } => {
            state.update_mouse_position(position.x, position.y);
            false
        }
        WindowEvent::MouseWheel { delta, .. } => {
            handle_mouse_wheel(delta, state);
            false
        }
        _ => false,
    }
}

/// 处理键盘输入
fn handle_keyboard_input(event: &winit::event::KeyEvent, state: &mut AppState) -> bool {
    let pressed = event.state == ElementState::Pressed;

    if let PhysicalKey::Code(keycode) = event.physical_key {
        match keycode {
            KeyCode::Escape => {
                println!("按下 ESC，退出应用");
                return true;
            }
            KeyCode::KeyW => {
                state.key_w = pressed;
                if pressed {
                    println!("W 键按下");
                }
            }
            KeyCode::KeyA => {
                state.key_a = pressed;
                if pressed {
                    println!("A 键按下");
                }
            }
            KeyCode::KeyS => {
                state.key_s = pressed;
                if pressed {
                    println!("S 键按下");
                }
            }
            KeyCode::KeyD => {
                state.key_d = pressed;
                if pressed {
                    println!("D 键按下");
                }
            }
            KeyCode::Space => {
                state.key_space = pressed;
                if pressed {
                    println!("空格键按下");
                }
            }
            KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                state.key_shift = pressed;
                if pressed {
                    println!("Shift 键按下");
                }
            }
            KeyCode::KeyR => {
                if pressed {
                    state.reset();
                }
            }
            KeyCode::KeyP => {
                if pressed {
                    state.print_status();
                }
            }
            KeyCode::F11 => {
                if pressed {
                    state.fullscreen = !state.fullscreen;
                    println!("全屏切换: {}", state.fullscreen);
                }
            }
            _ => {}
        }
    }

    false
}

/// 处理鼠标按钮
fn handle_mouse_button(button_state: ElementState, button: MouseButton, state: &mut AppState) {
    let pressed = button_state == ElementState::Pressed;

    match button {
        MouseButton::Left => {
            state.mouse_left_pressed = pressed;
            if pressed {
                println!("鼠标左键按下 at ({:.1}, {:.1})", state.mouse_x, state.mouse_y);
            } else {
                println!("鼠标左键释放");
            }
        }
        MouseButton::Right => {
            state.mouse_right_pressed = pressed;
            if pressed {
                println!("鼠标右键按下 at ({:.1}, {:.1})", state.mouse_x, state.mouse_y);
            } else {
                println!("鼠标右键释放");
            }
        }
        MouseButton::Middle => {
            if pressed {
                println!("鼠标中键按下");
            }
        }
        _ => {}
    }
}

/// 处理鼠标滚轮
fn handle_mouse_wheel(delta: &winit::event::MouseScrollDelta, _state: &mut AppState) {
    match delta {
        winit::event::MouseScrollDelta::LineDelta(x, y) => {
            println!("鼠标滚轮: x={:.1}, y={:.1}", x, y);
        }
        winit::event::MouseScrollDelta::PixelDelta(pos) => {
            println!("鼠标滚轮像素: x={:.1}, y={:.1}", pos.x, pos.y);
        }
    }
}

/// 输入状态查询
pub struct InputQuery;

impl InputQuery {
    /// 检查是否按下移动键
    pub fn is_moving(state: &AppState) -> bool {
        state.key_w || state.key_a || state.key_s || state.key_d
    }

    /// 获取移动方向（归一化）
    pub fn get_movement_direction(state: &AppState) -> (f32, f32) {
        let mut x = 0.0_f32;
        let mut y = 0.0_f32;

        if state.key_w {
            y -= 1.0;
        }
        if state.key_s {
            y += 1.0;
        }
        if state.key_a {
            x -= 1.0;
        }
        if state.key_d {
            x += 1.0;
        }

        // 归一化
        let length = (x * x + y * y).sqrt();
        if length > 0.0 {
            x /= length;
            y /= length;
        }

        (x, y)
    }

    /// 检查是否按下跳跃键
    pub fn is_jumping(state: &AppState) -> bool {
        state.key_space
    }

    /// 检查是否按下冲刺键
    pub fn is_sprinting(state: &AppState) -> bool {
        state.key_shift
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_moving() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        assert_eq!(InputQuery::is_moving(&state), false);

        state.key_w = true;
        assert_eq!(InputQuery::is_moving(&state), true);
    }

    #[test]
    fn test_movement_direction() {
        let mut state = AppState::new("Test".to_string(), 800, 600);

        // 向上
        state.key_w = true;
        let (x, y) = InputQuery::get_movement_direction(&state);
        assert_eq!(x, 0.0);
        assert_eq!(y, -1.0);

        // 向右上（对角线）
        state.key_d = true;
        let (x, y) = InputQuery::get_movement_direction(&state);
        assert!((x - 0.707).abs() < 0.01); // 归一化后约为 0.707
        assert!((y - (-0.707)).abs() < 0.01);
    }

    #[test]
    fn test_jumping() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        assert_eq!(InputQuery::is_jumping(&state), false);

        state.key_space = true;
        assert_eq!(InputQuery::is_jumping(&state), true);
    }

    #[test]
    fn test_sprinting() {
        let mut state = AppState::new("Test".to_string(), 800, 600);
        assert_eq!(InputQuery::is_sprinting(&state), false);

        state.key_shift = true;
        assert_eq!(InputQuery::is_sprinting(&state), true);
    }
}
