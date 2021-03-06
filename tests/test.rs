#[path = "../src/core/lib.rs"]
mod opensi;

use opensi::Package;

const PATH: &str = "tests/data/slamjam2.siq";

#[test]
fn open_pack() {
    let package = Package::open(PATH);
    assert_eq!(package.is_ok(), true);
}

#[test]
fn read_package_name() {
    let package = Package::open(PATH).expect("pack is not found"); 
    assert_eq!(package.name.is_some(), true);
    assert_eq!(package.name.unwrap(), "SLAM JAM 2".to_owned());
}
