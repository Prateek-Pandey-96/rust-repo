trait Vehicle{
    fn drive(&self);
}

struct Car{}
impl Vehicle for Car{
    fn drive(&self) {
        println!("Car is being driven!");
    }
}

struct Bike{}
impl Vehicle for Bike{
    fn drive(&self) {
        println!("Bike is being ridden!");
    }
}

pub fn learn(){
    let vehicle1: Box<dyn Vehicle>;
    vehicle1 = Box::new(Bike{});
    vehicle1.drive();

    let vehicle2: Box<dyn Vehicle>;
    vehicle2 = Box::new(Car{});
    vehicle2.drive();
}