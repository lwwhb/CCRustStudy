use glam::Vec3;
use std::f32::consts::PI;

// ============================================================================
// 材质系统
// ============================================================================

/// PBR 材质
#[derive(Debug, Clone, Copy)]
pub struct PbrMaterial {
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32, // 环境光遮蔽
}

impl PbrMaterial {
    pub fn new(albedo: Vec3, metallic: f32, roughness: f32) -> Self {
        Self {
            albedo,
            metallic,
            roughness,
            ao: 1.0,
        }
    }

    /// 金属材质
    pub fn metal(albedo: Vec3) -> Self {
        Self::new(albedo, 1.0, 0.2)
    }

    /// 塑料材质
    pub fn plastic(albedo: Vec3) -> Self {
        Self::new(albedo, 0.0, 0.5)
    }

    /// 粗糙材质
    pub fn rough(albedo: Vec3) -> Self {
        Self::new(albedo, 0.0, 0.9)
    }
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self::new(Vec3::ONE, 0.0, 0.5)
    }
}

/// Phong 材质
#[derive(Debug, Clone, Copy)]
pub struct PhongMaterial {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
}

impl PhongMaterial {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self::new(
            Vec3::splat(0.1),
            Vec3::splat(0.8),
            Vec3::splat(0.5),
            32.0,
        )
    }
}

// ============================================================================
// 光源系统
// ============================================================================

/// 方向光
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }

    /// 太阳光
    pub fn sun() -> Self {
        Self::new(Vec3::new(-0.3, -1.0, -0.3), Vec3::ONE, 1.0)
    }
}

/// 点光源
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }
    }

    /// 计算衰减
    pub fn attenuation(&self, distance: f32) -> f32 {
        1.0 / (self.constant + self.linear * distance + self.quadratic * distance * distance)
    }
}

/// 聚光灯
#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub cutoff: f32,      // 内圆锥角
    pub outer_cutoff: f32, // 外圆锥角
}

impl SpotLight {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        color: Vec3,
        intensity: f32,
        cutoff: f32,
    ) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            color,
            intensity,
            cutoff: cutoff.to_radians().cos(),
            outer_cutoff: (cutoff + 5.0).to_radians().cos(),
        }
    }

    /// 计算聚光灯强度
    pub fn spot_intensity(&self, light_dir: Vec3) -> f32 {
        let theta = light_dir.dot(self.direction);
        let epsilon = self.cutoff - self.outer_cutoff;
        ((theta - self.outer_cutoff) / epsilon).clamp(0.0, 1.0)
    }
}

// ============================================================================
// Phong 光照计算
// ============================================================================

/// Phong 光照计算器
pub struct PhongLighting;

impl PhongLighting {
    /// 计算方向光照
    pub fn calculate_directional(
        light: &DirectionalLight,
        material: &PhongMaterial,
        normal: Vec3,
        view_dir: Vec3,
    ) -> Vec3 {
        let light_dir = -light.direction;

        // 环境光
        let ambient = material.ambient * light.color;

        // 漫反射
        let diff = normal.dot(light_dir).max(0.0);
        let diffuse = material.diffuse * diff * light.color;

        // 镜面反射
        let reflect_dir = Self::reflect(-light_dir, normal);
        let spec = view_dir.dot(reflect_dir).max(0.0).powf(material.shininess);
        let specular = material.specular * spec * light.color;

        (ambient + diffuse + specular) * light.intensity
    }

    /// 计算点光源照
    pub fn calculate_point(
        light: &PointLight,
        material: &PhongMaterial,
        frag_pos: Vec3,
        normal: Vec3,
        view_dir: Vec3,
    ) -> Vec3 {
        let light_dir = (light.position - frag_pos).normalize();
        let distance = (light.position - frag_pos).length();
        let attenuation = light.attenuation(distance);

        // 环境光
        let ambient = material.ambient * light.color;

        // 漫反射
        let diff = normal.dot(light_dir).max(0.0);
        let diffuse = material.diffuse * diff * light.color;

        // 镜面反射
        let reflect_dir = Self::reflect(-light_dir, normal);
        let spec = view_dir.dot(reflect_dir).max(0.0).powf(material.shininess);
        let specular = material.specular * spec * light.color;

        (ambient + diffuse + specular) * light.intensity * attenuation
    }

    /// 反射向量
    fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
        incident - 2.0 * incident.dot(normal) * normal
    }
}

// ============================================================================
// PBR 光照计算
// ============================================================================

/// PBR 光照计算器
pub struct PbrLighting;

impl PbrLighting {
    /// 计算方向光 PBR
    pub fn calculate_directional(
        light: &DirectionalLight,
        material: &PbrMaterial,
        normal: Vec3,
        view_dir: Vec3,
    ) -> Vec3 {
        let light_dir = -light.direction;
        let halfway = (view_dir + light_dir).normalize();

        // 法线分布函数 (GGX)
        let ndf = Self::distribution_ggx(normal, halfway, material.roughness);

        // 几何函数
        let geometry = Self::geometry_smith(normal, view_dir, light_dir, material.roughness);

        // 菲涅尔方程
        let f0 = Vec3::splat(0.04).lerp(material.albedo, material.metallic);
        let fresnel = Self::fresnel_schlick(halfway.dot(view_dir).max(0.0), f0);

        // Cook-Torrance BRDF
        let numerator = ndf * geometry * fresnel;
        let denominator = 4.0 * normal.dot(view_dir).max(0.0) * normal.dot(light_dir).max(0.0);
        let specular = numerator / denominator.max(0.001);

        // 能量守恒
        let ks = fresnel;
        let kd = (Vec3::ONE - ks) * (1.0 - material.metallic);

        let n_dot_l = normal.dot(light_dir).max(0.0);
        (kd * material.albedo / PI + specular) * light.color * light.intensity * n_dot_l
    }

