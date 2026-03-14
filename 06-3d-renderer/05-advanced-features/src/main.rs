use glam::{Mat4, Vec3, Vec4};
use std::time::{Duration, Instant};

// ============================================================================
// 包围体系统
// ============================================================================

/// 轴对齐包围盒 (AABB)
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// 从顶点列表创建 AABB
    pub fn from_vertices(vertices: &[[f32; 3]]) -> Self {
        if vertices.is_empty() {
            return Self::new(Vec3::ZERO, Vec3::ZERO);
        }

        let mut min = Vec3::from(vertices[0]);
        let mut max = Vec3::from(vertices[0]);

        for vertex in vertices.iter().skip(1) {
            let v = Vec3::from(*vertex);
            min = min.min(v);
            max = max.max(v);
        }

        Self::new(min, max)
    }

    /// 获取中心点
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// 获取尺寸
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// 获取半尺寸
    pub fn half_size(&self) -> Vec3 {
        self.size() * 0.5
    }

    /// 变换 AABB
    pub fn transform(&self, matrix: Mat4) -> Self {
        let corners = [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ];

        let mut min = matrix.transform_point3(corners[0]);
        let mut max = min;

        for corner in corners.iter().skip(1) {
            let transformed = matrix.transform_point3(*corner);
            min = min.min(transformed);
            max = max.max(transformed);
        }

        Self::new(min, max)
    }

    /// 检测与另一个 AABB 的相交
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

/// 球体包围体
#[derive(Debug, Clone, Copy)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl BoundingSphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// 从 AABB 创建
    pub fn from_aabb(aabb: &Aabb) -> Self {
        let center = aabb.center();
        let radius = aabb.half_size().length();
        Self::new(center, radius)
    }

    /// 检测与另一个球体的相交
    pub fn intersects(&self, other: &BoundingSphere) -> bool {
        let distance = (self.center - other.center).length();
        distance <= (self.radius + other.radius)
    }
}

// ============================================================================
// 视锥剔除
// ============================================================================

/// 平面
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f32) -> Self {
        Self { normal, distance }
    }

    /// 从点和法线创建
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize();
        let distance = -normal.dot(point);
        Self::new(normal, distance)
    }

    /// 计算点到平面的距离
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

/// 视锥体
#[derive(Debug, Clone)]
pub struct Frustum {
    pub planes: [Plane; 6], // 左、右、上、下、近、远
}

impl Frustum {
    /// 从视图投影矩阵创建
    pub fn from_matrix(vp: Mat4) -> Self {
        let rows = [
            Vec4::new(vp.x_axis.x, vp.y_axis.x, vp.z_axis.x, vp.w_axis.x),
            Vec4::new(vp.x_axis.y, vp.y_axis.y, vp.z_axis.y, vp.w_axis.y),
            Vec4::new(vp.x_axis.z, vp.y_axis.z, vp.z_axis.z, vp.w_axis.z),
            Vec4::new(vp.x_axis.w, vp.y_axis.w, vp.z_axis.w, vp.w_axis.w),
        ];

        let planes = [
            // 左平面
            Self::normalize_plane(rows[3] + rows[0]),
            // 右平面
            Self::normalize_plane(rows[3] - rows[0]),
            // 下平面
            Self::normalize_plane(rows[3] + rows[1]),
            // 上平面
            Self::normalize_plane(rows[3] - rows[1]),
            // 近平面
            Self::normalize_plane(rows[3] + rows[2]),
            // 远平面
            Self::normalize_plane(rows[3] - rows[2]),
        ];

        Self { planes }
    }

    fn normalize_plane(v: Vec4) -> Plane {
        let length = Vec3::new(v.x, v.y, v.z).length();
        Plane::new(
            Vec3::new(v.x / length, v.y / length, v.z / length),
            v.w / length,
        )
    }

    /// 检测球体是否在视锥内
    pub fn contains_sphere(&self, sphere: &BoundingSphere) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(sphere.center) < -sphere.radius {
                return false;
            }
        }
        true
    }

    /// 检测 AABB 是否在视锥内
    pub fn contains_aabb(&self, aabb: &Aabb) -> bool {
        for plane in &self.planes {
            let center = aabb.center();
            let half_size = aabb.half_size();

            let r = half_size.x * plane.normal.x.abs()
                + half_size.y * plane.normal.y.abs()
                + half_size.z * plane.normal.z.abs();

            if plane.distance_to_point(center) < -r {
                return false;
            }
        }
        true
    }
}

