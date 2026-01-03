fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &[
                "../../proto/trade.proto",
                "../../proto/user.proto",
                "../../proto/controller_notify.proto",
                "../../proto/market.proto",
                "../../proto/price_aggregator.proto",
                "../../proto/risk_management.proto",
            ],
            &["../../proto"],
        )
        .unwrap();
}
