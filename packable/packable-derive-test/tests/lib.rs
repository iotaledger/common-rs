// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{ffi::OsStr, fs::read_dir, path::Path};

use trybuild::TestCases;

fn for_each_entry(root_path: &Path, f: impl Fn(&Path)) {
    let rs_ext = Some(OsStr::new("rs"));
    for entry in read_dir(root_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension() == rs_ext {
            f(&path);
        }
    }
}

macro_rules! make_test {
    ($($skip:ident),*) => {
        #[test]
        fn packable() {
            let path = Path::new("tests/");
            let cases = TestCases::new();

            for_each_entry(&path.join("pass/"), |path| {
                {
                    let path = path.with_extension("");
                    #[allow(unused_variables)]
                    let file_name = path.file_name();
                    $(
                        let skip = Some(OsStr::new(stringify!($skip)));
                        if file_name == skip {
                            return;
                        }
                    )*
                }
                cases.pass(path);
            });

            for_each_entry(&path.join("fail/"), |path| {
                {
                    let path = path.with_extension("");
                    #[allow(unused_variables)]
                    let file_name = path.file_name();
                    $(
                        let skip = Some(OsStr::new(stringify!($skip)));
                        if file_name == skip {
                            return;
                        }
                    )*
                }
                cases.compile_fail(path);
            });
        }
    };
}

// FIXME: change this when the new error message hits stable.
#[rustversion::stable]
make_test!();
#[rustversion::not(stable)]
make_test!(
    incorrect_tag_enum,
    invalid_packable_with,
    packable_is_structural,
    invalid_field_type_verify_with
);
