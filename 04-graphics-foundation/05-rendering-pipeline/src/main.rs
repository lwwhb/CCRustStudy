use glam::{Mat4, Quat, Vec3};
use std::collections::HashMap;

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

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.rotation = Quat::from_axis_angle(axis, angle) * self.rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
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

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }
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

    /// 创建立方体网格
    pub fn cube() -> Self {
        let vertices = vec![
            // 前面
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(vertices, indices)
    }

    /// 创建平面网格
    pub fn plane() -> Self {
        let vertices = vec![
            Vertex::new([-1.0, 0.0, -1.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
            Vertex::new([1.0, 0.0, -1.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex::new([1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
            Vertex::new([-1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(vertices, indices)
    }
}

// ============================================================================
// 场景图
// ============================================================================

/// 场景节点 ID
pub type NodeId = usize;

/// 场景节点
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: NodeId,
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<Mesh>,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub visible: bool,
}

impl SceneNode {
    pub fn new(id: NodeId, name: String) -> Self {
        Self {
            id,
            name,
            transform: Transform::new(),
            mesh: None,
            parent: None,
            children: Vec::new(),
            visible: true,
        }
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// 场景图
#[derive(Debug)]
pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    next_id: NodeId,
    root_nodes: Vec<NodeId>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_id: 0,
            root_nodes: Vec::new(),
        }
    }

    /// 添加节点
    pub fn add_node(&mut self, mut node: SceneNode) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        node.id = id;

        if node.parent.is_none() {
            self.root_nodes.push(id);
        }

        self.nodes.insert(id, node);
        id
    }

    /// 添加子节点
    pub fn add_child(&mut self, parent_id: NodeId, mut child: SceneNode) -> Option<NodeId> {
        let child_id = self.next_id;
        self.next_id += 1;
        child.id = child_id;
        child.parent = Some(parent_id);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
            self.nodes.insert(child_id, child);
            Some(child_id)
        } else {
            None
        }
    }

    /// 获取节点
    pub fn get_node(&self, id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&id)
    }

    /// 获取可变节点
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&id)
    }

    /// 计算世界变换矩阵
    pub fn get_world_transform(&self, id: NodeId) -> Mat4 {
        let mut transform = Mat4::IDENTITY;
        let mut current_id = Some(id);

        while let Some(node_id) = current_id {
            if let Some(node) = self.nodes.get(&node_id) {
                transform = node.transform.to_matrix() * transform;
                current_id = node.parent;
            } else {
                break;
            }
        }

        transform
    }

    /// 遍历所有可见节点
    pub fn traverse<F>(&self, mut visitor: F)
    where
        F: FnMut(&SceneNode, Mat4),
    {
        for &root_id in &self.root_nodes {
            self.traverse_node(root_id, Mat4::IDENTITY, &mut visitor);
        }
    }

    fn traverse_node<F>(&self, node_id: NodeId, parent_transform: Mat4, visitor: &mut F)
    where
        F: FnMut(&SceneNode, Mat4),
    {
        if let Some(node) = self.nodes.get(&node_id) {
            if !node.visible {
                return;
            }

            let world_transform = parent_transform * node.transform.to_matrix();
            visitor(node, world_transform);

            for &child_id in &node.children {
                self.traverse_node(child_id, world_transform, visitor);
            }
        }
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 相机
// ============================================================================

/// 相机
#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, aspect: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

// ============================================================================
// 渲染管线配置
// ============================================================================

/// 深度测试配置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthTest {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
}

/// 剔除模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

/// 混合模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    None,
    Alpha,
    Additive,
    Multiply,
}

/// 渲染管线配置
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub depth_test: DepthTest,
    pub depth_write: bool,
    pub cull_mode: CullMode,
    pub blend_mode: BlendMode,
}

impl PipelineConfig {
    pub fn default_opaque() -> Self {
        Self {
            depth_test: DepthTest::Less,
            depth_write: true,
            cull_mode: CullMode::Back,
            blend_mode: BlendMode::None,
        }
    }

    pub fn default_transparent() -> Self {
        Self {
            depth_test: DepthTest::Less,
            depth_write: false,
            cull_mode: CullMode::None,
            blend_mode: BlendMode::Alpha,
        }
    }
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self::default_opaque()
    }
}

// ============================================================================
// 渲染器
// ============================================================================

/// 渲染命令
#[derive(Debug, Clone)]
pub struct RenderCommand {
    pub mesh: Mesh,
    pub transform: Mat4,
    pub pipeline_config: PipelineConfig,
}

/// 渲染器
#[derive(Debug)]
pub struct Renderer {
    commands: Vec<RenderCommand>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn submit(&mut self, mesh: Mesh, transform: Mat4, config: PipelineConfig) {
        self.commands.push(RenderCommand {
            mesh,
            transform,
            pipeline_config: config,
        });
    }

    pub fn render(&mut self, scene: &SceneGraph, camera: &Camera) {
        self.commands.clear();

        // 收集渲染命令
        scene.traverse(|node, world_transform| {
            if let Some(ref mesh) = node.mesh {
                self.submit(
                    mesh.clone(),
                    camera.view_projection_matrix() * world_transform,
                    PipelineConfig::default(),
                );
            }
        });

        // 排序（透明物体需要从后往前渲染）
        self.sort_commands();
    }

