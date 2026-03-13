/// 矩阵变换
///
/// 演示矩阵的基本变换操作

use nalgebra::{Matrix4, Vector3, Vector4, Point3};
use std::f32::consts::PI;

/// 演示平移变换
pub fn demonstrate_translation() {
    println!("=== 平移变换 ===\n");

    let translation = Matrix4::new_translation(&Vector3::new(5.0, 3.0, 2.0));
    println!("平移矩阵 (5, 3, 2):");
    println!("{}", translation);

    let point = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let transformed = translation * point;
    println!("变换前: {:?}", point);
    println!("变换后: {:?}", transformed);
}

/// 演示旋转变换
pub fn demonstrate_rotation() {
    println!("\n=== 旋转变换 ===\n");

    // 绕 Z 轴旋转 90 度
    let rotation_z = Matrix4::from_euler_angles(0.0, 0.0, PI / 2.0);
    println!("绕 Z 轴旋转 90°:");

    let point = Vector4::new(1.0, 0.0, 0.0, 1.0);
    let rotated = rotation_z * point;
    println!("变换前: {:?}", point);
    println!("变换后: {:?}", rotated);

    // 绕 Y 轴旋转 45 度
    let rotation_y = Matrix4::from_euler_angles(0.0, PI / 4.0, 0.0);
    println!("\n绕 Y 轴旋转 45°:");
    let point2 = Vector4::new(1.0, 0.0, 0.0, 1.0);
    let rotated2 = rotation_y * point2;
    println!("变换前: {:?}", point2);
    println!("变换后: {:?}", rotated2);
}

/// 演示缩放变换
pub fn demonstrate_scaling() {
    println!("\n=== 缩放变换 ===\n");

    // 均匀缩放
    let uniform_scale = Matrix4::new_scaling(2.0);
    println!("均匀缩放 (2x):");

    let point = Vector4::new(1.0, 2.0, 3.0, 1.0);
    let scaled = uniform_scale * point;
    println!("变换前: {:?}", point);
    println!("变换后: {:?}", scaled);

    // 非均匀缩放
    let non_uniform_scale = Matrix4::new_nonuniform_scaling(&Vector3::new(2.0, 3.0, 4.0));
    println!("\n非均匀缩放 (2x, 3x, 4x):");
    let scaled2 = non_uniform_scale * point;
    println!("变换前: {:?}", point);
    println!("变换后: {:?}", scaled2);
}

/// 演示变换组合
pub fn demonstrate_transform_composition() {
    println!("\n=== 变换组合 ===\n");

    // 先缩放，再旋转，最后平移
    let scale = Matrix4::new_scaling(2.0);
    let rotation = Matrix4::from_euler_angles(0.0, 0.0, PI / 4.0);
    let translation = Matrix4::new_translation(&Vector3::new(5.0, 0.0, 0.0));

    // 注意：变换顺序是从右到左
    let combined = translation * rotation * scale;

    let point = Vector4::new(1.0, 0.0, 0.0, 1.0);
    println!("原始点: {:?}", point);

    let scaled_point = scale * point;
    println!("缩放后: {:?}", scaled_point);

    let rotated_point = rotation * scaled_point;
    println!("旋转后: {:?}", rotated_point);

    let final_point = translation * rotated_point;
    println!("平移后: {:?}", final_point);

    let direct = combined * point;
    println!("直接变换: {:?}", direct);
}

/// 演示矩阵求逆
pub fn demonstrate_inverse() {
    println!("\n=== 矩阵求逆 ===\n");

    let translation = Matrix4::new_translation(&Vector3::new(5.0, 3.0, 2.0));
    println!("平移矩阵:");
    println!("{}", translation);

    if let Some(inverse) = translation.try_inverse() {
        println!("逆矩阵:");
        println!("{}", inverse);

        let identity = translation * inverse;
        println!("矩阵 × 逆矩阵 = 单位矩阵:");
        println!("{}", identity);
    }
}

/// 演示视图矩阵
pub fn demonstrate_view_matrix() {
    println!("\n=== 视图矩阵 (Look At) ===\n");

    let eye = Point3::new(0.0, 0.0, 5.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    let view = Matrix4::look_at_rh(&eye, &target, &up);

    println!("相机位置: {:?}", eye);
    println!("目标位置: {:?}", target);
    println!("上方向: {:?}", up);
    println!("视图矩阵:");
    println!("{}", view);
}

/// 演示投影矩阵
pub fn demonstrate_projection() {
    println!("\n=== 投影矩阵 ===\n");

    // 透视投影
    let aspect = 16.0 / 9.0;
    let fovy = PI / 4.0; // 45 度
    let near = 0.1;
    let far = 100.0;

    let perspective = Matrix4::new_perspective(aspect, fovy, near, far);
    println!("透视投影矩阵:");
    println!("{}", perspective);

    // 正交投影
    let left = -10.0;
    let right = 10.0;
    let bottom = -10.0;
    let top = 10.0;
    let near_ortho = 0.1;
    let far_ortho = 100.0;

    let orthographic = Matrix4::new_orthographic(left, right, bottom, top, near_ortho, far_ortho);
    println!("\n正交投影矩阵:");
    println!("{}", orthographic);
}

/// 演示模型-视图-投影 (MVP) 矩阵
pub fn demonstrate_mvp() {
    println!("\n=== MVP 矩阵 ===\n");

    // 模型矩阵：将物体放置在世界空间
    let model = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -5.0))
        * Matrix4::from_euler_angles(0.0, PI / 4.0, 0.0);

    // 视图矩阵：相机变换
    let view = Matrix4::look_at_rh(
        &Point3::new(0.0, 2.0, 10.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    // 投影矩阵：透视投影
    let projection = Matrix4::new_perspective(16.0 / 9.0, PI / 4.0, 0.1, 100.0);

    // MVP 矩阵
    let mvp = projection * view * model;

    println!("模型矩阵:");
    println!("{}", model);
    println!("\n视图矩阵:");
    println!("{}", view);
    println!("\n投影矩阵:");
    println!("{}", projection);
    println!("\nMVP 矩阵:");
    println!("{}", mvp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let translation = Matrix4::new_translation(&Vector3::new(5.0, 3.0, 2.0));
        let point = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let result = translation * point;
        assert_eq!(result, Vector4::new(6.0, 4.0, 3.0, 1.0));
    }

    #[test]
    fn test_scaling() {
        let scale = Matrix4::new_scaling(2.0);
        let point = Vector4::new(1.0, 2.0, 3.0, 1.0);
        let result = scale * point;
        assert_eq!(result, Vector4::new(2.0, 4.0, 6.0, 1.0));
    }

    #[test]
    fn test_identity() {
        let identity = Matrix4::identity();
        let point = Vector4::new(1.0, 2.0, 3.0, 1.0);
        let result = identity * point;
        assert_eq!(result, point);
    }

    #[test]
    fn test_inverse() {
        let translation = Matrix4::new_translation(&Vector3::new(5.0, 3.0, 2.0));
        let inverse = translation.try_inverse().unwrap();
        let identity = translation * inverse;

        // 检查是否接近单位矩阵
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0_f32 } else { 0.0_f32 };
                assert!((identity[(i, j)] - expected).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn test_rotation_90_degrees() {
        let rotation = Matrix4::from_euler_angles(0.0, 0.0, PI / 2.0);
        let point = Vector4::new(1.0, 0.0, 0.0, 1.0);
        let result = rotation * point;

        // 旋转 90 度后，(1, 0, 0) 应该变成 (0, 1, 0)
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
        assert!((result.z - 0.0).abs() < 1e-6);
    }
}