// ============================================================================
// LOD 系统
// ============================================================================

/// LOD 级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodLevel {
    High,
    Medium,
    Low,
}

/// LOD 配置
#[derive(Debug, Clone, Copy)]
pub struct LodConfig {
    pub high_distance: f32,
    pub medium_distance: f32,
}

impl LodConfig {
    pub fn new(high_distance: f32, medium_distance: f32) -> Self {
        Self {
            high_distance,
            medium_distance,
        }
    }

    /// 根据距离选择 LOD 级别
    pub fn select_level(&self, distance: f32) -> LodLevel {
        if distance < self.high_distance {
            LodLevel::High
        } else if distance < self.medium_distance {
            LodLevel::Medium
        } else {
            LodLevel::Low
        }
    }
}

impl Default for LodConfig {
    fn default() -> Self {
        Self::new(10.0, 50.0)
    }
}

// ============================================================================
// 性能统计
// ============================================================================

/// 渲染统计
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderStats {
    pub frame_time: Duration,
    pub draw_calls: u32,
    pub vertices_rendered: u32,
    pub triangles_rendered: u32,
    pub objects_culled: u32,
    pub objects_rendered: u32,
}

impl RenderStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fps(&self) -> f32 {
        if self.frame_time.as_secs_f32() > 0.0 {
            1.0 / self.frame_time.as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// 性能计时器
pub struct PerformanceTimer {
    start: Instant,
}

impl PerformanceTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

impl Default for PerformanceTimer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    println!("3D 渲染器 - 高级特性与优化");
    println!("============================");
    println!();
    println!("功能:");
    println!("- 视锥剔除");
    println!("- LOD 系统");
    println!("- 性能统计");
    println!("- 包围体计算");
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_creation() {
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(aabb.center(), Vec3::ZERO);
        assert_eq!(aabb.size(), Vec3::splat(2.0));
    }

    #[test]
    fn test_aabb_from_vertices() {
        let vertices = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let aabb = Aabb::from_vertices(&vertices);
        assert_eq!(aabb.min, Vec3::ZERO);
        assert_eq!(aabb.max, Vec3::ONE);
    }

    #[test]
    fn test_aabb_intersection() {
        let aabb1 = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let aabb2 = Aabb::new(Vec3::splat(0.5), Vec3::splat(1.5));
        assert!(aabb1.intersects(&aabb2));

        let aabb3 = Aabb::new(Vec3::splat(2.0), Vec3::splat(3.0));
        assert!(!aabb1.intersects(&aabb3));
    }

    #[test]
    fn test_bounding_sphere() {
        let sphere1 = BoundingSphere::new(Vec3::ZERO, 1.0);
        let sphere2 = BoundingSphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0);
        assert!(sphere1.intersects(&sphere2));

        let sphere3 = BoundingSphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0);
        assert!(!sphere1.intersects(&sphere3));
    }

    #[test]
    fn test_plane() {
        let plane = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        assert_eq!(plane.distance_to_point(Vec3::new(0.0, 1.0, 0.0)), 1.0);
        assert_eq!(plane.distance_to_point(Vec3::new(0.0, -1.0, 0.0)), -1.0);
    }

    #[test]
    fn test_lod_selection() {
        let config = LodConfig::new(10.0, 50.0);
        assert_eq!(config.select_level(5.0), LodLevel::High);
        assert_eq!(config.select_level(30.0), LodLevel::Medium);
        assert_eq!(config.select_level(100.0), LodLevel::Low);
    }

    #[test]
    fn test_render_stats() {
        let mut stats = RenderStats::new();
        stats.frame_time = Duration::from_millis(16);
        assert!((stats.fps() - 62.5).abs() < 0.1);
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::new();
        std::thread::sleep(Duration::from_millis(10));
        assert!(timer.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_frustum_sphere_culling() {
        let vp = Mat4::perspective_rh(45.0_f32.to_radians(), 1.0, 0.1, 100.0)
            * Mat4::look_at_rh(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), Vec3::Y);
        let frustum = Frustum::from_matrix(vp);

        let sphere_inside = BoundingSphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);
        assert!(frustum.contains_sphere(&sphere_inside));

        let sphere_outside = BoundingSphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0);
        assert!(!frustum.contains_sphere(&sphere_outside));
    }

    #[test]
    fn test_aabb_transform() {
        let aabb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let transform = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let transformed = aabb.transform(transform);
        assert_eq!(transformed.min, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(transformed.max, Vec3::new(2.0, 1.0, 1.0));
    }
}
