

fn main() {
    //红绿灯
    println!("红绿灯");
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("红灯时间：{}",red.time());
    println!("绿灯时间：{}",green.time());
    println!("黄灯时间：{}",yellow.time());
    //求数组和
    println!("求数组和");
    let sum = [5;10];
    println!("{:?} sum :{:?}",sum ,sum_u32(&sum));
    let sum = [5,4294967295];
    println!("{:?} sum :{:?}",sum ,sum_u32(&sum));
    // 图形面积
    println!("图形面积");
    let triangle = Triangle{
        h:4.0,
        a:5.0
    };
    println!("三角形：");
    print_shape(triangle);
    let round = Round{
        r:43.0
    };
    println!("圆形：");
    print_shape(round);
    let square = Square{
        a:43.0
    };
    println!("正方形：");
    print_shape(square);
}
//红绿灯
enum TrafficLight{
    Red,
    Green,
    Yellow,
}

impl TrafficLight{
    fn time(&self) -> u8{
        match self {
            TrafficLight::Red =>5,
            TrafficLight::Green =>6,
            TrafficLight::Yellow =>7,
        }
    }
}

//求数组和
fn sum_u32(a:&[u32]) -> Option<u32>{
    let mut sum:Option<u32> = Some(0);
    for item in a{
        sum =item.checked_add(match sum{
            Some(x) => x,
            None => return None,}
            );
    }
    sum
}

//求图形面积

trait Shape{
    fn area(&self) -> f64;
}

struct Triangle{
    h:f64,
    a:f64,
}

impl Shape for Triangle{
    fn area(&self)->f64{
        self.h*self.a/2.0
    }
}
struct Round{
    r:f64,
}
impl Shape for Round{
    fn area(&self)->f64{
        std::f64::consts::PI* (self.r *self.r)
    }
}
struct Square{
    a:f64,
}
impl Shape for Square{
    fn area(&self)->f64{
        self.a*self.a
    }
}

fn print_shape <T:Shape>(t:T){
    println!("图形面积为：{}",t.area());
}
