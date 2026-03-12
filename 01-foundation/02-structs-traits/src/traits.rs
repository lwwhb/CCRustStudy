/// Trait 定义：计算面积
pub trait Area {
    fn area(&self) -> f64;
}

/// Trait 定义：计算周长
pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

/// Trait 定义：绘制图形
pub trait Draw {
    fn draw(&self);
}

/// Trait 定义：获取图形信息
pub trait ShapeInfo {
    fn name(&self) -> &str;
    fn description(&self) -> String;
}

/// 组合 trait：表示一个完整的图形
/// 这个 trait 继承了 Area 和 Perimeter，可以用作 trait 对象
pub trait Shape: Area + Perimeter {
    fn shape_type(&self) -> &str {
        "未知图形"
    }
}
