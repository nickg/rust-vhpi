use vhpi::startup_routines;

startup_routines! {
    test_string_indexing_startup,
}

#[no_mangle]
pub extern "C" fn test_string_indexing_startup() {
    let _ = vhpi::register_cb(vhpi::CbReason::StartOfSimulation, start_of_sim);
}

fn str_change(data: &vhpi::CbData) {
    let handle = &data.obj;
    println!(
        "signal {:?} changed to {:?}",
        handle.get_name().unwrap(),
        handle.get_value(vhpi::Format::ObjType)
    );
    let type_handle = handle.handle(vhpi::OneToOne::Type);
    for (index, i) in type_handle.index_range().enumerate() {
        let index_handle = handle.handle_by_index(vhpi::OneToMany::IndexedNames, index as i32);
        println!(
            "  index {i}: {:?}",
            index_handle.map(|h| h.get_value(vhpi::Format::ObjType))
        );
    }
}

fn start_of_sim(_data: &vhpi::CbData) {
    vhpi::printf("start of simulation");
    let root = vhpi::handle(vhpi::OneToOne::RootInst);
    println!("root name is {:?}", root.get_name());
    println!("root kind is {:?}", root.get_kind());
    let str_handle = root.handle_by_name("v_str");
    if let Some(str_handle) = str_handle {
        println!(
            "signal {:?} has kind {:?}",
            str_handle.get_name(),
            str_handle.get_kind()
        );
        let _ = str_handle.register_cb(vhpi::CbReason::ValueChange, str_change);
    } else {
        println!("signal v_str not found");
    }

    let str_array_handle = root.handle_by_name("v_str_array");
    if let Some(str_array_handle) = str_array_handle {
        println!(
            "signal {:?} has kind {:?}",
            str_array_handle.get_name(),
            str_array_handle.get_kind()
        );
        let _ = str_array_handle.register_cb(vhpi::CbReason::ValueChange, str_change);
    } else {
        println!("signal v_str_array not found");
    }
}
