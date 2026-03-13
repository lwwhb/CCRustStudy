/// 相机系统
///
/// 演示相机矩阵和相机控制

use nalgebra::{Matrix4, Point3, Vector3};
use std::f32::consts::PI;

/// 相机结构体
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// 创建新相机
    pub fn new(position: Point3<f32>, target: Point3<f32>) -> Self {
        Camera {
            position,
            target,
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: PI / 4.0, // 45 度
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 100.0,
        }
    }

    /// 获取视图矩阵
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    /// 获取投影矩阵
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }

    /// 获取 VP 矩阵（视图 × 投影）
    pub fn vp_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    /// 获取相机前方向
    pub fn forward(&self) -> Vector3<f32> {
        (self.target - self.position).normalize()
    }

    /// 获取相机右方向
    pub fn right(&self) -> Vector3<f32> {
        self.forward().cross(&self.up).normalize()
    }

    /// 获取相机上方向
    pub fn camera_up(&self) -> Vector3<f32> {
        self.right().cross(&self.forward()).normalize()
    }

    /// 向前移动
    pub fn move_forward(&mut self, distance: f32) {
        let direction = self.forward();
        self.position += direction * distance;
        self.target += direction * distance;
    }

    /// 向右移动
    pub fn move_right(&mut self, distance: f32) {
        let direction = self.right();
        self.position += direction * distance;
        self.target += direction * distance;
    }

    /// 向上移动
    pub fn move_up(&mut self, distance: f32) {
        self.position.y += distance;
        self.target.y += distance;
    }

    /// 绕目标点旋转（轨道相机）
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let offset = self.position - self.target;
        let distance = offset.magnitude();

        // 计算当前的球坐标
        let mut theta = offset.z.atan2(offset.x);
        let mut phi = (offset.y / distance).acos();

        // 更新角度
        theta += yaw;
        phi += pitch;

        // 限制 phi 避免翻转
        phi = phi.clamp(0.01, PI - 0.01);

        // 转换回笛卡尔坐标
        let x = distance * phi.sin() * theta.cos();
        let y = distance * phi.cos();
        let z = distance * phi.sin() * theta.sin();

        self.position = self.target + Vector3::new(x, y, z);
    }
}

/// 演示基本相机
pub fn demonstrate_basic_camera() {
    println!("=== 基本相机 ===\n");

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
    );

    println!("相机位置: {:?}", camera.position);
    println!("目标位置: {:?}", camera.target);
    println!("上方向: {:?}", camera.up);
    println!();

    println!("视图矩阵:");
    println!("{}", camera.view_matrix());
    println!();

    println!("投影矩阵:");
    println!("{}", camera.projection_matrix());
}

/// 演示相机方向
pub fn demonstrate_camera_directions() {
    println!("\n=== 相机方向 ===\n");

    let camera = Camera::new(
        Point3::new(5.0, 3.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
    );

    println!("相机位置: {:?}", camera.position);
    println!("目标位置: {:?}", camera.target);
    println!();

    println!("前方向: {:?}", camera.forward());
    println!("右方向: {:?}", camera.right());
    println!("上方向: {:?}", camera.camera_up());
}

/// 演示相机移动
pub fn demonstrate_camera_movement() {
    println!("\n=== 相机移动 ===\n");

    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
    );

    println!("初始位置: {:?}", camera.position);

    camera.move_forward(2.0);
    println!("向前移动 2.0: {:?}", camera.position);

    camera.move_right(1.0);
    println!("向右移动 1.0: {:?}", camera.position);

    camera.move_up(1.5);
    println!("向上移动 1.5: {:?}", camera.position);
}

/// 演示轨道相机
pub fn demonstrate_orbit_camera() {
    println!("\n=== 轨道相机 ===\n");

    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
    );

    println!("初始位置: {:?}", camera.position);
    println!("目标位置: {:?}", camera.target);

    // 绕 Y 轴旋转 45 度
    camera.orbit(PI / 4.0, 0.0);
    println!("\n绕 Y 轴旋转 45°:");
    println!("新位置: {:?}", camera.position);

    // 向上倾斜 30 度
    camera.orbit(0.0, PI / 6.0);
    println!("\n向上倾斜 30°:");
    println!("新位置: {:?}", camera.position);
}

/// 第一人称相机
pub struct FPSCamera {
    pub position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl FPSCamera {
    pub fn new(position: Point3<f32>) -> Self {
        FPSCamera {
            position,
            yaw: -PI / 2.0, // 朝向 -Z
            pitch: 0.0,
            fov: PI / 4.0,
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn forward(&self) -> Vector3<f32> {
        Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize()
    }

    pub fn right(&self) -> Vector3<f32> {
        Vector3::new(
            self.yaw.sin(),
            0.0,
            -self.yaw.cos(),
        ).normalize()
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let target = self.position + self.forward();
        Matrix4::look_at_rh(&self.position, &target, &Vector3::y())
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }
}

/// 演示第一人称相机
pub fn demonstrate_fps_camera() {
    println!("\n=== 第一人称相机 ===\n");

    let mut camera = FPSCamera::new(Point3::new(0.0, 1.0, 0.0));

    println!("初始位置: {:?}", camera.position);
    println!("初始朝向: {:?}", camera.forward());

    // 向右转 90 度
    camera.yaw += PI / 2.0;
    println!("\n向右转 90°:");
    println!("新朝向: {:?}", camera.forward());

    // 向上看 30 度
    camera.pitch += PI / 6.0;
    println!("\n向上看 30°:");
    println!("新朝向: {:?}", camera.forward());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(
            Point3::new(0.0, 0.0, 5.0),
            Point3::new(0.0, 0.0, 0.0),
        );

        assert_eq!(camera.position, Point3::new(0.0, 0.0, 5.0));
        assert_eq!(camera.target, Point3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_camera_forward() {
        let camera = Camera::new(
            Point3::new(0.0, 0.0, 5.0),
            Point3::new(0.0, 0.0, 0.0),
        );

        let forward = camera.forward();
        assert!((forward.x - 0.0).abs() < 1e-6);
        assert!((forward.y - 0.0).abs() < 1e-6);
        assert!((forward.z - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::new(
            Point3::new(0.0, 0.0, 5.0),
            Point3::new(0.0, 0.0, 0.0),
        );

        camera.move_forward(1.0);
        assert!((camera.position.z - 4.0).abs() < 1e-6);

        camera.move_up(2.0);
        assert!((camera.position.y - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_fps_camera() {
        let camera = FPSCamera::new(Point3::new(0.0, 0.0, 0.0));
        let forward = camera.forward();

        // 默认朝向应该是 -Z 方向
        assert!((forward.x - 0.0).abs() < 1e-6);
        assert!((forward.y - 0.0).abs() < 1e-6);
        assert!((forward.z - (-1.0)).abs() < 1e-6);
    }
}
