use alloy_rpc_types::Log;

pub trait SolEventDecode: Sized {
    fn decode_log(log: &Log) -> alloy_sol_types::Result<Self>;
}

#[macro_export]
macro_rules! impl_sol_event_decode {
    ($($event_type:ty),+) => {
        use alloy_extension::SolEventDecode;
        use alloy_sol_types::SolEvent;

        $(
            const _: fn() = || {
                fn assert_impl<T: SolEvent>() {}
                assert_impl::<$event_type>();
            };
            
            impl SolEventDecode for $event_type {
                fn decode_log(log: &alloy_rpc_types::Log) -> alloy_sol_types::Result<Self> {
                    Self::decode_log_data(&log.inner, true)
                }
            }
        )+
    };
}