    /// GGX 法线分布函数
    fn distribution_ggx(normal: Vec3, halfway: Vec3, roughness: f32) -> f32 {
        let a = roughness * roughness;
        let a2 = a * a;
        let n_dot_h = normal.dot(halfway).max(0.0);
        let n_dot_h2 = n_dot_h * n_dot_h;

        let num = a2;
        let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
        let denom = PI * denom * denom;

        num / denom
    }

    /// Smith 几何函数
    fn geometry_smith(normal: Vec3, view_dir: Vec3, light_dir: Vec3, roughness: f32) -> f32 {
        let n_dot_v = normal.dot(view_dir).max(0.0);
        let n_dot_l = normal.dot(light_dir).max(0.0);
        let ggx2 = Self::geometry_schlick_ggx(n_dot_v, roughness);
        let ggx1 = Self::geometry_schlick_ggx(n_dot_l, roughness);

        ggx1 * ggx2
    }

    fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
        let r = roughness + 1.0;
        let k = (r * r) / 8.0;

        let num = n_dot_v;
        let denom = n_dot_v * (1.0 - k) + k;

        num / denom
    }

    /// Schlick 菲涅尔近似
    fn fresnel_schlick(cos_theta: f32, f0: Vec3) -> Vec3 {
        f0 + (Vec3::ONE - f0) * (1.0 - cos_theta).powf(5.0)
    }
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
    println!("光照与材质系统");

    // 创建材质
    let metal = PbrMaterial::metal(Vec3::new(0.9, 0.9, 0.9));
    println!("金属材质: {:?}", metal);

    // 创建光源
    let sun = DirectionalLight::sun();
    println!("太阳光: {:?}", sun);

    // 计算光照
    let normal = Vec3::Y;
    let view_dir = Vec3::new(0.0, 1.0, 1.0).normalize();
    let color = PbrLighting::calculate_directional(&sun, &metal, normal, view_dir);
    println!("光照颜色: {:?}", color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbr_material_creation() {
        let mat = PbrMaterial::new(Vec3::ONE, 0.5, 0.5);
        assert_eq!(mat.metallic, 0.5);
        assert_eq!(mat.roughness, 0.5);
    }

    #[test]
    fn test_metal_material() {
        let mat = PbrMaterial::metal(Vec3::new(0.9, 0.9, 0.9));
        assert_eq!(mat.metallic, 1.0);
        assert!(mat.roughness < 0.5);
    }

    #[test]
    fn test_directional_light() {
        let light = DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Vec3::ONE, 1.0);
        assert_eq!(light.direction, Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(light.intensity, 1.0);
    }

    #[test]
    fn test_point_light_attenuation() {
        let light = PointLight::new(Vec3::ZERO, Vec3::ONE, 1.0);
        let atten = light.attenuation(1.0);
        assert!(atten > 0.0 && atten <= 1.0);
    }

    #[test]
    fn test_spot_light_intensity() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::ONE,
            1.0,
            30.0,
        );
        let intensity = light.spot_intensity(Vec3::new(0.0, -1.0, 0.0));
        assert!(intensity > 0.0);
    }

    #[test]
    fn test_phong_lighting() {
        let light = DirectionalLight::sun();
        let material = PhongMaterial::default();
        let normal = Vec3::Y;
        let view_dir = Vec3::new(0.0, 1.0, 0.0);

        let color = PhongLighting::calculate_directional(&light, &material, normal, view_dir);
        assert!(color.length() > 0.0);
    }

    #[test]
    fn test_pbr_lighting() {
        let light = DirectionalLight::sun();
        let material = PbrMaterial::default();
        let normal = Vec3::Y;
        let view_dir = Vec3::new(0.0, 1.0, 0.0);

        let color = PbrLighting::calculate_directional(&light, &material, normal, view_dir);
        assert!(color.length() > 0.0);
    }

    #[test]
    fn test_fresnel_schlick() {
        let f0 = Vec3::splat(0.04);
        let fresnel = PbrLighting::fresnel_schlick(0.5, f0);
        assert!(fresnel.x >= f0.x);
    }

    #[test]
    fn test_distribution_ggx() {
        let normal = Vec3::Y;
        let halfway = Vec3::Y;
        let ndf = PbrLighting::distribution_ggx(normal, halfway, 0.5);
        assert!(ndf > 0.0);
    }

    #[test]
    fn test_geometry_smith() {
        let normal = Vec3::Y;
        let view_dir = Vec3::new(0.0, 1.0, 0.0);
        let light_dir = Vec3::new(0.0, 1.0, 0.0);
        let g = PbrLighting::geometry_smith(normal, view_dir, light_dir, 0.5);
        assert!(g > 0.0 && g <= 1.0);
    }
}
