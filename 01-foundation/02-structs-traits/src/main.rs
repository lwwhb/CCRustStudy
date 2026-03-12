mod shapes;
mod traits;

use shapes::{Circle, Rectangle, Triangle};
use traits::{Area, Draw, Perimeter, Shape};

fn main() {
    println!("=== 图形库演示 ===\n");

    // 创建各种图形
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(10.0, 20.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0).expect("无效的三角形");

    // 绘制每个图形
    circle.draw();
    println!();

    rectangle.draw();
    println!();

    triangle.draw();
    println!();

    // 使用 trait 对象实现多态
    // 这里我们创建一个包含不同类型图形的集合
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(rectangle),
        Box::new(triangle),
    ];

    // 计算所有图形的总面积和总周长
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    let total_perimeter: f64 = shapes.iter().map(|s| s.perimeter()).sum();

    println!("=== 统计信息 ===");
    println!("图形总数: {}", shapes.len());
    println!("所有图形总面积: {:.2}", total_area);
    println!("所有图形总周长: {:.2}", total_perimeter);

    // 演示泛型函数
    println!("\n=== 泛型函数演示 ===");
    print_shape_info(&Circle::new(3.0));
    print_shape_info(&Rectangle::new(5.0, 5.0));
}

/// 泛型函数：打印任何实现了 Area 和 Perimeter trait 的图形信息
fn print_shape_info<T>(shape: &T)
where
    T: Area + Perimeter,
{
    println!(
        "面积: {:.2}, 周长: {:.2}",
        shape.area(),
        shape.perimeter()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle::new(5.0);
        assert!((circle.area() - 78.54).abs() < 0.01);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_trait_objects() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle::new(5.0)),
            Box::new(Rectangle::new(10.0, 20.0)),
        ];

        assert_eq!(shapes.len(), 2);
        assert!(shapes[0].area() > 0.0);
        assert!(shapes[1].area() > 0.0);
    }
}

