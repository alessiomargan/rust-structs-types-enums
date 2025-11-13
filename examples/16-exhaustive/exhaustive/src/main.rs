#[allow(dead_code)]
enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
    Barolo,
    Sangiovese,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("This is a Cabertnet Franc wine."),
        WineGrapes::Tannat => println!("This is a Tannat wine."),
        WineGrapes::Merlot => println!("This is a Merlot wine."),
        _ => println!("This could be a Barolo or Sangiovese wine") /* This arm is unreachable because all variants are covered */
    }
}


enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

fn main() {

    taste_wine(WineGrapes::CabernetFranc);

    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 6.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            _ => 0.0, // This arm is now unreachable because all variants are covered
        })
        .sum();

    println!("Total area NO Triangle: {} sq. units", total_area);

    let total_area2: f64 = shapes
        .iter()
        .map(|shape| shape.area())
        .sum();


    println!("Total area 2: {} sq. units", total_area2);
}