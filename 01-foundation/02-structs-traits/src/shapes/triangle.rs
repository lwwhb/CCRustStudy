use crate::traits::{Area, Draw, Perimeter, Shape, ShapeInfo};

/// 三角形结构体
#[derive(Debug, Clone)]
pub struct Triangle {
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Triangle {
    /// 创建新的三角形
    ///
    /// # 参数
    /// * `side_a` - 第一条边
    /// * `side_b` - 第二条边
    /// * `side_c` - 第三条边
    ///
    /// # 返回
    /// * `Some(Triangle)` - 如果三边可以构成三角形
    /// * `None` - 如果三边不能构成三角形
    pub fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Self> {
        // 检查三角形不等式：任意两边之和大于第三边
        if side_a + side_b > side_c && side_a + side_c > side_b && side_b + side_c > side_a {
            Some(Triangle {
                side_a,
                side_b,
                side_c,
            })
        } else {
            None
        }
    }

    /// 判断是否为等边三角形
    pub fn is_equilateral(&self) -> bool {
        (self.side_a - self.side_b).abs() < f64::EPSILON
            && (self.side_b - self.side_c).abs() < f64::EPSILON
    }

    /// 判断是否为等腰三角形
    pub fn is_isosceles(&self) -> bool {
        (self.side_a - self.side_b).abs() < f64::EPSILON
            || (self.side_b - self.side_c).abs() < f64::EPSILON
            || (self.side_a - self.side_c).abs() < f64::EPSILON
    }

    /// 判断是否为直角三角形
    pub fn is_right_angled(&self) -> bool {
        let mut sides = vec![self.side_a, self.side_b, self.side_c];
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // 勾股定理：a² + b² = c²
        (sides[0].powi(2) + sides[1].powi(2) - sides[2].powi(2)).abs() < 0.0001
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        // 使用海伦公式：A = √[s(s-a)(s-b)(s-c)]
        // 其中 s = (a+b+c)/2
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
}

impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("三角形:");
        println!(
            "  边长: {:.2}, {:.2}, {:.2}",
            self.side_a, self.side_b, self.side_c
        );

        if self.is_equilateral() {
            println!("  (等边三角形)");
        } else if self.is_isosceles() {
            println!("  (等腰三角形)");
        }

        if self.is_right_angled() {
            println!("  (直角三角形)");
        }

        println!("  面积: {:.2}", self.area());
        println!("  周长: {:.2}", self.perimeter());
    }
}

impl Shape for Triangle {
    fn shape_type(&self) -> &str {
        if self.is_equilateral() {
            "等边三角形"
        } else if self.is_isosceles() {
            "等腰三角形"
        } else if self.is_right_angled() {
            "直角三角形"
        } else {
            "三角形"
        }
    }
}

impl ShapeInfo for Triangle {
    fn name(&self) -> &str {
        if self.is_equilateral() {
            "等边三角形"
        } else if self.is_isosceles() {
            "等腰三角形"
        } else if self.is_right_angled() {
            "直角三角形"
        } else {
            "三角形"
        }
    }

    fn description(&self) -> String {
        format!(
            "边长为 {:.2}, {:.2}, {:.2} 的三角形",
            self.side_a, self.side_b, self.side_c
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_triangle() {
        let triangle = Triangle::new(3.0, 4.0, 5.0);
        assert!(triangle.is_some());
    }

    #[test]
    fn test_invalid_triangle() {
        let triangle = Triangle::new(1.0, 2.0, 10.0);
        assert!(triangle.is_none());
    }

    #[test]
    fn test_right_angled_triangle() {
        let triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert!(triangle.is_right_angled());
        assert_eq!(triangle.area(), 6.0);
        assert_eq!(triangle.perimeter(), 12.0);
    }

    #[test]
    fn test_equilateral_triangle() {
        let triangle = Triangle::new(5.0, 5.0, 5.0).unwrap();
        assert!(triangle.is_equilateral());
        assert!(triangle.is_isosceles());
    }

    #[test]
    fn test_isosceles_triangle() {
        let triangle = Triangle::new(5.0, 5.0, 8.0).unwrap();
        assert!(!triangle.is_equilateral());
        assert!(triangle.is_isosceles());
    }

    #[test]
    fn test_triangle_name() {
        let right = Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert_eq!(right.name(), "直角三角形");

        let equilateral = Triangle::new(5.0, 5.0, 5.0).unwrap();
        assert_eq!(equilateral.name(), "等边三角形");
    }
}
