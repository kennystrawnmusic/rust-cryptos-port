// Generic x86-64 target for bare-metal code - Floating point disabled
//
// Can be used in conjunction with the `target-feature` and
// `target-cpu` compiler flags to opt-in more hardware-specific
// features.

use super::{CodeModel, PanicStrategy};
use super::Target;

pub fn target() -> Target {
    let mut opts = super::cryptos_base::opts();
    opts.cpu = "x86-64".into();
    opts.disable_redzone = true;
    opts.panic_strategy = PanicStrategy::Abort;
    opts.code_model = Some(CodeModel::Kernel);
    opts.features = "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float"
        .into();

    Target {
        llvm_target: "x86_64-unknown-none-elf".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .into(),
        arch: "x86_64".into(),
        options: opts,
    }
}
