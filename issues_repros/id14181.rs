//ISSUE #14181 - C-bug, I-false-positive

fn example(kind: &cargo_metadata::TargetKind) {
    match kind {
        cargo_metadata::TargetKind::Lib { .. } => todo!(),
        _ => todo!(),
    }
}
