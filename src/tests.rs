use super::*;

#[test]
fn save_and_revert() {
    let mut rev = Reversible::new(4);
    assert_eq!(rev.as_mut(), &mut 4);
    *rev.as_mut() = 13;
    assert_eq!(rev.as_ref(), &4);
    assert_eq!(rev.as_mut(), &mut 13);
    rev.save();
    assert_eq!(rev.as_ref(), &13);
    *rev.as_mut() = 4;
    assert_eq!(rev.as_ref(), &13);
    assert_eq!(rev.as_mut(), &mut 4);
    rev.revert();
    assert_eq!(rev.as_mut(), &mut 13);
}