    fn sort_commands(&mut self) {
        // 简单排序：不透明物体在前，透明物体在后
        self.commands.sort_by(|a, b| {
            let a_transparent = matches!(a.pipeline_config.blend_mode, BlendMode::Alpha);
            let b_transparent = matches!(b.pipeline_config.blend_mode, BlendMode::Alpha);
            a_transparent.cmp(&b_transparent)
        });
    }

    pub fn get_commands(&self) -> &[RenderCommand] {
        &self.commands
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
    println!("3D 渲染管线演示");
    println!("================");

    // 创建场景图
    let mut scene = SceneGraph::new();

    // 添加立方体
    let cube_node = SceneNode::new(0, "Cube".to_string())
        .with_mesh(Mesh::cube())
        .with_transform(Transform::from_position(Vec3::new(0.0, 1.0, 0.0)));
    scene.add_node(cube_node);

    // 添加平面
    let plane_node = SceneNode::new(0, "Plane".to_string())
        .with_mesh(Mesh::plane())
        .with_transform(Transform::from_position(Vec3::ZERO));
    scene.add_node(plane_node);

    // 创建相机
    let camera = Camera::new(
        Vec3::new(3.0, 3.0, 3.0),
        Vec3::ZERO,
        16.0 / 9.0,
    );

    // 创建渲染器
    let mut renderer = Renderer::new();
    renderer.render(&scene, &camera);

    println!("场景节点数: {}", scene.nodes.len());
    println!("渲染命令数: {}", renderer.get_commands().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let mut transform = Transform::new();
        assert_eq!(transform.position, Vec3::ZERO);
        assert_eq!(transform.scale, Vec3::ONE);

        transform.translate(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(transform.position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_mesh_creation() {
        let cube = Mesh::cube();
        assert!(!cube.vertices.is_empty());
        assert!(!cube.indices.is_empty());

        let plane = Mesh::plane();
        assert_eq!(plane.vertices.len(), 4);
        assert_eq!(plane.indices.len(), 6);
    }

    #[test]
    fn test_scene_graph() {
        let mut scene = SceneGraph::new();

        let node = SceneNode::new(0, "Test".to_string());
        let id = scene.add_node(node);

        assert!(scene.get_node(id).is_some());
        assert_eq!(scene.get_node(id).unwrap().name, "Test");
    }

    #[test]
    fn test_scene_hierarchy() {
        let mut scene = SceneGraph::new();

        let parent = SceneNode::new(0, "Parent".to_string());
        let parent_id = scene.add_node(parent);

        let child = SceneNode::new(0, "Child".to_string());
        let child_id = scene.add_child(parent_id, child);

        assert!(child_id.is_some());
        let parent_node = scene.get_node(parent_id).unwrap();
        assert_eq!(parent_node.children.len(), 1);
    }

    #[test]
    fn test_world_transform() {
        let mut scene = SceneGraph::new();

        let mut parent_transform = Transform::new();
        parent_transform.position = Vec3::new(1.0, 0.0, 0.0);

        let parent = SceneNode::new(0, "Parent".to_string())
            .with_transform(parent_transform);
        let parent_id = scene.add_node(parent);

        let mut child_transform = Transform::new();
        child_transform.position = Vec3::new(0.0, 1.0, 0.0);

        let child = SceneNode::new(0, "Child".to_string())
            .with_transform(child_transform);
        let child_id = scene.add_child(parent_id, child).unwrap();

        let world_transform = scene.get_world_transform(child_id);
        let world_pos = world_transform.transform_point3(Vec3::ZERO);

        // 子节点的世界位置应该是 (1, 1, 0)
        assert!((world_pos.x - 1.0).abs() < 0.001);
        assert!((world_pos.y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_camera() {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            16.0 / 9.0,
        );

        let view = camera.view_matrix();
        let proj = camera.projection_matrix();
        let vp = camera.view_projection_matrix();

        assert_ne!(view, Mat4::IDENTITY);
        assert_ne!(proj, Mat4::IDENTITY);
        assert_ne!(vp, Mat4::IDENTITY);
    }

    #[test]
    fn test_pipeline_config() {
        let opaque = PipelineConfig::default_opaque();
        assert_eq!(opaque.depth_test, DepthTest::Less);
        assert!(opaque.depth_write);
        assert_eq!(opaque.cull_mode, CullMode::Back);

        let transparent = PipelineConfig::default_transparent();
        assert!(!transparent.depth_write);
        assert_eq!(transparent.blend_mode, BlendMode::Alpha);
    }

    #[test]
    fn test_renderer() {
        let mut scene = SceneGraph::new();
        let node = SceneNode::new(0, "Cube".to_string())
            .with_mesh(Mesh::cube());
        scene.add_node(node);

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            1.0,
        );

        let mut renderer = Renderer::new();
        renderer.render(&scene, &camera);

        assert_eq!(renderer.get_commands().len(), 1);
    }

    #[test]
    fn test_scene_traversal() {
        let mut scene = SceneGraph::new();

        let node1 = SceneNode::new(0, "Node1".to_string());
        let node2 = SceneNode::new(0, "Node2".to_string());

        scene.add_node(node1);
        scene.add_node(node2);

        let mut count = 0;
        scene.traverse(|_node, _transform| {
            count += 1;
        });

        assert_eq!(count, 2);
    }
}
