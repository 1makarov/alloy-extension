use alloy_rpc_types::Log;

pub trait SolEventDecode: Sized {
    fn decode_log(log: &Log) -> alloy_sol_types::Result<Self>;
}

#[macro_export]
macro_rules! impl_sol_event_decode {
    ($event_type:ty) => {
        impl SolEventDecode for $event_type {
            fn decode_log(log: &alloy_rpc_types::Log) -> alloy_sol_types::Result<Self> {
                use alloy_sol_types::SolEvent;
                Self::decode_log_data(&log.inner, true)
            }
        }
    };
    ($($event_type:ty),*) => {
        $(
            impl SolEventDecode for $event_type {
                fn decode_log(log: &alloy_rpc_types::Log) -> alloy_sol_types::Result<Self> {
                    use alloy_sol_types::SolEvent;
                    Self::decode_log_data(&log.inner, true)
                }
            }
        )*
    };
}
