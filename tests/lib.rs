extern crate spark;

#[test]
fn basic_input() {
    assert_eq!(
        spark::graph(&[1.0, 5.0, 22.0, 13.0, 53.0]),
        "▁▁▃▂█"
    );

    assert_eq!(
        spark::graph(&[0.0, 30.0, 55.0, 80.0, 33.0, 150.0]),
        "▁▂▃▄▂█"
    );

    assert_eq!(
        spark::graph(&[9.0, 13.0, 5.0, 17.0, 1.0]),
        "▄▆▂█▁"
    );
}
