
//trait
pub trait Area {
  fn area(&self) -> f64;
}

//struct
pub struct Circle {
  pub radius: f64,
}
pub struct Square {
  pub side: f64,
}
pub struct Triangle {
  pub base: f64,
  pub height: f64,
}

impl Area for Circle {
  fn area(&self) -> f64 {
      std::f64::consts::PI * self.radius.powi(2)
  }
}

impl Area for Square {
  fn area(&self) -> f64 {
      self.side.powi(2)
  }
}

impl Area for Triangle {
  fn area(&self) -> f64 {
      self.base * self.height / 2.0
  }
}

//求图形面积的函数，接收实现了特定trait的泛型参数
pub fn calculate_area<T: Area>(graphic: T) -> f64 {
  graphic.area()
}