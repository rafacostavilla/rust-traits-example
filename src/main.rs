use rust_traits_example::{Pair, self};

fn main() {
    let pair1 = Pair::new(1, 2);
    // let pair2 = Pair::new(Some(2), Some(1));
    pair1.cmp_display();
    // pair2.cmp_display();
}
