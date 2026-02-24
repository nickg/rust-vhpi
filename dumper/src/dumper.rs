fn value_change(data: &vhpi::CbData) {
    let val = data.obj.get_value(vhpi::Format::ObjType);
    match val {
        Ok(value) => {
            let full_name = data.obj.get_str(vhpi::StrProperty::FullName).unwrap();
            vhpi::printf!("value change {} => {}", full_name, value);
        }
        Err(err) => {
            let full_name = data.obj.get_str(vhpi::StrProperty::FullName).unwrap();
            let kind = data.obj.get_kind();
            let type_handle = data.obj.handle(vhpi::OneToOne::Type);
            match type_handle.get_kind() {
                Some(vhpi::ClassKind::ArrayTypeDecl) => {
                    let elem_type_handle = type_handle.handle(vhpi::OneToOne::ElemType);
                    for i in type_handle.index_range() {
                        if let Some(indexed_handle) =
                            data.obj.handle_by_index(vhpi::OneToMany::IndexedNames, i)
                        {
                            vhpi::printf!(
                                "element {} of array {} is of type {} with value {}",
                                i,
                                data.obj.get_name(),
                                elem_type_handle.get_name(),
                                indexed_handle
                                    .get_value(vhpi::Format::ObjType)
                                    .unwrap_or(vhpi::Value::Unknown)
                            );
                        }
                    }
                }
                Some(vhpi::ClassKind::RecordTypeDecl) => {
                    for field_handle in data.obj.iterator(vhpi::OneToMany::SelectedNames) {
                        let field_type_handle = field_handle.handle(vhpi::OneToOne::Type);

                        vhpi::printf!(
                            "signal {} is a field of the record {} of type {} with value {}",
                            field_handle.get_name(),
                            data.obj.get_name(),
                            field_type_handle.get_name(),
                            field_handle
                                .get_value(vhpi::Format::ObjType)
                                .unwrap_or(vhpi::Value::Unknown)
                        );
                    }
                }
                Some(type_kind) => {
                    vhpi::printf!(
                        "value change on {}, but failed to get value: {} (kind: {:?}, type: {}, type kind: {:?})",
                        full_name,
                        err,
                        kind,
                        type_handle.get_name(),
                        type_kind
                    );
                }
                None => {
                    vhpi::printf!("Invalid kind for {}: {}", full_name, err);
                }
            }
        }
    }
}

fn walk_region(region: &vhpi::Handle) {
    for port in region.iterator(vhpi::OneToMany::PortDecls) {
        println!("port {}", port.get_name());
        let _ = port.register_cb(vhpi::CbReason::ValueChange, value_change);
    }

    for sig in region.iterator(vhpi::OneToMany::SigDecls) {
        let type_handle = sig.handle(vhpi::OneToOne::Type);
        match type_handle.get_kind() {
            Some(vhpi::ClassKind::ArrayTypeDecl) => {
                match sig.get_value(vhpi::Format::ObjType) {
                    Ok(_) => {
                        println!(
                            "signal {} is an array of type {}",
                            sig.get_name(),
                            type_handle.get_name()
                        );
                    }
                    Err(err) => {
                        let elem_type_handle = type_handle.handle(vhpi::OneToOne::ElemType);
                        for i in type_handle.index_range() {
                            println!(
                                "element {} of array {} is of type {}",
                                i,
                                sig.get_name(),
                                elem_type_handle.get_name(),
                            );
                            if let Some(h) = sig.handle_by_index(vhpi::OneToMany::IndexedNames, i) {
                                if let Err(e) =
                                    h.register_cb(vhpi::CbReason::ValueChange, value_change)
                                {
                                    vhpi::printf!(
                                        "failed to register callback for element {}: {:?}",
                                        i,
                                        e
                                    );
                                }
                            }
                        }
                        println!(
                            "signal {} is an array of type {}",
                            sig.get_name(),
                            type_handle.get_name()
                        );
                        println!("but failed to get value: {err}");
                    }
                }
                let _ = sig.register_cb(vhpi::CbReason::ValueChange, value_change);
            }
            Some(vhpi::ClassKind::RecordTypeDecl) => {
                println!(
                    "signal {} is a record of type {}",
                    sig.get_name(),
                    type_handle.get_name()
                );
                for field in sig.iterator(vhpi::OneToMany::SelectedNames) {
                    let field_type_handle = field.handle(vhpi::OneToOne::Type);
                    println!(
                        "field {} of record {} (type: {})",
                        field.get_name(),
                        sig.get_name(),
                        field_type_handle.get_name()
                    );
                    if let Err(e) = field.register_cb(vhpi::CbReason::ValueChange, value_change) {
                        vhpi::printf!(
                            "failed to register callback for field {}: {:?}",
                            field.get_name(),
                            e
                        );
                    }
                }
                let _ = sig.register_cb(vhpi::CbReason::ValueChange, value_change);
            }
            Some(vhpi::ClassKind::EnumTypeDecl) => {
                println!(
                    "signal {} is an enum of type {} with values {:?}",
                    sig.get_name(),
                    type_handle.get_name(),
                    type_handle.enum_literals().unwrap_or_default()
                );
                let _ = sig.register_cb(vhpi::CbReason::ValueChange, value_change);
            }
            Some(kind) => {
                println!(
                    "signal {}, type {} (kind: {:?})",
                    sig.get_name(),
                    type_handle.get_name(),
                    kind
                );
                let _ = sig.register_cb(vhpi::CbReason::ValueChange, value_change);
            }
            None => {
                println!("signal {} with unsupported kind", sig.get_name(),);
            }
        }
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
    let (time, status) = vhpi::get_next_time();
    vhpi::printf!("next time step: {} (status: {status})", time);
}

fn end_of_sim(_data: &vhpi::CbData) {
    let time = vhpi::get_time();
    let cycles = vhpi::get_cycles();
    let time_resolution = vhpi::simulator_time_resolution();
    let total_time = time * time_resolution;
    vhpi::printf!(
        "end of simulation at time {total_time} ({cycles} cycle{})",
        if cycles == 1 { "" } else { "s" }
    );
}

#[no_mangle]
pub extern "C" fn dumper_startup() {
    vhpi::printf("dumper plugin loaded");
    vhpi::printf!("simulator name: {}", vhpi::simulator_name());
    vhpi::printf!(
        "simulator time resolution: {}",
        vhpi::simulator_time_resolution()
    );
    vhpi::printf!(
        "simulator capabilities: {:?}",
        vhpi::simulator_capabilities()
    );

    let _ = vhpi::register_cb(vhpi::CbReason::StartOfSimulation, start_of_sim);
    let _ = vhpi::register_cb(vhpi::CbReason::EndOfSimulation, end_of_sim);
    let _ = vhpi::register_cb(vhpi::CbReason::NextTimeStep, next_time_step);
}
