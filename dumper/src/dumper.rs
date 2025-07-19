fn value_change(data: &vhpi::CbData) {
    let value = data.obj.get_value(vhpi::Format::ObjType).unwrap();
    vhpi::printf!("value change {} => {}", data.obj.get_name(), value);
}

fn walk_region(region: &vhpi::Handle) {
    for port in region.iterator(vhpi::OneToMany::PortDecls) {
        println!("port {}", port.get_name());
        port.register_cb(vhpi::CbReason::ValueChange, value_change);
    }

    for sig in region.iterator(vhpi::OneToMany::SigDecls) {
        println!("signal {}", sig.get_name());
        sig.register_cb(vhpi::CbReason::ValueChange, value_change);
    }

    for sub in region.iterator(vhpi::OneToMany::InternalRegions) {
        println!("internal region {}", sub.get_name());
        walk_region(&sub);
    }
}

fn start_of_sim(_data: &vhpi::CbData) {
    vhpi::printf("start of simulation");

    let root = vhpi::handle(vhpi::OneToOne::RootInst);

    vhpi::printf!("root name is {}", root.get_name());
    vhpi::printf!("root kind is {:?}", root.get_kind());

    walk_region(&root);
}

fn next_time_step(_data: &vhpi::CbData) {
    vhpi::printf("next time step");
}

fn end_of_sim(_data: &vhpi::CbData) {
    vhpi::printf("end of simulation");
}

#[no_mangle]
pub extern "C" fn dumper_startup() {
    vhpi::printf("dumper plugin loaded");

    vhpi::register_cb(vhpi::CbReason::StartOfSimulation, start_of_sim);
    vhpi::register_cb(vhpi::CbReason::EndOfSimulation, end_of_sim);
    vhpi::register_cb(vhpi::CbReason::NextTimeStep, next_time_step);
}
