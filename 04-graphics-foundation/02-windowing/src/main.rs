mod app;
mod input;

use app::AppState;
use input::handle_window_event;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

fn main() {
    println!("=== Winit 窗口演示 ===\n");
    println!("控制说明:");
    println!("  WASD - 移动");
    println!("  Space - 跳跃");
    println!("  Shift - 冲刺");
    println!("  R - 重置状态");
    println!("  P - 打印状态");
    println!("  F11 - 切换全屏");
    println!("  ESC - 退出\n");

    // 创建事件循环
    let event_loop = EventLoop::new().unwrap();

    // 创建窗口
    let window_attributes = Window::default_attributes()
        .with_title("Winit 窗口演示")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

    let window = event_loop.create_window(window_attributes).unwrap();

    // 创建应用状态
    let mut state = AppState::new("Winit Demo".to_string(), 800, 600);
    state.player_x = 400.0;
    state.player_y = 300.0;

    let mut last_update = Instant::now();

    // 运行事件循环
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => {
                let should_close = handle_window_event(&event, &mut state);
                if should_close {
                    elwt.exit();
                }

                // 处理窗口关闭
                if let WindowEvent::CloseRequested = event {
                    elwt.exit();
                }
            }
            Event::AboutToWait => {
                // 更新逻辑
                let now = Instant::now();
                let delta_time = now.duration_since(last_update).as_secs_f32();
                last_update = now;

                // 更新玩家位置
                state.update_player(delta_time);

                // 更新 FPS
                state.update_fps();

                // 请求重绘
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // 这里应该进行渲染，但我们只是演示窗口和输入
                // 在后续模块中会添加实际的渲染代码
            }
            _ => {}
        }
    }).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let state = AppState::new("Test".to_string(), 800, 600);
        assert_eq!(state.width, 800);
        assert_eq!(state.height, 600);
    }
}
