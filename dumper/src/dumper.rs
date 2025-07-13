fn value_change(_data: &vhpi::CbData) {
    vhpi::printf("value change");
}

fn start_of_sim(_data: &vhpi::CbData) {
    vhpi::printf("start of simulation");

    let null = vhpi::Handle::null();
    let root = null.handle(vhpi::OneToOne::RootInst);

    println!("root name is {}", root.get_name());
    println!("root kind is {:?}", root.get_kind());

    for sig in root.iterator(vhpi::OneToMany::SigDecls) {
        println!("signal {}", sig.get_name());
        sig.register_cb(vhpi::CbReason::ValueChange, value_change);
    }
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
