use crate::spec::{LinkerFlavor, LldFlavor, TargetOptions};
 
pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "cryptos".into(),
        executables: true,
        linker: Some("ld.lld".into()),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}