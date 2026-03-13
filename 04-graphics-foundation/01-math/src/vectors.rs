/// 向量运算
///
/// 演示 2D、3D 和 4D 向量的基本运算

use nalgebra::{Vector2, Vector3, Vector4};

/// 演示 2D 向量运算
pub fn demonstrate_vector2() {
    println!("=== 2D 向量运算 ===\n");

    let v1 = Vector2::new(3.0, 4.0);
    let v2 = Vector2::new(1.0, 2.0);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    // 向量加法
    let sum = v1 + v2;
    println!("v1 + v2 = {:?}", sum);

    // 向量减法
    let diff = v1 - v2;
    println!("v1 - v2 = {:?}", diff);

    // 标量乘法
    let scaled = v1 * 2.0;
    println!("v1 * 2 = {:?}", scaled);

    // 长度（模）
    let length = v1.magnitude();
    println!("||v1|| = {}", length);

    // 归一化
    let normalized = v1.normalize();
    println!("normalize(v1) = {:?}", normalized);
    println!("||normalize(v1)|| = {}", normalized.magnitude());

    // 点积
    let dot = v1.dot(&v2);
    println!("v1 · v2 = {}", dot);
}

/// 演示 3D 向量运算
pub fn demonstrate_vector3() {
    println!("\n=== 3D 向量运算 ===\n");

    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    // 向量加法
    let sum = v1 + v2;
    println!("v1 + v2 = {:?}", sum);

    // 点积
    let dot = v1.dot(&v2);
    println!("v1 · v2 = {}", dot);

    // 叉积
    let cross = v1.cross(&v2);
    println!("v1 × v2 = {:?}", cross);

    // 验证叉积垂直于原向量
    println!("(v1 × v2) · v1 = {}", cross.dot(&v1));
    println!("(v1 × v2) · v2 = {}", cross.dot(&v2));

    // 距离
    let distance = (v1 - v2).magnitude();
    println!("distance(v1, v2) = {}", distance);
}

/// 演示向量投影
pub fn demonstrate_projection() {
    println!("\n=== 向量投影 ===\n");

    let v = Vector3::new(3.0, 4.0, 0.0);
    let onto = Vector3::new(1.0, 0.0, 0.0);

    println!("v = {:?}", v);
    println!("onto = {:?}", onto);

    // 投影公式: proj_onto(v) = (v · onto / ||onto||²) * onto
    let projection = (v.dot(&onto) / onto.magnitude_squared()) * onto;
    println!("projection of v onto onto = {:?}", projection);
}

/// 演示向量角度
pub fn demonstrate_angles() {
    println!("\n=== 向量角度 ===\n");

    let v1 = Vector3::new(1.0_f32, 0.0, 0.0);
    let v2 = Vector3::new(1.0_f32, 1.0, 0.0);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    // 使用点积计算角度: cos(θ) = (v1 · v2) / (||v1|| * ||v2||)
    let cos_angle: f32 = v1.dot(&v2) / (v1.magnitude() * v2.magnitude());
    let angle_rad = cos_angle.acos();
    let angle_deg = angle_rad.to_degrees();

    println!("angle = {} radians", angle_rad);
    println!("angle = {} degrees", angle_deg);
}

/// 演示线性插值
pub fn demonstrate_lerp() {
    println!("\n=== 线性插值 (Lerp) ===\n");

    let start = Vector3::new(0.0, 0.0, 0.0);
    let end = Vector3::new(10.0, 10.0, 10.0);

    println!("start = {:?}", start);
    println!("end = {:?}", end);

    for i in 0..=5 {
        let t = i as f32 / 5.0;
        let interpolated = start.lerp(&end, t);
        println!("lerp(t={:.1}) = {:?}", t, interpolated);
    }
}

/// 演示单位向量
pub fn demonstrate_unit_vectors() {
    println!("\n=== 单位向量 ===\n");

    let x_axis = Vector3::<f32>::x_axis();
    let y_axis = Vector3::<f32>::y_axis();
    let z_axis = Vector3::<f32>::z_axis();

    println!("X axis: {:?}", x_axis);
    println!("Y axis: {:?}", y_axis);
    println!("Z axis: {:?}", z_axis);

    // 验证正交性
    println!("X · Y = {}", x_axis.dot(&y_axis));
    println!("Y · Z = {}", y_axis.dot(&z_axis));
    println!("Z · X = {}", z_axis.dot(&x_axis));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normalization() {
        let v = Vector3::new(3.0_f32, 4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.magnitude() - 1.0_f32).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        let start = Vector3::new(0.0, 0.0, 0.0);
        let end = Vector3::new(10.0, 10.0, 10.0);
        let mid = start.lerp(&end, 0.5);
        assert_eq!(mid, Vector3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_orthogonal_unit_vectors() {
        let x = Vector3::<f32>::x_axis();
        let y = Vector3::<f32>::y_axis();
        let z = Vector3::<f32>::z_axis();

        // 单位向量应该垂直
        assert!((x.dot(&y)).abs() < 1e-6);
        assert!((y.dot(&z)).abs() < 1e-6);
        assert!((z.dot(&x)).abs() < 1e-6);
    }
}
