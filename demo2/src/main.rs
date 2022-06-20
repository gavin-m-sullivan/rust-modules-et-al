mod auxiliary_building;
mod tower_building;

use auxiliary_building::auxiliary as aux;
use tower_building::tower as tower;

use auxiliary_building::plug as plug;

fn main() {
    println!("Hello, world!");

    aux::generate_energy();
    tower::start_consumption();

    plug::device::do_it();
    plug::other_device::do_it_with_other();
}
