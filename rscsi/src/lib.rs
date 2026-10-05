pub use rscsi_core::*;
pub use rscsi_multipath as multipath;
pub use rscsi_sbc as sbc;
pub use rscsi_spc as spc;
pub use rscsi_transport as transport;
pub use rscsi_transport_iscsi as iscsi;
pub use rscsi_util as util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_me() {
        assert_eq!(whoami(), "rscsi-core");
    }
}
