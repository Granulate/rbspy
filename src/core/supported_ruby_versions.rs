use anyhow::{format_err, Result};
use semver::Version;

pub struct RubyVersion {
    pub semver_version: Version,
    pub get_execution_context_fn: crate::core::types::GetExecutionContextFn,
    pub is_maybe_thread_fn: crate::core::types::IsMaybeThreadFn,
    pub get_stack_trace_fn: crate::core::types::StackTraceFn,
}

pub fn get(v: &str) -> Result<RubyVersion> {
    match v {
        "1.9.1" => Ok(RubyVersion {
            semver_version: Version::new(1, 9, 1),
            get_execution_context_fn: super::ruby_version::ruby_1_9_1_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_1_9_1_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_1_9_1_0::is_maybe_thread,
        }),
        "1.9.2" => Ok(RubyVersion {
            semver_version: Version::new(1, 9, 2),
            get_execution_context_fn: super::ruby_version::ruby_1_9_2_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_1_9_2_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_1_9_2_0::is_maybe_thread,
        }),
        "1.9.3" => Ok(RubyVersion {
            semver_version: Version::new(1, 9, 3),
            get_execution_context_fn: super::ruby_version::ruby_1_9_3_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_1_9_3_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_1_9_3_0::is_maybe_thread,
        }),
        "2.0.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 0, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_0_0_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_0_0_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_0_0_0::is_maybe_thread,
        }),
        "2.1.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_1_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_0::is_maybe_thread,
        }),
        "2.1.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_1_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_1::is_maybe_thread,
        }),
        "2.1.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_1_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_2::is_maybe_thread,
        }),
        "2.1.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_1_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_3::is_maybe_thread,
        }),
        "2.1.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_1_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_4::is_maybe_thread,
        }),
        "2.1.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_1_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_5::is_maybe_thread,
        }),
        "2.1.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_1_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_6::is_maybe_thread,
        }),
        "2.1.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_1_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_7::is_maybe_thread,
        }),
        "2.1.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_1_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_8::is_maybe_thread,
        }),
        "2.1.9" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 9),
            get_execution_context_fn: super::ruby_version::ruby_2_1_9::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_9::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_9::is_maybe_thread,
        }),
        "2.1.10" => Ok(RubyVersion {
            semver_version: Version::new(2, 1, 10),
            get_execution_context_fn: super::ruby_version::ruby_2_1_10::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_1_10::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_1_10::is_maybe_thread,
        }),
        "2.2.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_2_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_0::is_maybe_thread,
        }),
        "2.2.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_2_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_1::is_maybe_thread,
        }),
        "2.2.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_2_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_2::is_maybe_thread,
        }),
        "2.2.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_2_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_3::is_maybe_thread,
        }),
        "2.2.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_2_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_4::is_maybe_thread,
        }),
        "2.2.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_2_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_5::is_maybe_thread,
        }),
        "2.2.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_2_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_6::is_maybe_thread,
        }),
        "2.2.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_2_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_7::is_maybe_thread,
        }),
        "2.2.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_2_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_8::is_maybe_thread,
        }),
        "2.2.9" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 9),
            get_execution_context_fn: super::ruby_version::ruby_2_2_9::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_9::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_9::is_maybe_thread,
        }),
        "2.2.10" => Ok(RubyVersion {
            semver_version: Version::new(2, 2, 10),
            get_execution_context_fn: super::ruby_version::ruby_2_2_10::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_2_10::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_2_10::is_maybe_thread,
        }),
        "2.3.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_3_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_0::is_maybe_thread,
        }),
        "2.3.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_3_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_1::is_maybe_thread,
        }),
        "2.3.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_3_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_2::is_maybe_thread,
        }),
        "2.3.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_3_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_3::is_maybe_thread,
        }),
        "2.3.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_3_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_4::is_maybe_thread,
        }),
        "2.3.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_3_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_5::is_maybe_thread,
        }),
        "2.3.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_3_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_6::is_maybe_thread,
        }),
        "2.3.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_3_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_7::is_maybe_thread,
        }),
        "2.3.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 3, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_3_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_3_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_3_8::is_maybe_thread,
        }),
        "2.4.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_4_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_0::is_maybe_thread,
        }),
        "2.4.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_4_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_1::is_maybe_thread,
        }),
        "2.4.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_4_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_2::is_maybe_thread,
        }),
        "2.4.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_4_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_3::is_maybe_thread,
        }),
        "2.4.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_4_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_4::is_maybe_thread,
        }),
        "2.4.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_4_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_5::is_maybe_thread,
        }),
        "2.4.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_4_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_6::is_maybe_thread,
        }),
        "2.4.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_4_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_7::is_maybe_thread,
        }),
        "2.4.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_4_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_8::is_maybe_thread,
        }),
        "2.4.9" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 9),
            get_execution_context_fn: super::ruby_version::ruby_2_4_9::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_9::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_9::is_maybe_thread,
        }),
        "2.4.10" => Ok(RubyVersion {
            semver_version: Version::new(2, 4, 10),
            get_execution_context_fn: super::ruby_version::ruby_2_4_10::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_4_10::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_4_10::is_maybe_thread,
        }),
        "2.5.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_5_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_0::is_maybe_thread,
        }),
        "2.5.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_5_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_1::is_maybe_thread,
        }),
        "2.5.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_5_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_2::is_maybe_thread,
        }),
        "2.5.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_5_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_3::is_maybe_thread,
        }),
        "2.5.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_5_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_4::is_maybe_thread,
        }),
        "2.5.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_5_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_5::is_maybe_thread,
        }),
        "2.5.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_5_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_6::is_maybe_thread,
        }),
        "2.5.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_5_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_7::is_maybe_thread,
        }),
        "2.5.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_5_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_8::is_maybe_thread,
        }),
        "2.5.9" => Ok(RubyVersion {
            semver_version: Version::new(2, 5, 9),
            get_execution_context_fn: super::ruby_version::ruby_2_5_9::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_5_9::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_5_9::is_maybe_thread,
        }),
        "2.6.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_6_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_0::is_maybe_thread,
        }),
        "2.6.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_6_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_1::is_maybe_thread,
        }),
        "2.6.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_6_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_2::is_maybe_thread,
        }),
        "2.6.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_6_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_3::is_maybe_thread,
        }),
        "2.6.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_6_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_4::is_maybe_thread,
        }),
        "2.6.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_6_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_5::is_maybe_thread,
        }),
        "2.6.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_6_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_6::is_maybe_thread,
        }),
        "2.6.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_6_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_7::is_maybe_thread,
        }),
        "2.6.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_6_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_8::is_maybe_thread,
        }),
        "2.6.9" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 9),
            get_execution_context_fn: super::ruby_version::ruby_2_6_9::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_9::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_9::is_maybe_thread,
        }),
        "2.6.10" => Ok(RubyVersion {
            semver_version: Version::new(2, 6, 10),
            get_execution_context_fn: super::ruby_version::ruby_2_6_10::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_6_10::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_6_10::is_maybe_thread,
        }),
        "2.7.0" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 0),
            get_execution_context_fn: super::ruby_version::ruby_2_7_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_0::is_maybe_thread,
        }),
        "2.7.1" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 1),
            get_execution_context_fn: super::ruby_version::ruby_2_7_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_1::is_maybe_thread,
        }),
        "2.7.2" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 2),
            get_execution_context_fn: super::ruby_version::ruby_2_7_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_2::is_maybe_thread,
        }),
        "2.7.3" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 3),
            get_execution_context_fn: super::ruby_version::ruby_2_7_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_3::is_maybe_thread,
        }),
        "2.7.4" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 4),
            get_execution_context_fn: super::ruby_version::ruby_2_7_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_4::is_maybe_thread,
        }),
        "2.7.5" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 5),
            get_execution_context_fn: super::ruby_version::ruby_2_7_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_5::is_maybe_thread,
        }),
        "2.7.6" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 6),
            get_execution_context_fn: super::ruby_version::ruby_2_7_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_6::is_maybe_thread,
        }),
        "2.7.7" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 7),
            get_execution_context_fn: super::ruby_version::ruby_2_7_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_7::is_maybe_thread,
        }),
        "2.7.8" => Ok(RubyVersion {
            semver_version: Version::new(2, 7, 8),
            get_execution_context_fn: super::ruby_version::ruby_2_7_8::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_2_7_8::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_2_7_8::is_maybe_thread,
        }),
        "3.0.0" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 0),
            get_execution_context_fn: super::ruby_version::ruby_3_0_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_0::is_maybe_thread,
        }),
        "3.0.1" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 1),
            get_execution_context_fn: super::ruby_version::ruby_3_0_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_1::is_maybe_thread,
        }),
        "3.0.2" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 2),
            get_execution_context_fn: super::ruby_version::ruby_3_0_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_2::is_maybe_thread,
        }),
        "3.0.3" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 3),
            get_execution_context_fn: super::ruby_version::ruby_3_0_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_3::is_maybe_thread,
        }),
        "3.0.4" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 4),
            get_execution_context_fn: super::ruby_version::ruby_3_0_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_4::is_maybe_thread,
        }),
        "3.0.5" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 5),
            get_execution_context_fn: super::ruby_version::ruby_3_0_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_5::is_maybe_thread,
        }),
        "3.0.6" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 6),
            get_execution_context_fn: super::ruby_version::ruby_3_0_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_6::is_maybe_thread,
        }),
        "3.0.7" => Ok(RubyVersion {
            semver_version: Version::new(3, 0, 7),
            get_execution_context_fn: super::ruby_version::ruby_3_0_7::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_0_7::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_0_7::is_maybe_thread,
        }),
        "3.1.0" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 0),
            get_execution_context_fn: super::ruby_version::ruby_3_1_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_0::is_maybe_thread,
        }),
        "3.1.1" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 1),
            get_execution_context_fn: super::ruby_version::ruby_3_1_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_1::is_maybe_thread,
        }),
        "3.1.2" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 2),
            get_execution_context_fn: super::ruby_version::ruby_3_1_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_2::is_maybe_thread,
        }),
        "3.1.3" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 3),
            get_execution_context_fn: super::ruby_version::ruby_3_1_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_3::is_maybe_thread,
        }),
        "3.1.4" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 4),
            get_execution_context_fn: super::ruby_version::ruby_3_1_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_4::is_maybe_thread,
        }),
        "3.1.5" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 5),
            get_execution_context_fn: super::ruby_version::ruby_3_1_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_5::is_maybe_thread,
        }),
        "3.1.6" => Ok(RubyVersion {
            semver_version: Version::new(3, 1, 6),
            get_execution_context_fn: super::ruby_version::ruby_3_1_6::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_1_6::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_1_6::is_maybe_thread,
        }),
        "3.2.0" => Ok(RubyVersion {
            semver_version: Version::new(3, 2, 0),
            get_execution_context_fn: super::ruby_version::ruby_3_2_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_2_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_2_0::is_maybe_thread,
        }),
        "3.2.1" => Ok(RubyVersion {
            semver_version: Version::new(3, 2, 1),
            get_execution_context_fn: super::ruby_version::ruby_3_2_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_2_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_2_1::is_maybe_thread,
        }),
        "3.2.2" => Ok(RubyVersion {
            semver_version: Version::new(3, 2, 2),
            get_execution_context_fn: super::ruby_version::ruby_3_2_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_2_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_2_2::is_maybe_thread,
        }),
        "3.2.3" => Ok(RubyVersion {
            semver_version: Version::new(3, 2, 3),
            get_execution_context_fn: super::ruby_version::ruby_3_2_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_2_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_2_3::is_maybe_thread,
        }),
        "3.2.4" => Ok(RubyVersion {
            semver_version: Version::new(3, 2, 4),
            get_execution_context_fn: super::ruby_version::ruby_3_2_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_2_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_2_4::is_maybe_thread,
        }),
        "3.2.5" => Ok(RubyVersion {
            semver_version: Version::new(3, 2, 5),
            get_execution_context_fn: super::ruby_version::ruby_3_2_5::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_2_5::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_2_5::is_maybe_thread,
        }),
        "3.3.0" => Ok(RubyVersion {
            semver_version: Version::new(3, 3, 0),
            get_execution_context_fn: super::ruby_version::ruby_3_3_0::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_3_0::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_3_0::is_maybe_thread,
        }),
        "3.3.1" => Ok(RubyVersion {
            semver_version: Version::new(3, 3, 1),
            get_execution_context_fn: super::ruby_version::ruby_3_3_1::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_3_1::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_3_1::is_maybe_thread,
        }),
        "3.3.2" => Ok(RubyVersion {
            semver_version: Version::new(3, 3, 2),
            get_execution_context_fn: super::ruby_version::ruby_3_3_2::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_3_2::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_3_2::is_maybe_thread,
        }),
        "3.3.3" => Ok(RubyVersion {
            semver_version: Version::new(3, 3, 3),
            get_execution_context_fn: super::ruby_version::ruby_3_3_3::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_3_3::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_3_3::is_maybe_thread,
        }),
        "3.3.4" => Ok(RubyVersion {
            semver_version: Version::new(3, 3, 4),
            get_execution_context_fn: super::ruby_version::ruby_3_3_4::get_execution_context,
            get_stack_trace_fn: super::ruby_version::ruby_3_3_4::get_stack_trace,
            is_maybe_thread_fn: super::ruby_version::ruby_3_3_4::is_maybe_thread,
        }),
        _ => Err(format_err!("rbspy doesn't support Ruby {} yet. If this is a new patch-level version of Ruby, you can try using `--force-version` with the previous version number.", v)),
    }
}
