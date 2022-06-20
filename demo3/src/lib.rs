mod auxiliary_building;
mod tower_building;

use auxiliary_building::auxiliary as aux;
use tower_building::tower as tower;

fn test_it() {
    aux::generate_energy();
    tower::start_consumption();

    auxiliary_building::plug::device::do_it();
    auxiliary_building::plug::other_device::do_it_with_other();
}