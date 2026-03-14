use glam::{Mat4, Quat, Vec3};
use std::collections::HashMap;
use thiserror::Error;

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("资源未找到: {0}")]
    ResourceNotFound(String),

    #[error("初始化失败: {0}")]
    InitializationFailed(String),

    #[error("渲染失败: {0}")]
    RenderFailed(String),
}

// ============================================================================
// 资源 ID 系统
// ============================================================================

/// 资源 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(u64);

impl ResourceId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// 资源 ID 生成器
#[derive(Debug)]
pub struct ResourceIdGenerator {
    next_id: u64,
}

impl ResourceIdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn generate(&mut self) -> ResourceId {
        let id = ResourceId(self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for ResourceIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 变换系统
// ============================================================================

/// 3D 变换
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 网格数据
// ============================================================================

/// 顶点数据
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

/// 网格
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn cube() -> Self {
        let vertices = vec![
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(vertices, indices)
    }
}

// ============================================================================
// 材质系统
// ============================================================================

/// 材质
#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub albedo: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
}

impl Material {
    pub fn new(name: String) -> Self {
        Self {
            name,
            albedo: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
        }
    }

    pub fn default_material() -> Self {
        Self::new("Default".to_string())
    }
}

// ============================================================================
// 资源管理器
// ============================================================================

/// 资源管理器
#[derive(Debug)]
pub struct ResourceManager {
    id_generator: ResourceIdGenerator,
    meshes: HashMap<ResourceId, Mesh>,
    materials: HashMap<ResourceId, Material>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            id_generator: ResourceIdGenerator::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
        }
    }

    /// 添加网格
    pub fn add_mesh(&mut self, mesh: Mesh) -> ResourceId {
        let id = self.id_generator.generate();
        self.meshes.insert(id, mesh);
        id
    }

    /// 获取网格
    pub fn get_mesh(&self, id: ResourceId) -> Option<&Mesh> {
        self.meshes.get(&id)
    }

    /// 添加材质
    pub fn add_material(&mut self, material: Material) -> ResourceId {
        let id = self.id_generator.generate();
        self.materials.insert(id, material);
        id
    }

    /// 获取材质
    pub fn get_material(&self, id: ResourceId) -> Option<&Material> {
        self.materials.get(&id)
    }

    /// 资源统计
    pub fn stats(&self) -> ResourceStats {
        ResourceStats {
            mesh_count: self.meshes.len(),
            material_count: self.materials.len(),
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 资源统计
#[derive(Debug, Clone, Copy)]
pub struct ResourceStats {
    pub mesh_count: usize,
    pub material_count: usize,
}

// ============================================================================
// 场景图
// ============================================================================

/// 场景节点
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<ResourceId>,
    pub material: Option<ResourceId>,
    pub children: Vec<SceneNode>,
    pub visible: bool,
}

impl SceneNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            transform: Transform::new(),
            mesh: None,
            material: None,
            children: Vec::new(),
            visible: true,
        }
    }

    pub fn with_mesh(mut self, mesh_id: ResourceId) -> Self {
        self.mesh = Some(mesh_id);
        self
    }

    pub fn with_material(mut self, material_id: ResourceId) -> Self {
        self.material = Some(material_id);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }
}

/// 场景
#[derive(Debug)]
pub struct Scene {
    pub name: String,
    pub root: SceneNode,
}

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            root: SceneNode::new(format!("{}_root", name)),
        }
    }

    /// 遍历场景节点
    pub fn traverse<F>(&self, mut visitor: F)
    where
        F: FnMut(&SceneNode, Mat4),
    {
        self.traverse_node(&self.root, Mat4::IDENTITY, &mut visitor);
    }

    fn traverse_node<F>(&self, node: &SceneNode, parent_transform: Mat4, visitor: &mut F)
    where
        F: FnMut(&SceneNode, Mat4),
    {
        if !node.visible {
            return;
        }

        let world_transform = parent_transform * node.transform.to_matrix();
        visitor(node, world_transform);

        for child in &node.children {
            self.traverse_node(child, world_transform, visitor);
        }
    }
}

// ============================================================================
// 渲染器接口
// ============================================================================

