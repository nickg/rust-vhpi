fn value_change(data: &vhpi::CbData) {
    let value = data.obj.get_value(vhpi::Format::ObjType).unwrap();
    let full_name = data.obj.get_str(vhpi::StrProperty::FullName).unwrap();
    vhpi::printf!("value change {} => {}", full_name, value);
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
    let handle = vhpi::handle(vhpi::OneToOne::RootInst);
    let (time, status) = handle.get_next_time();
    vhpi::printf!("next time step: {} (status: {status})", time.to_i64());
}

fn end_of_sim(_data: &vhpi::CbData) {
    let handle = vhpi::handle(vhpi::OneToOne::RootInst);
    let time = handle.get_time().to_i64();
    let cycles = handle.get_cycles();
    vhpi::printf!(
        "end of simulation at time {time} ({cycles} cycle{})",
        if cycles == 1 { "" } else { "s" }
    );
}

#[no_mangle]
pub extern "C" fn dumper_startup() {
    vhpi::printf("dumper plugin loaded");

    vhpi::register_cb(vhpi::CbReason::StartOfSimulation, start_of_sim);
    vhpi::register_cb(vhpi::CbReason::EndOfSimulation, end_of_sim);
    vhpi::register_cb(vhpi::CbReason::NextTimeStep, next_time_step);
}
