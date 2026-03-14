use glam::Vec3;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("文件未找到: {0}")]
    FileNotFound(String),

    #[error("解析失败: {0}")]
    ParseError(String),

    #[error("IO 错误: {0}")]
    IoError(String),

    #[error("资源已存在: {0}")]
    ResourceExists(String),
}

// ============================================================================
// 资源数据结构
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

/// 网格数据
#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(name: String, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            name,
            vertices,
            indices,
        }
    }

    /// 创建立方体网格
    pub fn cube(name: String) -> Self {
        let vertices = vec![
            // 前面
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 1.0]),
            // 后面
            Vertex::new([0.5, -0.5, -0.5], [0.0, 0.0, -1.0], [0.0, 0.0]),
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0], [1.0, 0.0]),
            Vertex::new([-0.5, 0.5, -0.5], [0.0, 0.0, -1.0], [1.0, 1.0]),
            Vertex::new([0.5, 0.5, -0.5], [0.0, 0.0, -1.0], [0.0, 1.0]),
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, // 前面
            4, 5, 6, 6, 7, 4, // 后面
        ];

        Self::new(name, vertices, indices)
    }

    /// 创建平面网格
    pub fn plane(name: String) -> Self {
        let vertices = vec![
            Vertex::new([-1.0, 0.0, -1.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
            Vertex::new([1.0, 0.0, -1.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex::new([1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
            Vertex::new([-1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(name, vertices, indices)
    }
}

/// 纹理数据
#[derive(Debug, Clone)]
pub struct Texture {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn new(name: String, width: u32, height: u32, data: Vec<u8>) -> Self {
        Self {
            name,
            width,
            height,
            data,
        }
    }

    /// 创建纯色纹理
    pub fn solid_color(name: String, width: u32, height: u32, color: [u8; 4]) -> Self {
        let size = (width * height * 4) as usize;
        let mut data = Vec::with_capacity(size);

        for _ in 0..(width * height) {
            data.extend_from_slice(&color);
        }

        Self::new(name, width, height, data)
    }

    /// 创建棋盘纹理
    pub fn checkerboard(name: String, size: u32) -> Self {
        let mut data = Vec::with_capacity((size * size * 4) as usize);

        for y in 0..size {
            for x in 0..size {
                let is_white = (x / 8 + y / 8) % 2 == 0;
                let color = if is_white {
                    [255, 255, 255, 255]
                } else {
                    [0, 0, 0, 255]
                };
                data.extend_from_slice(&color);
            }
        }

        Self::new(name, size, size, data)
    }
}

/// 材质数据
#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub albedo: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub texture_name: Option<String>,
}

impl Material {
    pub fn new(name: String) -> Self {
        Self {
            name,
            albedo: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            texture_name: None,
        }
    }

    pub fn with_albedo(mut self, albedo: [f32; 4]) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn with_texture(mut self, texture_name: String) -> Self {
        self.texture_name = Some(texture_name);
        self
    }
}

/// 模型数据（包含多个网格）
#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn new(name: String) -> Self {
        Self {
            name,
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }
}

// ============================================================================
// 简化的 OBJ 解析器
// ============================================================================

/// OBJ 文件加载器
pub struct ObjLoader;

impl ObjLoader {
    /// 解析 OBJ 数据
    pub fn parse(name: String, data: &str) -> Result<Mesh, ResourceError> {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for line in data.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" if parts.len() >= 4 => {
                    // 顶点位置
                    let x = parts[1].parse().map_err(|_| {
                        ResourceError::ParseError("无效的顶点坐标".to_string())
                    })?;
                    let y = parts[2].parse().map_err(|_| {
                        ResourceError::ParseError("无效的顶点坐标".to_string())
                    })?;
                    let z = parts[3].parse().map_err(|_| {
                        ResourceError::ParseError("无效的顶点坐标".to_string())
                    })?;
                    positions.push([x, y, z]);
                }
                "vn" if parts.len() >= 4 => {
                    // 法线
                    let x = parts[1].parse().map_err(|_| {
                        ResourceError::ParseError("无效的法线".to_string())
                    })?;
                    let y = parts[2].parse().map_err(|_| {
                        ResourceError::ParseError("无效的法线".to_string())
                    })?;
                    let z = parts[3].parse().map_err(|_| {
                        ResourceError::ParseError("无效的法线".to_string())
                    })?;
                    normals.push([x, y, z]);
                }
                "vt" if parts.len() >= 3 => {
                    // 纹理坐标
                    let u = parts[1].parse().map_err(|_| {
                        ResourceError::ParseError("无效的纹理坐标".to_string())
                    })?;
                    let v = parts[2].parse().map_err(|_| {
                        ResourceError::ParseError("无效的纹理坐标".to_string())
                    })?;
                    tex_coords.push([u, v]);
                }
                "f" if parts.len() >= 4 => {
                    // 面（简化处理，只支持三角形）
                    for i in 1..=3 {
                        let vertex_data: Vec<&str> = parts[i].split('/').collect();
                        let pos_idx: usize = vertex_data[0].parse::<usize>().map_err(|_| {
                            ResourceError::ParseError("无效的顶点索引".to_string())
                        })? - 1;

                        let position = positions.get(pos_idx).copied().unwrap_or([0.0, 0.0, 0.0]);
                        let normal = if vertex_data.len() > 2 && !vertex_data[2].is_empty() {
                            let norm_idx: usize = vertex_data[2].parse::<usize>().map_err(|_| {
                                ResourceError::ParseError("无效的法线索引".to_string())
                            })? - 1;
                            normals.get(norm_idx).copied().unwrap_or([0.0, 1.0, 0.0])
                        } else {
                            [0.0, 1.0, 0.0]
                        };
                        let tex_coord = if vertex_data.len() > 1 && !vertex_data[1].is_empty() {
                            let tex_idx: usize = vertex_data[1].parse::<usize>().map_err(|_| {
                                ResourceError::ParseError("无效的纹理坐标索引".to_string())
                            })? - 1;
                            tex_coords.get(tex_idx).copied().unwrap_or([0.0, 0.0])
                        } else {
                            [0.0, 0.0]
                        };

                        vertices.push(Vertex::new(position, normal, tex_coord));
                        indices.push((vertices.len() - 1) as u32);
                    }
                }
                _ => {}
            }
        }

        Ok(Mesh::new(name, vertices, indices))
    }
}

// ============================================================================
// 资源加载器
// ============================================================================

/// 资源加载器
pub struct ResourceLoader {
    meshes: Arc<RwLock<HashMap<String, Arc<Mesh>>>>,
    textures: Arc<RwLock<HashMap<String, Arc<Texture>>>>,
    materials: Arc<RwLock<HashMap<String, Arc<Material>>>>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self {
            meshes: Arc::new(RwLock::new(HashMap::new())),
            textures: Arc::new(RwLock::new(HashMap::new())),
            materials: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 加载网格（从内存数据）
    pub async fn load_mesh_from_data(
        &self,
        name: String,
        data: &str,
    ) -> Result<Arc<Mesh>, ResourceError> {
        // 检查是否已加载
        {
            let meshes = self.meshes.read().await;
            if let Some(mesh) = meshes.get(&name) {
                return Ok(Arc::clone(mesh));
            }
        }

        // 解析 OBJ 数据
        let mesh = ObjLoader::parse(name.clone(), data)?;
        let mesh_arc = Arc::new(mesh);

        // 缓存
        {
            let mut meshes = self.meshes.write().await;
            meshes.insert(name, Arc::clone(&mesh_arc));
        }

        Ok(mesh_arc)
    }

    /// 加载预定义网格
    pub async fn load_primitive_mesh(
        &self,
        name: String,
        primitive: PrimitiveMesh,
    ) -> Result<Arc<Mesh>, ResourceError> {
        // 检查是否已加载
        {
            let meshes = self.meshes.read().await;
            if let Some(mesh) = meshes.get(&name) {
                return Ok(Arc::clone(mesh));
            }
        }

        let mesh = match primitive {
            PrimitiveMesh::Cube => Mesh::cube(name.clone()),
            PrimitiveMesh::Plane => Mesh::plane(name.clone()),
        };

        let mesh_arc = Arc::new(mesh);

        // 缓存
        {
            let mut meshes = self.meshes.write().await;
            meshes.insert(name, Arc::clone(&mesh_arc));
        }

        Ok(mesh_arc)
    }

    /// 加载纹理
    pub async fn load_texture(
        &self,
        name: String,
        texture: Texture,
    ) -> Result<Arc<Texture>, ResourceError> {
        // 检查是否已加载
        {
            let textures = self.textures.read().await;
            if let Some(tex) = textures.get(&name) {
                return Ok(Arc::clone(tex));
            }
        }

        let texture_arc = Arc::new(texture);

        // 缓存
        {
            let mut textures = self.textures.write().await;
            textures.insert(name, Arc::clone(&texture_arc));
        }

        Ok(texture_arc)
    }

    /// 加载材质
    pub async fn load_material(
        &self,
        name: String,
        material: Material,
    ) -> Result<Arc<Material>, ResourceError> {
        // 检查是否已加载
        {
            let materials = self.materials.read().await;
            if let Some(mat) = materials.get(&name) {
                return Ok(Arc::clone(mat));
            }
        }

        let material_arc = Arc::new(material);

        // 缓存
        {
            let mut materials = self.materials.write().await;
            materials.insert(name, Arc::clone(&material_arc));
        }

        Ok(material_arc)
    }

    /// 获取网格
    pub async fn get_mesh(&self, name: &str) -> Option<Arc<Mesh>> {
        let meshes = self.meshes.read().await;
        meshes.get(name).map(Arc::clone)
    }

    /// 获取纹理
    pub async fn get_texture(&self, name: &str) -> Option<Arc<Texture>> {
        let textures = self.textures.read().await;
        textures.get(name).map(Arc::clone)
    }

    /// 获取材质
    pub async fn get_material(&self, name: &str) -> Option<Arc<Material>> {
        let materials = self.materials.read().await;
        materials.get(name).map(Arc::clone)
    }

    /// 获取统计信息
    pub async fn stats(&self) -> ResourceStats {
        let meshes = self.meshes.read().await;
        let textures = self.textures.read().await;
        let materials = self.materials.read().await;

        ResourceStats {
            mesh_count: meshes.len(),
            texture_count: textures.len(),
            material_count: materials.len(),
        }
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// 预定义网格类型
#[derive(Debug, Clone, Copy)]
pub enum PrimitiveMesh {
    Cube,
    Plane,
}

/// 资源统计
#[derive(Debug, Clone, Copy)]
pub struct ResourceStats {
    pub mesh_count: usize,
    pub texture_count: usize,
    pub material_count: usize,
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
    println!("资源加载系统");
    println!("支持的功能:");
    println!("- OBJ 模型加载");
    println!("- 纹理加载");
    println!("- 材质管理");
    println!("- 异步资源缓存");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::new([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], [0.5, 0.5]);
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.normal, [0.0, 1.0, 0.0]);
        assert_eq!(vertex.tex_coords, [0.5, 0.5]);
    }

    #[test]
    fn test_mesh_cube() {
        let mesh = Mesh::cube("test_cube".to_string());
        assert_eq!(mesh.name, "test_cube");
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.indices.is_empty());
    }

    #[test]
    fn test_mesh_plane() {
        let mesh = Mesh::plane("test_plane".to_string());
        assert_eq!(mesh.name, "test_plane");
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
    }

    #[test]
    fn test_texture_solid_color() {
        let texture = Texture::solid_color("white".to_string(), 2, 2, [255, 255, 255, 255]);
        assert_eq!(texture.width, 2);
        assert_eq!(texture.height, 2);
        assert_eq!(texture.data.len(), 16); // 2x2x4
    }

    #[test]
    fn test_texture_checkerboard() {
        let texture = Texture::checkerboard("checker".to_string(), 16);
        assert_eq!(texture.width, 16);
        assert_eq!(texture.height, 16);
        assert_eq!(texture.data.len(), 16 * 16 * 4);
    }

    #[test]
    fn test_material_creation() {
        let material = Material::new("test_mat".to_string())
            .with_albedo([1.0, 0.0, 0.0, 1.0])
            .with_texture("texture.png".to_string());

        assert_eq!(material.name, "test_mat");
        assert_eq!(material.albedo, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(material.texture_name, Some("texture.png".to_string()));
    }

    #[test]
    fn test_obj_parser_simple() {
        let obj_data = r#"
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.0 1.0 0.0
vn 0.0 0.0 1.0
vt 0.0 0.0
vt 1.0 0.0
vt 0.0 1.0
f 1/1/1 2/2/1 3/3/1
"#;

        let mesh = ObjLoader::parse("test".to_string(), obj_data).unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.indices.len(), 3);
    }

    #[tokio::test]
    async fn test_resource_loader_mesh() {
        let loader = ResourceLoader::new();

        let mesh = loader
            .load_primitive_mesh("cube".to_string(), PrimitiveMesh::Cube)
            .await
            .unwrap();

        assert_eq!(mesh.name, "cube");

        // 测试缓存
        let mesh2 = loader.get_mesh("cube").await.unwrap();
        assert_eq!(mesh2.name, "cube");
    }

    #[tokio::test]
    async fn test_resource_loader_texture() {
        let loader = ResourceLoader::new();

        let texture = Texture::solid_color("white".to_string(), 4, 4, [255, 255, 255, 255]);
        let tex_arc = loader
            .load_texture("white".to_string(), texture)
            .await
            .unwrap();

        assert_eq!(tex_arc.name, "white");

        // 测试缓存
        let tex2 = loader.get_texture("white").await.unwrap();
        assert_eq!(tex2.name, "white");
    }

    #[tokio::test]
    async fn test_resource_loader_stats() {
        let loader = ResourceLoader::new();

        loader
            .load_primitive_mesh("cube".to_string(), PrimitiveMesh::Cube)
            .await
            .unwrap();

        let texture = Texture::solid_color("white".to_string(), 4, 4, [255, 255, 255, 255]);
        loader
            .load_texture("white".to_string(), texture)
            .await
            .unwrap();

        let stats = loader.stats().await;
        assert_eq!(stats.mesh_count, 1);
        assert_eq!(stats.texture_count, 1);
    }
}
