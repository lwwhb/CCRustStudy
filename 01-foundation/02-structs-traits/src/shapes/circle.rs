use crate::traits::{Area, Draw, Perimeter, Shape, ShapeInfo};

/// 圆形结构体
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    /// 创建新的圆形
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Circle {
    fn shape_type(&self) -> &str {
        "圆形"
    }
}

impl Draw for Circle {
    fn draw(&self) {
        println!("圆形:");
        println!("  半径: {:.2}", self.radius);
        println!("  面积: {:.2}", self.area());
        println!("  周长: {:.2}", self.perimeter());
    }
}

impl ShapeInfo for Circle {
    fn name(&self) -> &str {
        "圆形"
    }

    fn description(&self) -> String {
        format!("半径为 {:.2} 的圆形", self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(5.0);
        let area = circle.area();
        assert!((area - 78.54).abs() < 0.01);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(5.0);
        let perimeter = circle.perimeter();
        assert!((perimeter - 31.42).abs() < 0.01);
    }

    #[test]
    fn test_circle_info() {
        let circle = Circle::new(5.0);
        assert_eq!(circle.name(), "圆形");
        assert!(circle.description().contains("5.00"));
    }
}
