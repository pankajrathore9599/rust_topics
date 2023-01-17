
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn printdesc(&self) {
        println!("Rectangle {} multiple {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}


fn main() {
    let my_rect = Rectangle{width:45, height:45};
    my_rect.printdesc();

    println!("Rectangle is square: {}", my_rect.is_square());
}
