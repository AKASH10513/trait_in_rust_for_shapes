   
    use  stringinrust::{Rectangle,Square};
    use  stringinrust::Geometry;
    fn main()
    {
    
    let rectangle =  Rectangle{
        length:43,
        breadth:54,
    };
    let square =  Square{
        side : 5,
    };
    println!("Your rectangle area is :{} ", rectangle.area());
    println!("your rectangle permeter is  :{}", rectangle.perimeter());
    println!("your rectangle side is : {} ", square.area());
    }