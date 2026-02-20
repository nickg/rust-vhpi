fn value_change(data: &vhpi::CbData) {
    match data.obj.get_value(vhpi::Format::ObjType) {
        Ok(value) => {
            let full_name = data.obj.get_str(vhpi::StrProperty::FullName).unwrap();
            vhpi::printf!("value change {} => {}", full_name, value);
        }
        Err(err) => {
            let full_name = data.obj.get_str(vhpi::StrProperty::FullName).unwrap();
            let kind = data.obj.get_kind();
            let type_handle = data.obj.handle(vhpi::OneToOne::Type);
            let type_kind = type_handle.get_kind();
            match type_kind {
                vhpi::ClassKind::ArrayTypeDecl => {
                    let elem_type_handle = type_handle.handle(vhpi::OneToOne::ElemType);
                    for i in type_handle.index_range() {
                        let indexed_handle = data.obj.handle_by_index(i);
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
                vhpi::ClassKind::RecordTypeDecl => {
                    for field in data.obj.iterator(vhpi::OneToMany::SelectedNames) {
                        let field_type_handle = field.handle(vhpi::OneToOne::Type);

                        vhpi::printf!(
                            "signal {} is a part of the record {} of type {} with value {}",
                            field.get_name(),
                            data.obj.get_name(),
                            field_type_handle.get_name(),
                            field
                                .get_value(vhpi::Format::ObjType)
                                .unwrap_or(vhpi::Value::Unknown)
                        );
                    }
                }
                _ => {
                    vhpi::printf!(
                        "value change on {}, but failed to get value: {} (kind: {:?}, type: {}, type kind: {:?})",
                        full_name,
                        err,
                        kind,
                        type_handle.get_name(),
                        type_kind
                    );
                }
            }
        }
    }
}

fn walk_region(region: &vhpi::Handle) {
    for port in region.iterator(vhpi::OneToMany::PortDecls) {
        println!("port {}", port.get_name());
        port.register_cb(vhpi::CbReason::ValueChange, value_change);
    }

    for sig in region.iterator(vhpi::OneToMany::SigDecls) {
        let type_handle = sig.handle(vhpi::OneToOne::Type);
        let kind = type_handle.get_kind();
        match kind {
            vhpi::ClassKind::ArrayTypeDecl => {
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
                        }
                        println!(
                            "signal {} is an array of type {}",
                            sig.get_name(),
                            type_handle.get_name()
                        );
                        println!("but failed to get value: {err}");
                    }
                }
                sig.register_cb(vhpi::CbReason::ValueChange, value_change);
            }
            vhpi::ClassKind::RecordTypeDecl => {
                println!(
                    "signal {} is a record of type {}",
                    sig.get_name(),
                    type_handle.get_name()
                );
                for field in type_handle.iterator(vhpi::OneToMany::RecordElems) {
                    let field_type_handle = field.handle(vhpi::OneToOne::Type);

                    println!(
                        "field {} of record {} (type: {})",
                        field.get_name(),
                        sig.get_name(),
                        field_type_handle.get_name()
                    );
                }
                sig.register_cb(vhpi::CbReason::ValueChange, value_change);
            }
            _ => {
                println!(
                    "signal {}, type {} (kind: {:?})",
                    sig.get_name(),
                    type_handle.get_name(),
                    kind
                );
                sig.register_cb(vhpi::CbReason::ValueChange, value_change);
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
    vhpi::printf!("simulator name: {}", vhpi::simulator_name());
    vhpi::printf!(
        "simulator capabilities: {:?}",
        vhpi::simulator_capabilities()
    );

    vhpi::register_cb(vhpi::CbReason::StartOfSimulation, start_of_sim);
    vhpi::register_cb(vhpi::CbReason::EndOfSimulation, end_of_sim);
    vhpi::register_cb(vhpi::CbReason::NextTimeStep, next_time_step);
}
