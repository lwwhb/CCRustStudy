/// 四元数
///
/// 演示四元数旋转和插值

use nalgebra::{UnitQuaternion, Vector3};
use std::f32::consts::PI;

/// 演示四元数基础
pub fn demonstrate_quaternion_basics() {
    println!("=== 四元数基础 ===\n");

    // 单位四元数（无旋转）
    let identity = UnitQuaternion::<f32>::identity();
    println!("单位四元数: {:?}", identity);

    // 从轴角创建四元数
    let axis = Vector3::z_axis();
    let angle = PI / 2.0; // 90 度
    let rotation = UnitQuaternion::from_axis_angle(&axis, angle);
    println!("绕 Z 轴旋转 90°: {:?}", rotation);

    // 从欧拉角创建
    let euler = UnitQuaternion::from_euler_angles(0.0, 0.0, PI / 4.0);
    println!("从欧拉角 (0, 0, 45°): {:?}", euler);
}

/// 演示四元数旋转
pub fn demonstrate_quaternion_rotation() {
    println!("\n=== 四元数旋转 ===\n");

    // 创建旋转四元数（绕 Z 轴旋转 90 度）
    let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.0);

    // 应用旋转到点
    let point = Vector3::new(1.0, 0.0, 0.0);
    let rotated = rotation * point;

    println!("原始点: {:?}", point);
    println!("旋转后: {:?}", rotated);
    println!("预期: (0, 1, 0)");
}

/// 演示四元数组合
pub fn demonstrate_quaternion_composition() {
    println!("\n=== 四元数组合 ===\n");

    // 先绕 Z 轴旋转 45 度
    let rot1 = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 4.0);

    // 再绕 Y 轴旋转 45 度
    let rot2 = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI / 4.0);

    // 组合旋转
    let combined = rot2 * rot1;

    let point = Vector3::new(1.0, 0.0, 0.0);
    println!("原始点: {:?}", point);

    let after_rot1 = rot1 * point;
    println!("第一次旋转后: {:?}", after_rot1);

    let after_rot2 = rot2 * after_rot1;
    println!("第二次旋转后: {:?}", after_rot2);

    let direct = combined * point;
    println!("直接组合旋转: {:?}", direct);
}

/// 演示四元数插值 (Slerp)
pub fn demonstrate_slerp() {
    println!("\n=== 球面线性插值 (Slerp) ===\n");

    // 起始旋转：无旋转
    let start = UnitQuaternion::identity();

    // 结束旋转：绕 Z 轴旋转 90 度
    let end = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.0);

    println!("起始旋转: {:?}", start);
    println!("结束旋转: {:?}", end);
    println!();

    // 插值
    for i in 0..=5 {
        let t = i as f32 / 5.0;
        let interpolated = start.slerp(&end, t);

        let point = Vector3::new(1.0, 0.0, 0.0);
        let rotated = interpolated * point;

        println!("t = {:.1}: {:?}", t, rotated);
    }
}

/// 演示四元数求逆
pub fn demonstrate_quaternion_inverse() {
    println!("\n=== 四元数求逆 ===\n");

    let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 4.0);
    let inverse = rotation.inverse();

    println!("旋转: {:?}", rotation);
    println!("逆旋转: {:?}", inverse);

    // 验证：旋转 * 逆旋转 = 单位四元数
    let identity = rotation * inverse;
    println!("旋转 × 逆旋转: {:?}", identity);

    // 应用旋转再应用逆旋转
    let point = Vector3::new(1.0, 0.0, 0.0);
    let rotated = rotation * point;
    let back = inverse * rotated;

    println!("\n原始点: {:?}", point);
    println!("旋转后: {:?}", rotated);
    println!("逆旋转后: {:?}", back);
}

/// 演示四元数转换为矩阵
pub fn demonstrate_quaternion_to_matrix() {
    println!("\n=== 四元数转矩阵 ===\n");

    let quaternion = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 4.0);
    let matrix = quaternion.to_homogeneous();

    println!("四元数: {:?}", quaternion);
    println!("对应的旋转矩阵:");
    println!("{}", matrix);
}

/// 演示从两个向量创建旋转
pub fn demonstrate_rotation_between() {
    println!("\n=== 从两个向量创建旋转 ===\n");

    let from = Vector3::new(1.0_f32, 0.0, 0.0);
    let to = Vector3::new(0.0_f32, 1.0, 0.0);

    if let Some(rotation) = UnitQuaternion::rotation_between(&from, &to) {
        println!("从向量: {:?}", from);
        println!("到向量: {:?}", to);
        println!("旋转四元数: {:?}", rotation);

        let result = rotation * from;
        println!("应用旋转后: {:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quaternion_identity() {
        let identity = UnitQuaternion::identity();
        let point = Vector3::new(1.0, 2.0, 3.0);
        let result = identity * point;
        assert_eq!(result, point);
    }

    #[test]
    fn test_quaternion_rotation() {
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.0);
        let point = Vector3::new(1.0, 0.0, 0.0);
        let result = rotation * point;

        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
        assert!((result.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_quaternion_inverse() {
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 4.0);
        let inverse = rotation.inverse();
        let identity = rotation * inverse;

        let point = Vector3::new(1.0, 0.0, 0.0);
        let result = identity * point;

        assert!((result.x - point.x).abs() < 1e-6);
        assert!((result.y - point.y).abs() < 1e-6);
        assert!((result.z - point.z).abs() < 1e-6);
    }

    #[test]
    fn test_slerp() {
        let start = UnitQuaternion::identity();
        let end = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.0);

        let mid = start.slerp(&end, 0.5);
        let point = Vector3::new(1.0, 0.0, 0.0);
        let result = mid * point;

        // 中点应该旋转 45 度
        let expected_angle = PI / 4.0;
        let expected_x = expected_angle.cos();
        let expected_y = expected_angle.sin();

        assert!((result.x - expected_x).abs() < 1e-6);
        assert!((result.y - expected_y).abs() < 1e-6);
    }

    #[test]
    fn test_rotation_between() {
        let from = Vector3::new(1.0_f32, 0.0, 0.0);
        let to = Vector3::new(0.0_f32, 1.0, 0.0);

        if let Some(rotation) = UnitQuaternion::rotation_between(&from, &to) {
            let result = rotation * from;

            assert!((result.x - to.x).abs() < 1e-6);
            assert!((result.y - to.y).abs() < 1e-6);
            assert!((result.z - to.z).abs() < 1e-6);
        }
    }
}
