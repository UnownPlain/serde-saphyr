#![cfg(all(feature = "serialize", feature = "deserialize"))]
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct HasStrings {
    zero: String,
    xnan: String,
    colon: String,
    comment: String,
    inline_hash: String,
    ending_colon: String,
    trim_ending_colon: String,
}

#[test]
fn strings_that_look_special_are_quoted() -> Result<()> {
    let v = HasStrings {
        zero: "0".to_string(),
        xnan: "nan".to_string(),
        colon: "a: b".to_string(),
        comment: "# hi".to_string(),
        inline_hash: "a#b".to_string(),
        ending_colon: "hi:".to_string(),
        trim_ending_colon: "hey:\n".to_string(),
    };

    let out = serde_saphyr::to_string(&v).expect("serialize");

    // Each of these fields should be rendered in a string-safe form so they are preserved
    // and do not get parsed as numbers, special floats, or mapping syntax.
    assert!(out.contains("zero: '0'"), "'0' must be quoted: {out}");
    assert!(out.contains("xnan: 'nan'"), "'nan' must be quoted: {out}");
    assert!(out.contains("'# hi'"), "comment must be quoted: {out}");
    assert!(
        out.contains("inline_hash: a#b"),
        "inline hash should stay plain: {out}"
    );
    assert!(
        out.contains("colon: 'a: b'"),
        "'a: b' must be quoted: {out}"
    );
    assert!(
        out.contains("ending_colon: 'hi:'"),
        "ending colon must be quoted: {out}"
    );
    assert!(
        out.contains("trim_ending_colon: |\n  hey:\n"),
        "ending colon multiline value must use a safe scalar style: {out}"
    );

    let r = serde_saphyr::from_str(&out)?;
    assert_eq!(v, r);
    Ok(())
}
