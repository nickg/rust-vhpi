fn start_of_sim() {
    vhpi::printf("start of simulation");

    let null = vhpi::Handle::null();
    let root = null.handle(vhpi::OneToOne::RootInst);

    println!("root name is {}", root.get_name());
    println!("root kind is {:?}", root.get_kind());
}

fn next_time_step() {
    vhpi::printf("next time step");
}

fn end_of_sim() {
    vhpi::printf("end of simulation");
}

#[no_mangle]
pub extern "C" fn dumper_startup() {
    vhpi::printf("dumper plugin loaded");

    vhpi::Callback::new(vhpi::CbReason::StartOfSimulation, start_of_sim)
        .register();
    vhpi::Callback::new(vhpi::CbReason::EndOfSimulation, end_of_sim)
        .register();
    vhpi::Callback::new(vhpi::CbReason::RepNextTimeStep, next_time_step)
        .register();
}
