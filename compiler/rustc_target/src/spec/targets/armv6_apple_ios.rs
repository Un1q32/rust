use crate::spec::base::apple::{Arch, TargetEnv, base};
use crate::spec::{Cc, LinkerFlavor, Lld, Os, Target, TargetMetadata, TargetOptions};

pub(crate) fn target() -> Target {
    let (opts, llvm_target, arch) = base(Os::IOs, Arch::Armv6, TargetEnv::Normal);
    Target {
        llvm_target,
        metadata: TargetMetadata {
            description: Some("ARMv6K Apple iOS".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:o-p:32:32-Fi8-f64:32:64-v64:32:64-v128:32:128-a:0:32-n32-S32".into(),
        arch,
        options: TargetOptions {
            features: "+v6,+vfp2".into(),
            max_atomic_width: Some(64),
            has_thread_local: false,
            late_link_args: TargetOptions::link_args(LinkerFlavor::Darwin(Cc::Yes, Lld::No), &["-lgcc_s.1", "-lsyscompat"]),
            ..opts
        },
    }
}
