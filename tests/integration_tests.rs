use multid::V2;
use multid::iterators::V2Indices;

#[derive(Debug, Clone, PartialEq)]
struct OwnedU8(Box<u8>);

impl OwnedU8 {
    fn new(v: u8) -> Self {
        Self(Box::new(v))
    }
}

#[test]
fn test_owned() {
    let mut data: Vec<OwnedU8> = Vec::with_capacity(9);
    for i in 0..9 {
        data.push(OwnedU8::new(i));
    }
    let mut v2: V2<OwnedU8, 3, 3> = V2::new(data).unwrap();
    for ix in V2Indices::<3, 3>::new() {
        if let Some(wix) = ix.west()
            && let Some(nix) = ix.north()
        {
            let s = *v2[wix].0 + *v2[nix].0;
            if s % 3 == 0 {
                *v2[ix].0 += 10;
            }
        }
    }
    let expected: Vec<OwnedU8> = vec![
        OwnedU8::new(0),
        OwnedU8::new(1),
        OwnedU8::new(2),
        OwnedU8::new(3),
        OwnedU8::new(4),
        OwnedU8::new(15),
        OwnedU8::new(6),
        OwnedU8::new(7),
        OwnedU8::new(8),
    ];
    let expected = V2::new(expected).unwrap();
    assert_eq!(expected, v2);
}
