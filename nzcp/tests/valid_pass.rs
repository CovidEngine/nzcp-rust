use chrono::NaiveDate;
use nzcp::{verify_pass_barcode_with_trusted_issuer, DecentralizedIdentifier, PublicCovidPass};

const EXAMPLE_ISSUER: DecentralizedIdentifier<'static> = DecentralizedIdentifier::Web("nzcp.covid19.health.nz");

/// https://nzcp.covid19.health.nz/#valid-worked-example
#[tokio::test]
async fn valid_pass() {
    let barcode = "NZCP:/1/2KCEVIQEIVVWK6JNGEASNICZAEP2KALYDZSGSZB2O5SWEOTOPJRXALTDN53GSZBRHEXGQZLBNR2GQLTOPICRUYMBTIFAIGTUKBAAUYTWMOSGQQDDN5XHIZLYOSBHQJTIOR2HA4Z2F4XXO53XFZ3TGLTPOJTS6MRQGE4C6Y3SMVSGK3TUNFQWY4ZPOYYXQKTIOR2HA4Z2F4XW46TDOAXGG33WNFSDCOJONBSWC3DUNAXG46RPMNXW45DFPB2HGL3WGFTXMZLSONUW63TFGEXDALRQMR2HS4DFQJ2FMZLSNFTGSYLCNRSUG4TFMRSW45DJMFWG6UDVMJWGSY2DN53GSZCQMFZXG4LDOJSWIZLOORUWC3CTOVRGUZLDOSRWSZ3JOZSW4TTBNVSWISTBMNVWUZTBNVUWY6KOMFWWKZ2TOBQXE4TPO5RWI33CNIYTSNRQFUYDILJRGYDVAYFE6VGU4MCDGK7DHLLYWHVPUS2YIDJOA6Y524TD3AZRM263WTY2BE4DPKIF27WKF3UDNNVSVWRDYIYVJ65IRJJJ6Z25M2DO4YZLBHWFQGVQR5ZLIWEQJOZTS3IQ7JTNCFDX";

    let pass: PublicCovidPass = verify_pass_barcode_with_trusted_issuer(barcode, EXAMPLE_ISSUER)
        .await
        .unwrap();

    assert_eq!(
        pass,
        PublicCovidPass {
            given_name: String::from("Jack"),
            family_name: Some(String::from("Sparrow")),
            date_of_birth: NaiveDate::from_ymd(1960, 04, 16),
        }
    )
}
