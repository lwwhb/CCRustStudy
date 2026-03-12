use crate::traits::{Area, Draw, Perimeter, Shape, ShapeInfo};

/// 矩形结构体
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// 创建新的矩形
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    /// 判断是否为正方形
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("矩形:");
        println!("  宽度: {:.2}, 高度: {:.2}", self.width, self.height);
        if self.is_square() {
            println!("  (这是一个正方形)");
        }
        println!("  面积: {:.2}", self.area());
        println!("  周长: {:.2}", self.perimeter());
    }
}

impl Shape for Rectangle {
    fn shape_type(&self) -> &str {
        if self.is_square() {
            "正方形"
        } else {
            "矩形"
        }
    }
}

impl ShapeInfo for Rectangle {
    fn name(&self) -> &str {
        if self.is_square() {
            "正方形"
        } else {
            "矩形"
        }
    }

    fn description(&self) -> String {
        format!("宽 {:.2}，高 {:.2} 的矩形", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.perimeter(), 60.0);
    }

    #[test]
    fn test_square() {
        let square = Rectangle::new(10.0, 10.0);
        assert!(square.is_square());
        assert_eq!(square.name(), "正方形");
    }

    #[test]
    fn test_not_square() {
        let rect = Rectangle::new(10.0, 20.0);
        assert!(!rect.is_square());
        assert_eq!(rect.name(), "矩形");
    }
}
