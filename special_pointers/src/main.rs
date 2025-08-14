mod box_learning;
mod rc_learning;
mod refcell_learning;

fn main() {
    println!("Let's learn about box, rc and refcell");
    box_learning::learn();
    rc_learning::initialize();
    refcell_learning::learn();
}
