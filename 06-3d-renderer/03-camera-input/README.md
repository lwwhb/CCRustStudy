# 模块 6.3：相机与输入系统

## 🎯 学习目标

- 理解相机投影原理
- 实现第一人称相机
- 实现第三人称相机
- 掌握输入处理系统
- 学习相机动画

## 📚 核心概念

### 1. 相机类型

**透视相机 (Perspective Camera)**
- 模拟人眼视角
- 有近平面和远平面
- FOV（视野角度）

**正交相机 (Orthographic Camera)**
- 平行投影
- 用于 2D 或技术图纸

### 2. 相机变换

```rust
view_matrix = look_at(position, target, up)
projection_matrix = perspective(fov, aspect, near, far)
view_projection = projection * view
```

### 3. 第一人称相机

- 基于位置和朝向
- WASD 移动
- 鼠标控制视角

### 4. 第三人称相机

- 围绕目标旋转
- 距离控制
- 碰撞检测

## 💻 实战项目：相机系统

实现完整的相机和输入系统：
1. 透视相机
2. 第一人称控制器
3. 第三人称控制器
4. 输入管理器
5. 相机动画

## 🧪 测试

```bash
cargo test
```

## 📝 练习

1. 添加相机平滑移动
2. 实现相机碰撞检测
3. 添加相机震动效果
4. 实现轨道相机

## 🔗 深入阅读

- [Camera Systems](https://learnopengl.com/Getting-started/Camera)
- [Input Handling](https://docs.rs/winit/)
- [Camera Animation](https://www.gamedeveloper.com/programming/camera-control-in-3d-games)

## ✅ 检查清单

- [ ] 理解相机投影
- [ ] 实现透视相机
- [ ] 实现第一人称控制
- [ ] 实现第三人称控制
- [ ] 处理输入事件
- [ ] 完成所有测试
