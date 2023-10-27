use crate::spec::{StackProbeType, LinkerFlavorCli, LldFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "cryptos".into(),
        executables: true,
        linker: Some("ld.lld".into()),
        linker_flavor_json: LinkerFlavorCli::Lld(LldFlavor::Ld),
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}