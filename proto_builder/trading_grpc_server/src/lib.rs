pub mod exchange {
    tonic::include_proto!("exchange");
}

pub mod trade {
    tonic::include_proto!("trade");
}

pub mod user {
    tonic::include_proto!("user");
}
pub mod market {
    tonic::include_proto!("market");
}
pub mod controller_notify {
    tonic::include_proto!("controller_notify");
}
