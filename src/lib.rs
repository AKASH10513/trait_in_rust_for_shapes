
pub trait Geometry
{
    fn area(&self)->usize;
    fn perimeter(&self)->usize;

}


pub struct Rectangle{
    pub length:usize,
    pub breadth:usize,
}
pub struct Square{
    pub side: usize,
}
impl Geometry for Rectangle{
    fn area(&self)->usize{
        self.length*self.breadth
    }

    fn perimeter(&self)->usize{
        2*(self.length+self.breadth)
    }
}
impl Geometry for Square{
    fn area(&self)->usize{
        self.side * self.side
    }
    fn perimeter(&self)->usize{
        4*self.side
    }
}