/// 渲染器配置
#[derive(Debug, Clone)]
pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub msaa_samples: u32,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            vsync: true,
            msaa_samples: 1,
        }
    }
}

/// 渲染器
pub struct Renderer {
    config: RendererConfig,
    resource_manager: ResourceManager,
}

impl Renderer {
    pub fn new(config: RendererConfig) -> Result<Self, RendererError> {
        Ok(Self {
            config,
            resource_manager: ResourceManager::new(),
        })
    }

    pub fn resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn render(&mut self, scene: &Scene) -> Result<(), RendererError> {
        // 模拟渲染过程
        let mut render_count = 0;

        scene.traverse(|node, _transform| {
            if node.mesh.is_some() {
                render_count += 1;
            }
        });

        println!("渲染了 {} 个对象", render_count);
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
    }

    pub fn config(&self) -> &RendererConfig {
        &self.config
    }
}

// ============================================================================
// 主函数
// ============================================================================

fn main() -> Result<(), RendererError> {
    println!("=== 3D 渲染器架构演示 ===\n");

    // 创建渲染器
    let config = RendererConfig::default();
    let mut renderer = Renderer::new(config)?;

    println!("渲染器初始化成功");
    println!("分辨率: {}x{}", renderer.config().width, renderer.config().height);

    // 创建资源
    let cube_mesh = Mesh::cube();
    let mesh_id = renderer.resource_manager_mut().add_mesh(cube_mesh);

    let material = Material::default_material();
    let material_id = renderer.resource_manager_mut().add_material(material);

    println!("\n资源加载:");
    let stats = renderer.resource_manager().stats();
    println!("- 网格数量: {}", stats.mesh_count);
    println!("- 材质数量: {}", stats.material_count);

    // 创建场景
    let mut scene = Scene::new("MainScene".to_string());

    // 添加立方体
    let cube_node = SceneNode::new("Cube".to_string())
        .with_mesh(mesh_id)
        .with_material(material_id)
        .with_transform(Transform::from_position(Vec3::new(0.0, 0.0, -5.0)));

    scene.root.add_child(cube_node);

    println!("\n场景构建完成");

    // 渲染
    println!("\n开始渲染...");
    renderer.render(&scene)?;

    println!("\n渲染完成！");

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_id_generator() {
        let mut generator = ResourceIdGenerator::new();
        let id1 = generator.generate();
        let id2 = generator.generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_transform() {
        let transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let matrix = transform.to_matrix();
        assert_eq!(matrix.w_axis.x, 1.0);
        assert_eq!(matrix.w_axis.y, 2.0);
        assert_eq!(matrix.w_axis.z, 3.0);
    }

    #[test]
    fn test_mesh_creation() {
        let mesh = Mesh::cube();
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
    }

    #[test]
    fn test_resource_manager() {
        let mut manager = ResourceManager::new();
        let mesh = Mesh::cube();
        let id = manager.add_mesh(mesh);
        assert!(manager.get_mesh(id).is_some());
    }

    #[test]
    fn test_material() {
        let material = Material::default_material();
        assert_eq!(material.name, "Default");
        assert_eq!(material.metallic, 0.0);
    }

    #[test]
    fn test_scene_node() {
        let mut node = SceneNode::new("Test".to_string());
        let child = SceneNode::new("Child".to_string());
        node.add_child(child);
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_scene_traversal() {
        let scene = Scene::new("Test".to_string());
        let mut count = 0;
        scene.traverse(|_node, _transform| {
            count += 1;
        });
        assert_eq!(count, 1); // 只有根节点
    }

    #[test]
    fn test_renderer_creation() {
        let config = RendererConfig::default();
        let renderer = Renderer::new(config);
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_renderer_resize() {
        let config = RendererConfig::default();
        let mut renderer = Renderer::new(config).unwrap();
        renderer.resize(1024, 768);
        assert_eq!(renderer.config().width, 1024);
        assert_eq!(renderer.config().height, 768);
    }

    #[test]
    fn test_resource_stats() {
        let mut manager = ResourceManager::new();
        manager.add_mesh(Mesh::cube());
        manager.add_material(Material::default_material());
        let stats = manager.stats();
        assert_eq!(stats.mesh_count, 1);
        assert_eq!(stats.material_count, 1);
    }
}
