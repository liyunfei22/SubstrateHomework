mod question_two;
pub mod question_three;

use crate::question_two::summary;
use crate::question_three::calculate_area;
use crate::question_three::Circle;
use crate::question_three::Square;
use crate::question_three::Triangle;

enum Light {
    Red,
    Green,
    Yellow,
}
trait Duration {
    fn duration(&self) -> u32;
}
impl Duration for Light {
    fn duration(&self) -> u32 {
        match self {
           Light::Red => 10,
           Light::Green => 12,
           Light::Yellow => 3
        }
    }
}
fn main() {
    let red = Light::Red;
    let green = Light::Green;
    let yellow = Light::Yellow;
    println!("Red light duration: {}", red.duration());
    println!("Yellow light duration: {}", yellow.duration());
    println!("Green light duration: {}", green.duration());

    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = summary(&vec);

    match sum {
        Some(v) => println!("Test2: The sum Option enum is {:?}", Some(v)),
        None => {
            println!("Test2: The result is overflow");
            ()
        }
    };

        let triangle = Triangle {
            base: 3.0,
            height: 4.0,
        };
    
        let square = Square { side: 5.0 };
    
        let circle = Circle { radius: 10.0 };
    
        println!("The area of triangle is {}", calculate_area(triangle));
        println!("The area of square is {}", calculate_area(square));
        println!("The area of circle is {}", calculate_area(circle));
}