mod vectors;
mod matrices;
mod quaternions;
mod camera;

use vectors::*;
use matrices::*;
use quaternions::*;
use camera::*;

fn main() {
    println!("=== 图形数学演示 ===\n");

    // 演示 1：向量运算
    println!("=== 演示 1：向量运算 ===");
    demonstrate_vector2();
    demonstrate_vector3();
    vectors::demonstrate_projection();
    demonstrate_angles();
    demonstrate_lerp();
    demonstrate_unit_vectors();
    println!();

    // 演示 2：矩阵变换
    println!("=== 演示 2：矩阵变换 ===");
    demonstrate_translation();
    demonstrate_rotation();
    demonstrate_scaling();
    demonstrate_transform_composition();
    demonstrate_inverse();
    demonstrate_view_matrix();
    matrices::demonstrate_projection();
    demonstrate_mvp();
    println!();

    // 演示 3：四元数
    println!("=== 演示 3：四元数 ===");
    demonstrate_quaternion_basics();
    demonstrate_quaternion_rotation();
    demonstrate_quaternion_composition();
    demonstrate_slerp();
    demonstrate_quaternion_inverse();
    demonstrate_quaternion_to_matrix();
    demonstrate_rotation_between();
    println!();

    // 演示 4：相机系统
    println!("=== 演示 4：相机系统 ===");
    demonstrate_basic_camera();
    demonstrate_camera_directions();
    demonstrate_camera_movement();
    demonstrate_orbit_camera();
    demonstrate_fps_camera();
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_demonstrations() {
        // 向量
        demonstrate_vector2();
        demonstrate_vector3();
        vectors::demonstrate_projection();
        demonstrate_angles();
        demonstrate_lerp();
        demonstrate_unit_vectors();

        // 矩阵
        demonstrate_translation();
        demonstrate_rotation();
        demonstrate_scaling();
        demonstrate_transform_composition();
        demonstrate_inverse();
        demonstrate_view_matrix();
        matrices::demonstrate_projection();
        demonstrate_mvp();

        // 四元数
        demonstrate_quaternion_basics();
        demonstrate_quaternion_rotation();
        demonstrate_quaternion_composition();
        demonstrate_slerp();
        demonstrate_quaternion_inverse();
        demonstrate_quaternion_to_matrix();
        demonstrate_rotation_between();

        // 相机
        demonstrate_basic_camera();
        demonstrate_camera_directions();
        demonstrate_camera_movement();
        demonstrate_orbit_camera();
        demonstrate_fps_camera();
    }
}
