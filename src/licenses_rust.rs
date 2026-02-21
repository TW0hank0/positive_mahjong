#[allow(dead_code)]
pub fn get_licenses() -> Vec<LicenseInfo> {
  return vec![
    LicenseInfo {
        name: "ab_glyph",
        version: "0.2.32",
        license: "Apache-2.0",
        authors: vec!["Alex Butler <alexheretic@gmail.com>"] 
    },
    LicenseInfo {
        name: "ab_glyph_rasterizer",
        version: "0.1.10",
        license: "Apache-2.0",
        authors: vec!["Alex Butler <alexheretic@gmail.com>"] 
    },
    LicenseInfo {
        name: "ahash",
        version: "0.8.12",
        license: "MIT OR Apache-2.0",
        authors: vec!["Tom Kaitchuck <Tom.Kaitchuck@gmail.com>"] 
    },
    LicenseInfo {
        name: "android-activity",
        version: "0.6.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "android-build",
        version: "0.1.3",
        license: "MIT",
        authors: vec!["Kevin Boos <kevinaboos@gmail.com>", "Robius Project Maintainers"] 
    },
    LicenseInfo {
        name: "android-properties",
        version: "0.2.2",
        license: "MIT",
        authors: vec!["Mikhail Lappo <mikhail.lappo@esrlabs.com>"] 
    },
    LicenseInfo {
        name: "android_system_properties",
        version: "0.1.5",
        license: "MIT/Apache-2.0",
        authors: vec!["Nicolas Silva <nical@fastmail.com>"] 
    },
    LicenseInfo {
        name: "anyhow",
        version: "1.0.102",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "arc-swap",
        version: "1.8.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Michal 'vorner' Vaner <vorner@vorner.cz>"] 
    },
    LicenseInfo {
        name: "arrayref",
        version: "0.3.9",
        license: "BSD-2-Clause",
        authors: vec!["David Roundy <roundyd@physics.oregonstate.edu>"] 
    },
    LicenseInfo {
        name: "arrayvec",
        version: "0.7.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["bluss"] 
    },
    LicenseInfo {
        name: "as-raw-xcb-connection",
        version: "1.0.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "ascii",
        version: "1.1.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Thomas Bahn <thomas@thomas-bahn.net>", "Torbjørn Birch Moltu <t.b.moltu@lyse.net>", "Simon Sapin <simon.sapin@exyr.org>"] 
    },
    LicenseInfo {
        name: "ash",
        version: "0.38.0+1.3.281",
        license: "MIT OR Apache-2.0",
        authors: vec!["Maik Klein <maikklein@googlemail.com>", "Benjamin Saunders <ben.e.saunders@gmail.com>", "Marijn Suijten <marijn@traverseresearch.nl>"] 
    },
    LicenseInfo {
        name: "async-broadcast",
        version: "0.7.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "Yoshua Wuyts <yoshuawuyts@gmail.com>", "Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "async-channel",
        version: "2.5.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "async-executor",
        version: "1.14.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "async-io",
        version: "2.6.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "async-lock",
        version: "3.4.2",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "async-process",
        version: "2.5.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "async-recursion",
        version: "1.1.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Robert Usher <266585+dcchut@users.noreply.github.com>"] 
    },
    LicenseInfo {
        name: "async-signal",
        version: "0.2.13",
        license: "Apache-2.0 OR MIT",
        authors: vec!["John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "async-task",
        version: "4.7.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "async-trait",
        version: "0.1.89",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "atomic-waker",
        version: "1.1.2",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "Contributors to futures-rs"] 
    },
    LicenseInfo {
        name: "autocfg",
        version: "1.5.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Josh Stone <cuviper@gmail.com>"] 
    },
    LicenseInfo {
        name: "aws-lc-rs",
        version: "1.15.4",
        license: "ISC AND (Apache-2.0 OR ISC)",
        authors: vec!["AWS-LibCrypto"] 
    },
    LicenseInfo {
        name: "aws-lc-sys",
        version: "0.37.1",
        license: "ISC AND (Apache-2.0 OR ISC) AND OpenSSL",
        authors: vec!["AWS-LC"] 
    },
    LicenseInfo {
        name: "base64",
        version: "0.22.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Marshall Pierce <marshall@mpierce.org>"] 
    },
    LicenseInfo {
        name: "bit-set",
        version: "0.8.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Alexis Beingessner <a.beingessner@gmail.com>"] 
    },
    LicenseInfo {
        name: "bit-vec",
        version: "0.8.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Alexis Beingessner <a.beingessner@gmail.com>"] 
    },
    LicenseInfo {
        name: "bitflags",
        version: "1.3.2",
        license: "MIT/Apache-2.0",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "bitflags",
        version: "2.11.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "block",
        version: "0.1.6",
        license: "MIT",
        authors: vec!["Steven Sheldon"] 
    },
    LicenseInfo {
        name: "block2",
        version: "0.5.1",
        license: "MIT",
        authors: vec!["Steven Sheldon", "Mads Marquart <mads@marquart.dk>"] 
    },
    LicenseInfo {
        name: "block2",
        version: "0.6.2",
        license: "MIT",
        authors: vec!["Mads Marquart <mads@marquart.dk>"] 
    },
    LicenseInfo {
        name: "blocking",
        version: "1.6.2",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "bumpalo",
        version: "3.19.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Nick Fitzgerald <fitzgen@gmail.com>"] 
    },
    LicenseInfo {
        name: "bytemuck",
        version: "1.25.0",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec!["Lokathor <zefria@gmail.com>"] 
    },
    LicenseInfo {
        name: "bytemuck_derive",
        version: "1.10.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec!["Lokathor <zefria@gmail.com>"] 
    },
    LicenseInfo {
        name: "byteorder",
        version: "1.5.0",
        license: "Unlicense OR MIT",
        authors: vec!["Andrew Gallant <jamslam@gmail.com>"] 
    },
    LicenseInfo {
        name: "bytes",
        version: "1.11.1",
        license: "MIT",
        authors: vec!["Carl Lerche <me@carllerche.com>", "Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "calloop",
        version: "0.13.0",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "calloop",
        version: "0.14.4",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "calloop-wayland-source",
        version: "0.3.0",
        license: "MIT",
        authors: vec!["Kirill Chibisov <contact@kchibisov.com>"] 
    },
    LicenseInfo {
        name: "calloop-wayland-source",
        version: "0.4.1",
        license: "MIT",
        authors: vec!["Kirill Chibisov <contact@kchibisov.com>"] 
    },
    LicenseInfo {
        name: "camino",
        version: "1.2.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Without Boats <saoirse@without.boats>", "Ashley Williams <ashley666ashley@gmail.com>", "Steve Klabnik <steve@steveklabnik.com>", "Rain <rain@sunshowers.io>"] 
    },
    LicenseInfo {
        name: "cargo-platform",
        version: "0.1.9",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "cargo_metadata",
        version: "0.19.2",
        license: "MIT",
        authors: vec!["Oliver Schneider <git-spam-no-reply9815368754983@oli-obk.de>"] 
    },
    LicenseInfo {
        name: "cc",
        version: "1.2.56",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "cesu8",
        version: "1.1.0",
        license: "Apache-2.0/MIT",
        authors: vec!["Eric Kidd <git@randomhacks.net>"] 
    },
    LicenseInfo {
        name: "cfg-if",
        version: "1.0.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "cfg_aliases",
        version: "0.2.1",
        license: "MIT",
        authors: vec!["Zicklag <zicklag@katharostech.com>"] 
    },
    LicenseInfo {
        name: "chacha20",
        version: "0.10.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["RustCrypto Developers"] 
    },
    LicenseInfo {
        name: "chrono",
        version: "0.4.43",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "chunked_transfer",
        version: "1.5.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Corey Farwell <coreyf@rwell.org>"] 
    },
    LicenseInfo {
        name: "clipboard-win",
        version: "5.4.1",
        license: "BSL-1.0",
        authors: vec!["Douman <douman@gmx.se>"] 
    },
    LicenseInfo {
        name: "clipboard_macos",
        version: "0.1.1",
        license: "Apache-2.0",
        authors: vec!["Héctor Ramón Jiménez <hector0193@gmail.com>"] 
    },
    LicenseInfo {
        name: "clipboard_wayland",
        version: "0.2.2",
        license: "Apache-2.0",
        authors: vec!["Héctor Ramón Jiménez <hector0193@gmail.com>"] 
    },
    LicenseInfo {
        name: "clipboard_x11",
        version: "0.4.3",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector0193@gmail.com>"] 
    },
    LicenseInfo {
        name: "cmake",
        version: "0.1.57",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "codespan-reporting",
        version: "0.12.0",
        license: "Apache-2.0",
        authors: vec!["Brendan Zabarauskas <bjzaba@yahoo.com.au>"] 
    },
    LicenseInfo {
        name: "colored",
        version: "3.1.1",
        license: "MPL-2.0",
        authors: vec!["Thomas Wickham <mackwic@gmail.com>"] 
    },
    LicenseInfo {
        name: "combine",
        version: "4.6.7",
        license: "MIT",
        authors: vec!["Markus Westerlind <marwes91@gmail.com>"] 
    },
    LicenseInfo {
        name: "concurrent-queue",
        version: "2.5.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "Taiki Endo <te316e89@gmail.com>", "John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "core-foundation",
        version: "0.9.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "core-foundation",
        version: "0.10.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "core-foundation-sys",
        version: "0.8.7",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "core-graphics",
        version: "0.23.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "core-graphics-types",
        version: "0.1.3",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "core-graphics-types",
        version: "0.2.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "core_maths",
        version: "0.1.1",
        license: "MIT",
        authors: vec!["Robert Bastian <me@robertbastian.dev"] 
    },
    LicenseInfo {
        name: "cosmic-text",
        version: "0.15.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Jeremy Soller <jeremy@system76.com>"] 
    },
    LicenseInfo {
        name: "cpufeatures",
        version: "0.3.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["RustCrypto Developers"] 
    },
    LicenseInfo {
        name: "crossbeam-utils",
        version: "0.8.21",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "crunchy",
        version: "0.2.4",
        license: "MIT",
        authors: vec!["Eira Fransham <jackefransham@gmail.com>"] 
    },
    LicenseInfo {
        name: "cryoglyph",
        version: "0.1.0",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "ctor-lite",
        version: "0.1.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "cursor-icon",
        version: "1.2.0",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["Kirill Chibisov <contact@kchibisov.com>"] 
    },
    LicenseInfo {
        name: "darling",
        version: "0.20.11",
        license: "MIT",
        authors: vec!["Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "darling_core",
        version: "0.20.11",
        license: "MIT",
        authors: vec!["Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "darling_macro",
        version: "0.20.11",
        license: "MIT",
        authors: vec!["Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "derive_builder",
        version: "0.20.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Colin Kiegel <kiegel@gmx.de>", "Pascal Hertleif <killercup@gmail.com>", "Jan-Erik Rediger <janerik@fnordig.de>", "Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "derive_builder_core",
        version: "0.20.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Colin Kiegel <kiegel@gmx.de>", "Pascal Hertleif <killercup@gmail.com>", "Jan-Erik Rediger <janerik@fnordig.de>", "Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "derive_builder_macro",
        version: "0.20.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Colin Kiegel <kiegel@gmx.de>", "Pascal Hertleif <killercup@gmail.com>", "Jan-Erik Rediger <janerik@fnordig.de>", "Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "derive_more",
        version: "2.1.1",
        license: "MIT",
        authors: vec!["Jelte Fennema <github-tech@jeltef.nl>"] 
    },
    LicenseInfo {
        name: "derive_more-impl",
        version: "2.1.1",
        license: "MIT",
        authors: vec!["Jelte Fennema <github-tech@jeltef.nl>"] 
    },
    LicenseInfo {
        name: "destructure_traitobject",
        version: "0.2.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Jonathan Reem <jonathan.reem@gmail.com>", "Steven Fackler <sfackler@gmail.com>", "Alexander Regueiro <alexreg@me.com>", "Philip Peterson <philip.c.peterson@gmail.com>"] 
    },
    LicenseInfo {
        name: "dispatch",
        version: "0.2.0",
        license: "MIT",
        authors: vec!["Steven Sheldon"] 
    },
    LicenseInfo {
        name: "dispatch2",
        version: "0.3.0",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec!["Mads Marquart <mads@marquart.dk>", "Mary <mary@mary.zone>"] 
    },
    LicenseInfo {
        name: "displaydoc",
        version: "0.2.5",
        license: "MIT OR Apache-2.0",
        authors: vec!["Jane Lusby <jlusby@yaah.dev>"] 
    },
    LicenseInfo {
        name: "dlib",
        version: "0.5.2",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "document-features",
        version: "0.2.12",
        license: "MIT OR Apache-2.0",
        authors: vec!["Slint Developers <info@slint.dev>"] 
    },
    LicenseInfo {
        name: "downcast-rs",
        version: "1.2.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Ashish Myles <marcianx@gmail.com>", "Runji Wang <wangrunji0408@163.com>"] 
    },
    LicenseInfo {
        name: "dpi",
        version: "0.1.2",
        license: "Apache-2.0 AND MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "dunce",
        version: "1.0.5",
        license: "CC0-1.0 OR MIT-0 OR Apache-2.0",
        authors: vec!["Kornel <kornel@geekhood.net>"] 
    },
    LicenseInfo {
        name: "either",
        version: "1.15.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["bluss"] 
    },
    LicenseInfo {
        name: "encoding_rs",
        version: "0.8.35",
        license: "(Apache-2.0 OR MIT) AND BSD-3-Clause",
        authors: vec!["Henri Sivonen <hsivonen@hsivonen.fi>"] 
    },
    LicenseInfo {
        name: "endi",
        version: "1.1.1",
        license: "MIT",
        authors: vec!["Zeeshan Ali Khan <zeenix@gmail.com>"] 
    },
    LicenseInfo {
        name: "enumflags2",
        version: "0.7.12",
        license: "MIT OR Apache-2.0",
        authors: vec!["maik klein <maikklein@googlemail.com>", "Maja Kądziołka <maya@compilercrim.es>"] 
    },
    LicenseInfo {
        name: "enumflags2_derive",
        version: "0.7.12",
        license: "MIT OR Apache-2.0",
        authors: vec!["maik klein <maikklein@googlemail.com>", "Maja Kądziołka <maya@compilercrim.es>"] 
    },
    LicenseInfo {
        name: "equivalent",
        version: "1.0.2",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "errno",
        version: "0.3.14",
        license: "MIT OR Apache-2.0",
        authors: vec!["Chris Wong <lambda.fairy@gmail.com>", "Dan Gohman <dev@sunfishcode.online>"] 
    },
    LicenseInfo {
        name: "error-code",
        version: "3.3.2",
        license: "BSL-1.0",
        authors: vec!["Douman <douman@gmx.se>"] 
    },
    LicenseInfo {
        name: "etagere",
        version: "0.2.15",
        license: "MIT/Apache-2.0",
        authors: vec!["Nicolas Silva <nical@fastmail.com>"] 
    },
    LicenseInfo {
        name: "euclid",
        version: "0.22.13",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "event-listener",
        version: "5.4.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "event-listener-strategy",
        version: "0.5.4",
        license: "Apache-2.0 OR MIT",
        authors: vec!["John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "fastrand",
        version: "2.3.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>"] 
    },
    LicenseInfo {
        name: "find-msvc-tools",
        version: "0.1.9",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "fnv",
        version: "1.0.7",
        license: "Apache-2.0 / MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "foldhash",
        version: "0.1.5",
        license: "Zlib",
        authors: vec!["Orson Peters <orsonpeters@gmail.com>"] 
    },
    LicenseInfo {
        name: "foldhash",
        version: "0.2.0",
        license: "Zlib",
        authors: vec!["Orson Peters <orsonpeters@gmail.com>"] 
    },
    LicenseInfo {
        name: "font-types",
        version: "0.10.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "fontconfig-parser",
        version: "0.5.8",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "fontdb",
        version: "0.23.0",
        license: "MIT",
        authors: vec!["Yevhenii Reizner <razrfalcon@gmail.com>"] 
    },
    LicenseInfo {
        name: "foreign-types",
        version: "0.3.2",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "foreign-types",
        version: "0.5.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "foreign-types-macros",
        version: "0.2.3",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "foreign-types-shared",
        version: "0.1.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "foreign-types-shared",
        version: "0.3.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "form_urlencoded",
        version: "1.2.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["The rust-url developers"] 
    },
    LicenseInfo {
        name: "fs_extra",
        version: "1.3.0",
        license: "MIT",
        authors: vec!["Denis Kurilenko <webdesus@gmail.com>"] 
    },
    LicenseInfo {
        name: "futures",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-channel",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-core",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-executor",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-io",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-lite",
        version: "2.6.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "Contributors to futures-rs"] 
    },
    LicenseInfo {
        name: "futures-macro",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-sink",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-task",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "futures-util",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "gethostname",
        version: "1.1.0",
        license: "Apache-2.0",
        authors: vec!["Sebastian Wiesner <sebastian@swsnr.de>"] 
    },
    LicenseInfo {
        name: "getrandom",
        version: "0.2.17",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers"] 
    },
    LicenseInfo {
        name: "getrandom",
        version: "0.3.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers"] 
    },
    LicenseInfo {
        name: "getrandom",
        version: "0.4.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers"] 
    },
    LicenseInfo {
        name: "getset",
        version: "0.1.6",
        license: "MIT",
        authors: vec!["Ana Hobden <ana@hoverbear.org>", "John Baublitz <john.m.baublitz@gmail.com"] 
    },
    LicenseInfo {
        name: "gl_generator",
        version: "0.14.0",
        license: "Apache-2.0",
        authors: vec!["Brendan Zabarauskas <bjzaba@yahoo.com.au>", "Corey Richardson", "Arseny Kapoulkine"] 
    },
    LicenseInfo {
        name: "glam",
        version: "0.25.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Cameron Hart <cameron.hart@gmail.com>"] 
    },
    LicenseInfo {
        name: "glow",
        version: "0.16.0",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["Joshua Groves <josh@joshgroves.com>", "Dzmitry Malyshau <kvarkus@gmail.com>"] 
    },
    LicenseInfo {
        name: "glutin_wgl_sys",
        version: "0.6.1",
        license: "Apache-2.0",
        authors: vec!["Kirill Chibisov <contact@kchibisov.com>"] 
    },
    LicenseInfo {
        name: "gpu-alloc",
        version: "0.6.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Zakarum <zakarumych@ya.ru>"] 
    },
    LicenseInfo {
        name: "gpu-alloc-types",
        version: "0.3.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Zakarum <zakarumych@ya.ru>"] 
    },
    LicenseInfo {
        name: "gpu-allocator",
        version: "0.27.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Traverse Research <opensource@traverseresearch.nl>"] 
    },
    LicenseInfo {
        name: "gpu-descriptor",
        version: "0.3.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Zakarum <zakarumych@ya.ru>"] 
    },
    LicenseInfo {
        name: "gpu-descriptor-types",
        version: "0.2.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Zakarum <zakarumych@ya.ru>"] 
    },
    LicenseInfo {
        name: "guillotiere",
        version: "0.6.2",
        license: "MIT/Apache-2.0",
        authors: vec!["Nicolas Silva <nical@fastmail.com>"] 
    },
    LicenseInfo {
        name: "h2",
        version: "0.4.13",
        license: "MIT",
        authors: vec!["Carl Lerche <me@carllerche.com>", "Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "half",
        version: "2.7.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Kathryn Long <squeeself@gmail.com>"] 
    },
    LicenseInfo {
        name: "harfrust",
        version: "0.3.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "hashbrown",
        version: "0.15.5",
        license: "MIT OR Apache-2.0",
        authors: vec!["Amanieu d'Antras <amanieu@gmail.com>"] 
    },
    LicenseInfo {
        name: "hashbrown",
        version: "0.16.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Amanieu d'Antras <amanieu@gmail.com>"] 
    },
    LicenseInfo {
        name: "heck",
        version: "0.5.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "hermit-abi",
        version: "0.5.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Stefan Lankes"] 
    },
    LicenseInfo {
        name: "hex",
        version: "0.4.3",
        license: "MIT OR Apache-2.0",
        authors: vec!["KokaKiwi <kokakiwi@kokakiwi.net>"] 
    },
    LicenseInfo {
        name: "hexf-parse",
        version: "0.2.1",
        license: "CC0-1.0",
        authors: vec!["Kang Seonghoon <public+rust@mearie.org>"] 
    },
    LicenseInfo {
        name: "http",
        version: "1.4.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>", "Carl Lerche <me@carllerche.com>", "Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "http-body",
        version: "1.0.1",
        license: "MIT",
        authors: vec!["Carl Lerche <me@carllerche.com>", "Lucio Franco <luciofranco14@gmail.com>", "Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "http-body-util",
        version: "0.1.3",
        license: "MIT",
        authors: vec!["Carl Lerche <me@carllerche.com>", "Lucio Franco <luciofranco14@gmail.com>", "Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "httparse",
        version: "1.10.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "httpdate",
        version: "1.0.3",
        license: "MIT OR Apache-2.0",
        authors: vec!["Pyfisch <pyfisch@posteo.org>"] 
    },
    LicenseInfo {
        name: "humantime",
        version: "2.3.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "hyper",
        version: "1.8.1",
        license: "MIT",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "hyper-rustls",
        version: "0.27.7",
        license: "Apache-2.0 OR ISC OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "hyper-tls",
        version: "0.6.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "hyper-util",
        version: "0.1.20",
        license: "MIT",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "iana-time-zone",
        version: "0.1.65",
        license: "MIT OR Apache-2.0",
        authors: vec!["Andrew Straw <strawman@astraw.com>", "René Kijewski <rene.kijewski@fu-berlin.de>", "Ryan Lopopolo <rjl@hyperbo.la>"] 
    },
    LicenseInfo {
        name: "iana-time-zone-haiku",
        version: "0.1.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["René Kijewski <crates.io@k6i.de>"] 
    },
    LicenseInfo {
        name: "iced",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_core",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_debug",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_futures",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_graphics",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_program",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_renderer",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_runtime",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_tiny_skia",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_wgpu",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_widget",
        version: "0.14.2",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "iced_winit",
        version: "0.14.0",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector@hecrj.dev>"] 
    },
    LicenseInfo {
        name: "icu_collections",
        version: "2.1.1",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "icu_locale_core",
        version: "2.1.1",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "icu_normalizer",
        version: "2.1.1",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "icu_normalizer_data",
        version: "2.1.1",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "icu_properties",
        version: "2.1.2",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "icu_properties_data",
        version: "2.1.2",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "icu_provider",
        version: "2.1.1",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "id-arena",
        version: "2.3.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Nick Fitzgerald <fitzgen@gmail.com>", "Aleksey Kladov <aleksey.kladov@gmail.com>"] 
    },
    LicenseInfo {
        name: "ident_case",
        version: "1.0.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Ted Driggs <ted.driggs@outlook.com>"] 
    },
    LicenseInfo {
        name: "idna",
        version: "1.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The rust-url developers"] 
    },
    LicenseInfo {
        name: "idna_adapter",
        version: "1.2.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["The rust-url developers"] 
    },
    LicenseInfo {
        name: "indexmap",
        version: "2.13.0",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "ipnet",
        version: "2.11.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Kris Price <kris@krisprice.nz>"] 
    },
    LicenseInfo {
        name: "iri-string",
        version: "0.7.10",
        license: "MIT OR Apache-2.0",
        authors: vec!["YOSHIOKA Takuma <nop_thread@nops.red>"] 
    },
    LicenseInfo {
        name: "itoa",
        version: "1.0.17",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "jni",
        version: "0.21.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Josh Chase <josh@prevoty.com>"] 
    },
    LicenseInfo {
        name: "jni-sys",
        version: "0.3.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "jobserver",
        version: "0.1.34",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "js-sys",
        version: "0.3.85",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "khronos-egl",
        version: "6.0.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Timothée Haudebourg <author@haudebourg.net>", "Sean Kerr <sean@metatomic.io>"] 
    },
    LicenseInfo {
        name: "khronos_api",
        version: "3.1.0",
        license: "Apache-2.0",
        authors: vec!["Brendan Zabarauskas <bjzaba@yahoo.com.au>", "Corey Richardson", "Arseny Kapoulkine", "Pierre Krieger <pierre.krieger1708@gmail.com>"] 
    },
    LicenseInfo {
        name: "kurbo",
        version: "0.10.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["Raph Levien <raph.levien@gmail.com>"] 
    },
    LicenseInfo {
        name: "leb128fmt",
        version: "0.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Bryant Luk <code@bryantluk.com>"] 
    },
    LicenseInfo {
        name: "libc",
        version: "0.2.182",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "libloading",
        version: "0.8.9",
        license: "ISC",
        authors: vec!["Simonas Kazlauskas <libloading@kazlauskas.me>"] 
    },
    LicenseInfo {
        name: "libm",
        version: "0.2.16",
        license: "MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>", "Amanieu d'Antras <amanieu@gmail.com>", "Jorge Aparicio <japaricious@gmail.com>", "Trevor Gross <tg@trevorgross.com>"] 
    },
    LicenseInfo {
        name: "libredox",
        version: "0.1.12",
        license: "MIT",
        authors: vec!["4lDO2 <4lDO2@protonmail.com>"] 
    },
    LicenseInfo {
        name: "lilt",
        version: "0.8.1",
        license: "MIT",
        authors: vec!["cyypherus"] 
    },
    LicenseInfo {
        name: "linebender_resource_handle",
        version: "0.1.1",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "linux-raw-sys",
        version: "0.4.15",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Dan Gohman <dev@sunfishcode.online>"] 
    },
    LicenseInfo {
        name: "linux-raw-sys",
        version: "0.11.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Dan Gohman <dev@sunfishcode.online>"] 
    },
    LicenseInfo {
        name: "litemap",
        version: "0.8.1",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "litrs",
        version: "1.0.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Lukas Kalbertodt <lukas.kalbertodt@gmail.com>"] 
    },
    LicenseInfo {
        name: "local-ip-address",
        version: "0.6.10",
        license: "MIT OR Apache-2.0",
        authors: vec!["Leo Borai <estebanborai@gmail.com>"] 
    },
    LicenseInfo {
        name: "lock_api",
        version: "0.4.14",
        license: "MIT OR Apache-2.0",
        authors: vec!["Amanieu d'Antras <amanieu@gmail.com>"] 
    },
    LicenseInfo {
        name: "log",
        version: "0.4.29",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "log-mdc",
        version: "0.1.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "log4rs",
        version: "1.4.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>", "Evan Simmons <esims89@gmail.com>"] 
    },
    LicenseInfo {
        name: "lru",
        version: "0.16.3",
        license: "MIT",
        authors: vec!["Jerome Froelich <jeromefroelic@hotmail.com>"] 
    },
    LicenseInfo {
        name: "lru-slab",
        version: "0.1.2",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["Benjamin Saunders <ben.e.saunders@gmail.com>"] 
    },
    LicenseInfo {
        name: "malloc_buf",
        version: "0.0.6",
        license: "MIT",
        authors: vec!["Steven Sheldon"] 
    },
    LicenseInfo {
        name: "memchr",
        version: "2.8.0",
        license: "Unlicense OR MIT",
        authors: vec!["Andrew Gallant <jamslam@gmail.com>", "bluss"] 
    },
    LicenseInfo {
        name: "memmap2",
        version: "0.9.10",
        license: "MIT OR Apache-2.0",
        authors: vec!["Dan Burkert <dan@danburkert.com>", "Yevhenii Reizner <razrfalcon@gmail.com>", "The Contributors"] 
    },
    LicenseInfo {
        name: "memoffset",
        version: "0.9.1",
        license: "MIT",
        authors: vec!["Gilad Naaman <gilad.naaman@gmail.com>"] 
    },
    LicenseInfo {
        name: "metal",
        version: "0.32.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "mime",
        version: "0.3.17",
        license: "MIT OR Apache-2.0",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "mio",
        version: "1.1.1",
        license: "MIT",
        authors: vec!["Carl Lerche <me@carllerche.com>", "Thomas de Zeeuw <thomasdezeeuw@gmail.com>", "Tokio Contributors <team@tokio.rs>"] 
    },
    LicenseInfo {
        name: "mock_instant",
        version: "0.6.0",
        license: "0BSD",
        authors: vec!["museun <museun@outlook.com>"] 
    },
    LicenseInfo {
        name: "mundy",
        version: "0.2.2",
        license: "Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "naga",
        version: "27.0.3",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "native-tls",
        version: "0.2.16",
        license: "MIT OR Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "ndk",
        version: "0.9.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Mobile contributors"] 
    },
    LicenseInfo {
        name: "ndk-context",
        version: "0.1.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Windowing contributors"] 
    },
    LicenseInfo {
        name: "ndk-sys",
        version: "0.6.0+11769913",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Windowing contributors"] 
    },
    LicenseInfo {
        name: "neli",
        version: "0.7.4",
        license: "BSD-3-Clause",
        authors: vec!["John Baublitz <john.m.baublitz@gmail.com>"] 
    },
    LicenseInfo {
        name: "neli-proc-macros",
        version: "0.2.2",
        license: "BSD-3-Clause",
        authors: vec!["John Baublitz <john.m.baublitz@gmail.com>"] 
    },
    LicenseInfo {
        name: "num-traits",
        version: "0.2.19",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "num_enum",
        version: "0.7.5",
        license: "BSD-3-Clause OR MIT OR Apache-2.0",
        authors: vec!["Daniel Wagner-Hall <dawagner@gmail.com>", "Daniel Henry-Mantilla <daniel.henry.mantilla@gmail.com>", "Vincent Esche <regexident@gmail.com>"] 
    },
    LicenseInfo {
        name: "num_enum_derive",
        version: "0.7.5",
        license: "BSD-3-Clause OR MIT OR Apache-2.0",
        authors: vec!["Daniel Wagner-Hall <dawagner@gmail.com>", "Daniel Henry-Mantilla <daniel.henry.mantilla@gmail.com>", "Vincent Esche <regexident@gmail.com>"] 
    },
    LicenseInfo {
        name: "objc",
        version: "0.2.7",
        license: "MIT",
        authors: vec!["Steven Sheldon"] 
    },
    LicenseInfo {
        name: "objc-sys",
        version: "0.3.5",
        license: "MIT",
        authors: vec!["Mads Marquart <mads@marquart.dk>"] 
    },
    LicenseInfo {
        name: "objc2",
        version: "0.5.2",
        license: "MIT",
        authors: vec!["Steven Sheldon", "Mads Marquart <mads@marquart.dk>"] 
    },
    LicenseInfo {
        name: "objc2",
        version: "0.6.3",
        license: "MIT",
        authors: vec!["Mads Marquart <mads@marquart.dk>"] 
    },
    LicenseInfo {
        name: "objc2-app-kit",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-app-kit",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-cloud-kit",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-cloud-kit",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-contacts",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-data",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-data",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-foundation",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-graphics",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-image",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-image",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-location",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-text",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-core-video",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-encode",
        version: "4.1.0",
        license: "MIT",
        authors: vec!["Mads Marquart <mads@marquart.dk>"] 
    },
    LicenseInfo {
        name: "objc2-foundation",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-foundation",
        version: "0.3.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-io-surface",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-link-presentation",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-metal",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-quartz-core",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-quartz-core",
        version: "0.3.2",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-symbols",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-ui-kit",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-uniform-type-identifiers",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "objc2-user-notifications",
        version: "0.2.2",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "once_cell",
        version: "1.21.3",
        license: "MIT OR Apache-2.0",
        authors: vec!["Aleksey Kladov <aleksey.kladov@gmail.com>"] 
    },
    LicenseInfo {
        name: "openssl",
        version: "0.10.75",
        license: "Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "openssl-macros",
        version: "0.1.1",
        license: "MIT/Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "openssl-probe",
        version: "0.2.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "openssl-sys",
        version: "0.9.111",
        license: "MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>", "Steven Fackler <sfackler@gmail.com>"] 
    },
    LicenseInfo {
        name: "orbclient",
        version: "0.3.50",
        license: "MIT",
        authors: vec!["Jeremy Soller <jackpot51@gmail.com>"] 
    },
    LicenseInfo {
        name: "ordered-float",
        version: "2.10.1",
        license: "MIT",
        authors: vec!["Jonathan Reem <jonathan.reem@gmail.com>", "Matt Brubeck <mbrubeck@limpet.net>"] 
    },
    LicenseInfo {
        name: "ordered-float",
        version: "5.1.0",
        license: "MIT",
        authors: vec!["Jonathan Reem <jonathan.reem@gmail.com>", "Matt Brubeck <mbrubeck@limpet.net>"] 
    },
    LicenseInfo {
        name: "ordered-stream",
        version: "0.2.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Daniel De Graaf <code@danieldg.net>", "Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "owned_ttf_parser",
        version: "0.25.1",
        license: "Apache-2.0",
        authors: vec!["Alex Butler <alexheretic@gmail.com>"] 
    },
    LicenseInfo {
        name: "parking",
        version: "2.2.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "parking_lot",
        version: "0.12.5",
        license: "MIT OR Apache-2.0",
        authors: vec!["Amanieu d'Antras <amanieu@gmail.com>"] 
    },
    LicenseInfo {
        name: "parking_lot_core",
        version: "0.9.12",
        license: "MIT OR Apache-2.0",
        authors: vec!["Amanieu d'Antras <amanieu@gmail.com>"] 
    },
    LicenseInfo {
        name: "paste",
        version: "1.0.15",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "percent-encoding",
        version: "2.3.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["The rust-url developers"] 
    },
    LicenseInfo {
        name: "pin-project",
        version: "1.1.10",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "pin-project-internal",
        version: "1.1.10",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "pin-project-lite",
        version: "0.2.16",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "pin-utils",
        version: "0.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Josef Brandl <mail@josefbrandl.de>"] 
    },
    LicenseInfo {
        name: "piper",
        version: "0.2.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "pkg-config",
        version: "0.3.32",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "polling",
        version: "3.11.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Stjepan Glavina <stjepang@gmail.com>", "John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "portable-atomic",
        version: "1.13.1",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "portable-atomic-util",
        version: "0.2.5",
        license: "Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "positive_tool_rs",
        version: "0.7.0",
        license: "AGPL-3.0-only",
        authors: vec![] 
    },
    LicenseInfo {
        name: "potential_utf",
        version: "0.1.4",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "ppv-lite86",
        version: "0.2.21",
        license: "MIT OR Apache-2.0",
        authors: vec!["The CryptoCorrosion Contributors"] 
    },
    LicenseInfo {
        name: "presser",
        version: "0.3.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Embark <opensource@embark-studios.com>", "Gray Olson <gray@grayolson.com"] 
    },
    LicenseInfo {
        name: "prettyplease",
        version: "0.2.37",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "proc-macro-crate",
        version: "3.4.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Bastian Köcher <git@kchr.de>"] 
    },
    LicenseInfo {
        name: "proc-macro-error-attr2",
        version: "2.0.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["CreepySkeleton <creepy-skeleton@yandex.ru>", "GnomedDev <david2005thomas@gmail.com>"] 
    },
    LicenseInfo {
        name: "proc-macro-error2",
        version: "2.0.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["CreepySkeleton <creepy-skeleton@yandex.ru>", "GnomedDev <david2005thomas@gmail.com>"] 
    },
    LicenseInfo {
        name: "proc-macro2",
        version: "1.0.106",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>", "Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "profiling",
        version: "1.0.17",
        license: "MIT OR Apache-2.0",
        authors: vec!["Philip Degarmo <aclysma@gmail.com>"] 
    },
    LicenseInfo {
        name: "quick-xml",
        version: "0.38.4",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "quinn",
        version: "0.11.9",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "quinn-proto",
        version: "0.11.13",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "quinn-udp",
        version: "0.5.14",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "quote",
        version: "1.0.44",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "r-efi",
        version: "5.3.0",
        license: "MIT OR Apache-2.0 OR LGPL-2.1-or-later",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rand",
        version: "0.9.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers", "The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "rand",
        version: "0.10.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers", "The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "rand_chacha",
        version: "0.9.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers", "The Rust Project Developers", "The CryptoCorrosion Contributors"] 
    },
    LicenseInfo {
        name: "rand_core",
        version: "0.9.5",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers", "The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "rand_core",
        version: "0.10.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Rand Project Developers"] 
    },
    LicenseInfo {
        name: "range-alloc",
        version: "0.1.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["the gfx-rs Developers"] 
    },
    LicenseInfo {
        name: "rangemap",
        version: "1.7.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Jeff Parsons <jeff@parsons.io>"] 
    },
    LicenseInfo {
        name: "raw-window-handle",
        version: "0.6.2",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["Osspial <osspial@gmail.com>"] 
    },
    LicenseInfo {
        name: "read-fonts",
        version: "0.35.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "redox_syscall",
        version: "0.4.1",
        license: "MIT",
        authors: vec!["Jeremy Soller <jackpot51@gmail.com>"] 
    },
    LicenseInfo {
        name: "redox_syscall",
        version: "0.5.18",
        license: "MIT",
        authors: vec!["Jeremy Soller <jackpot51@gmail.com>"] 
    },
    LicenseInfo {
        name: "redox_syscall",
        version: "0.7.1",
        license: "MIT",
        authors: vec!["Jeremy Soller <jackpot51@gmail.com>"] 
    },
    LicenseInfo {
        name: "renderdoc-sys",
        version: "1.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Eyal Kalderon <ebkalderon@gmail.com>"] 
    },
    LicenseInfo {
        name: "reqwest",
        version: "0.13.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "ring",
        version: "0.17.14",
        license: "Apache-2.0 AND ISC",
        authors: vec![] 
    },
    LicenseInfo {
        name: "roxmltree",
        version: "0.20.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Yevhenii Reizner <razrfalcon@gmail.com>"] 
    },
    LicenseInfo {
        name: "rustc-hash",
        version: "1.1.0",
        license: "Apache-2.0/MIT",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "rustc-hash",
        version: "2.1.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["The Rust Project Developers"] 
    },
    LicenseInfo {
        name: "rustc_version",
        version: "0.4.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustix",
        version: "0.38.44",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Dan Gohman <dev@sunfishcode.online>", "Jakub Konka <kubkon@jakubkonka.com>"] 
    },
    LicenseInfo {
        name: "rustix",
        version: "1.1.3",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Dan Gohman <dev@sunfishcode.online>", "Jakub Konka <kubkon@jakubkonka.com>"] 
    },
    LicenseInfo {
        name: "rustls",
        version: "0.23.36",
        license: "Apache-2.0 OR ISC OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustls-native-certs",
        version: "0.8.3",
        license: "Apache-2.0 OR ISC OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustls-pki-types",
        version: "1.14.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustls-platform-verifier",
        version: "0.6.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustls-platform-verifier-android",
        version: "0.1.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustls-webpki",
        version: "0.103.9",
        license: "ISC",
        authors: vec![] 
    },
    LicenseInfo {
        name: "rustversion",
        version: "1.0.22",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "ryu",
        version: "1.0.23",
        license: "Apache-2.0 OR BSL-1.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "same-file",
        version: "1.0.6",
        license: "Unlicense/MIT",
        authors: vec!["Andrew Gallant <jamslam@gmail.com>"] 
    },
    LicenseInfo {
        name: "schannel",
        version: "0.1.28",
        license: "MIT",
        authors: vec!["Steven Fackler <sfackler@gmail.com>", "Steffen Butzer <steffen.butzer@outlook.com>"] 
    },
    LicenseInfo {
        name: "scoped-tls",
        version: "1.0.1",
        license: "MIT/Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "scopeguard",
        version: "1.2.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["bluss"] 
    },
    LicenseInfo {
        name: "sctk-adwaita",
        version: "0.10.1",
        license: "MIT",
        authors: vec!["Poly <marynczak.bartlomiej@gmail.com>"] 
    },
    LicenseInfo {
        name: "security-framework",
        version: "3.6.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>", "Kornel <kornel@geekhood.net>"] 
    },
    LicenseInfo {
        name: "security-framework-sys",
        version: "2.16.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Steven Fackler <sfackler@gmail.com>", "Kornel <kornel@geekhood.net>"] 
    },
    LicenseInfo {
        name: "self_cell",
        version: "1.2.2",
        license: "Apache-2.0 OR GPL-2.0-only",
        authors: vec!["Lukas Bergdoll <lukas.bergdoll@gmail.com>"] 
    },
    LicenseInfo {
        name: "semver",
        version: "1.0.27",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "serde",
        version: "1.0.228",
        license: "MIT OR Apache-2.0",
        authors: vec!["Erick Tryzelaar <erick.tryzelaar@gmail.com>", "David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "serde-value",
        version: "0.7.0",
        license: "MIT",
        authors: vec!["arcnmx"] 
    },
    LicenseInfo {
        name: "serde_core",
        version: "1.0.228",
        license: "MIT OR Apache-2.0",
        authors: vec!["Erick Tryzelaar <erick.tryzelaar@gmail.com>", "David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "serde_derive",
        version: "1.0.228",
        license: "MIT OR Apache-2.0",
        authors: vec!["Erick Tryzelaar <erick.tryzelaar@gmail.com>", "David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "serde_json",
        version: "1.0.149",
        license: "MIT OR Apache-2.0",
        authors: vec!["Erick Tryzelaar <erick.tryzelaar@gmail.com>", "David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "serde_repr",
        version: "0.1.20",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "serde_yaml",
        version: "0.9.34+deprecated",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "shlex",
        version: "1.3.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["comex <comexk@gmail.com>", "Fenhl <fenhl@fenhl.net>", "Adrian Taylor <adetaylor@chromium.org>", "Alex Touchet <alextouchet@outlook.com>", "Daniel Parks <dp+git@oxidized.org>", "Garrett Berg <googberg@gmail.com>"] 
    },
    LicenseInfo {
        name: "signal-hook-registry",
        version: "1.4.8",
        license: "MIT OR Apache-2.0",
        authors: vec!["Michal 'vorner' Vaner <vorner@vorner.cz>", "Masaki Hara <ackie.h.gmai@gmail.com>"] 
    },
    LicenseInfo {
        name: "skrifa",
        version: "0.37.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "slab",
        version: "0.4.12",
        license: "MIT",
        authors: vec!["Carl Lerche <me@carllerche.com>"] 
    },
    LicenseInfo {
        name: "slotmap",
        version: "1.1.1",
        license: "Zlib",
        authors: vec!["Orson Peters <orsonpeters@gmail.com>"] 
    },
    LicenseInfo {
        name: "smallvec",
        version: "1.15.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "smithay-client-toolkit",
        version: "0.19.2",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>", "i509VCB <mail@i509.me>", "Ashley Wulber <ashley@system76.com>"] 
    },
    LicenseInfo {
        name: "smithay-client-toolkit",
        version: "0.20.0",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>", "i509VCB <mail@i509.me>", "Ashley Wulber <ashley@system76.com>"] 
    },
    LicenseInfo {
        name: "smithay-clipboard",
        version: "0.7.3",
        license: "MIT",
        authors: vec!["Kirill Chibisov <contact@kchibisov.com>", "Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "smol_str",
        version: "0.2.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Aleksey Kladov <aleksey.kladov@gmail.com>"] 
    },
    LicenseInfo {
        name: "socket2",
        version: "0.6.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>", "Thomas de Zeeuw <thomasdezeeuw@gmail.com>"] 
    },
    LicenseInfo {
        name: "softbuffer",
        version: "0.4.8",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "spirv",
        version: "0.3.0+sdk-1.3.268.0",
        license: "Apache-2.0",
        authors: vec!["Lei Zhang <antiagainst@gmail.com>"] 
    },
    LicenseInfo {
        name: "stable_deref_trait",
        version: "1.2.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Robert Grosse <n210241048576@gmail.com>"] 
    },
    LicenseInfo {
        name: "static_assertions",
        version: "1.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Nikolai Vazquez"] 
    },
    LicenseInfo {
        name: "strict-num",
        version: "0.1.1",
        license: "MIT",
        authors: vec!["Yevhenii Reizner <razrfalcon@gmail.com>"] 
    },
    LicenseInfo {
        name: "strsim",
        version: "0.11.1",
        license: "MIT",
        authors: vec!["Danny Guo <danny@dannyguo.com>", "maxbachmann <oss@maxbachmann.de>"] 
    },
    LicenseInfo {
        name: "subtle",
        version: "2.6.1",
        license: "BSD-3-Clause",
        authors: vec!["Isis Lovecruft <isis@patternsinthevoid.net>", "Henry de Valence <hdevalence@hdevalence.ca>"] 
    },
    LicenseInfo {
        name: "svg_fmt",
        version: "0.4.5",
        license: "MIT/Apache-2.0",
        authors: vec!["Nicolas Silva <nical@fastmail.com>"] 
    },
    LicenseInfo {
        name: "swash",
        version: "0.2.6",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Chad Brokaw <cbrokaw@gmail.com>"] 
    },
    LicenseInfo {
        name: "syn",
        version: "2.0.116",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "sync_wrapper",
        version: "1.0.2",
        license: "Apache-2.0",
        authors: vec!["Actyx AG <developer@actyx.io>"] 
    },
    LicenseInfo {
        name: "synstructure",
        version: "0.13.2",
        license: "MIT",
        authors: vec!["Nika Layzell <nika@thelayzells.com>"] 
    },
    LicenseInfo {
        name: "sys-locale",
        version: "0.3.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["1Password"] 
    },
    LicenseInfo {
        name: "system-configuration",
        version: "0.7.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Mullvad VPN"] 
    },
    LicenseInfo {
        name: "system-configuration-sys",
        version: "0.6.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Mullvad VPN"] 
    },
    LicenseInfo {
        name: "tempfile",
        version: "3.25.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Steven Allen <steven@stebalien.com>", "The Rust Project Developers", "Ashley Mannix <ashleymannix@live.com.au>", "Jason White <me@jasonwhite.io>"] 
    },
    LicenseInfo {
        name: "termcolor",
        version: "1.4.1",
        license: "Unlicense OR MIT",
        authors: vec!["Andrew Gallant <jamslam@gmail.com>"] 
    },
    LicenseInfo {
        name: "thiserror",
        version: "1.0.69",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "thiserror",
        version: "2.0.18",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "thiserror-impl",
        version: "1.0.69",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "thiserror-impl",
        version: "2.0.18",
        license: "MIT OR Apache-2.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "thread-id",
        version: "5.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Ruud van Asseldonk <dev@veniogames.com>"] 
    },
    LicenseInfo {
        name: "tiny-skia",
        version: "0.11.4",
        license: "BSD-3-Clause",
        authors: vec!["Yevhenii Reizner <razrfalcon@gmail.com>"] 
    },
    LicenseInfo {
        name: "tiny-skia-path",
        version: "0.11.4",
        license: "BSD-3-Clause",
        authors: vec!["Yevhenii Reizner <razrfalcon@gmail.com>"] 
    },
    LicenseInfo {
        name: "tiny-xlib",
        version: "0.2.4",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["John Nunley <dev@notgull.net>"] 
    },
    LicenseInfo {
        name: "tiny_http",
        version: "0.12.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["pierre.krieger1708@gmail.com", "Corey Farwell <coreyf@rwell.org>"] 
    },
    LicenseInfo {
        name: "tinystr",
        version: "0.8.2",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "tinyvec",
        version: "1.10.0",
        license: "Zlib OR Apache-2.0 OR MIT",
        authors: vec!["Lokathor <zefria@gmail.com>"] 
    },
    LicenseInfo {
        name: "tinyvec_macros",
        version: "0.1.1",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["Soveu <marx.tomasz@gmail.com>"] 
    },
    LicenseInfo {
        name: "tokio",
        version: "1.49.0",
        license: "MIT",
        authors: vec!["Tokio Contributors <team@tokio.rs>"] 
    },
    LicenseInfo {
        name: "tokio-native-tls",
        version: "0.3.1",
        license: "MIT",
        authors: vec!["Tokio Contributors <team@tokio.rs>"] 
    },
    LicenseInfo {
        name: "tokio-rustls",
        version: "0.26.4",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "tokio-util",
        version: "0.7.18",
        license: "MIT",
        authors: vec!["Tokio Contributors <team@tokio.rs>"] 
    },
    LicenseInfo {
        name: "toml_datetime",
        version: "0.7.5+spec-1.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "toml_edit",
        version: "0.23.10+spec-1.0.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "toml_parser",
        version: "1.0.9+spec-1.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "tower",
        version: "0.5.3",
        license: "MIT",
        authors: vec!["Tower Maintainers <team@tower-rs.com>"] 
    },
    LicenseInfo {
        name: "tower-http",
        version: "0.6.8",
        license: "MIT",
        authors: vec!["Tower Maintainers <team@tower-rs.com>"] 
    },
    LicenseInfo {
        name: "tower-layer",
        version: "0.3.3",
        license: "MIT",
        authors: vec!["Tower Maintainers <team@tower-rs.com>"] 
    },
    LicenseInfo {
        name: "tower-service",
        version: "0.3.3",
        license: "MIT",
        authors: vec!["Tower Maintainers <team@tower-rs.com>"] 
    },
    LicenseInfo {
        name: "tracing",
        version: "0.1.44",
        license: "MIT",
        authors: vec!["Eliza Weisman <eliza@buoyant.io>", "Tokio Contributors <team@tokio.rs>"] 
    },
    LicenseInfo {
        name: "tracing-attributes",
        version: "0.1.31",
        license: "MIT",
        authors: vec!["Tokio Contributors <team@tokio.rs>", "Eliza Weisman <eliza@buoyant.io>", "David Barsky <dbarsky@amazon.com>"] 
    },
    LicenseInfo {
        name: "tracing-core",
        version: "0.1.36",
        license: "MIT",
        authors: vec!["Tokio Contributors <team@tokio.rs>"] 
    },
    LicenseInfo {
        name: "try-lock",
        version: "0.2.5",
        license: "MIT",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "ttf-parser",
        version: "0.25.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["Caleb Maclennan <caleb@alerque.com>", "Laurenz Stampfl <laurenz.stampfl@gmail.com>", "Yevhenii Reizner <razrfalcon@gmail.com>", "خالد حسني (Khaled Hosny) <khaled@aliftype.com>"] 
    },
    LicenseInfo {
        name: "typemap-ors",
        version: "1.0.0",
        license: "MIT",
        authors: vec!["Jonathan Reem <jonathan.reem@gmail.com>", "Anton Whalley anton@venshare.com"] 
    },
    LicenseInfo {
        name: "uds_windows",
        version: "1.1.0",
        license: "MIT",
        authors: vec!["Azure IoT Edge Devs", "Harald Hoyer <harald@redhat.com>"] 
    },
    LicenseInfo {
        name: "unicode-bidi",
        version: "0.3.18",
        license: "MIT OR Apache-2.0",
        authors: vec!["The Servo Project Developers"] 
    },
    LicenseInfo {
        name: "unicode-ident",
        version: "1.0.24",
        license: "(MIT OR Apache-2.0) AND Unicode-3.0",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "unicode-linebreak",
        version: "0.1.5",
        license: "Apache-2.0",
        authors: vec!["Axel Forsman <axelsfor@gmail.com>"] 
    },
    LicenseInfo {
        name: "unicode-script",
        version: "0.5.8",
        license: "MIT OR Apache-2.0",
        authors: vec!["Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "unicode-segmentation",
        version: "1.12.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["kwantam <kwantam@gmail.com>", "Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "unicode-width",
        version: "0.2.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["kwantam <kwantam@gmail.com>", "Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "unicode-xid",
        version: "0.2.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["erick.tryzelaar <erick.tryzelaar@gmail.com>", "kwantam <kwantam@gmail.com>", "Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "unsafe-any-ors",
        version: "1.0.0",
        license: "MIT",
        authors: vec!["Jonathan Reem <jonathan.reem@gmail.com>", "anton whalley anton@venshare.com"] 
    },
    LicenseInfo {
        name: "unsafe-libyaml",
        version: "0.2.11",
        license: "MIT",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "untrusted",
        version: "0.9.0",
        license: "ISC",
        authors: vec!["Brian Smith <brian@briansmith.org>"] 
    },
    LicenseInfo {
        name: "url",
        version: "2.5.8",
        license: "MIT OR Apache-2.0",
        authors: vec!["The rust-url developers"] 
    },
    LicenseInfo {
        name: "utf8_iter",
        version: "1.0.4",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Henri Sivonen <hsivonen@hsivonen.fi>"] 
    },
    LicenseInfo {
        name: "uuid",
        version: "1.21.0",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Ashley Mannix<ashleymannix@live.com.au>", "Dylan DPC<dylan.dpc@gmail.com>", "Hunar Roop Kahlon<hunar.roop@gmail.com>"] 
    },
    LicenseInfo {
        name: "vcpkg",
        version: "0.2.15",
        license: "MIT/Apache-2.0",
        authors: vec!["Jim McGrath <jimmc2@gmail.com>"] 
    },
    LicenseInfo {
        name: "version_check",
        version: "0.9.5",
        license: "MIT/Apache-2.0",
        authors: vec!["Sergio Benitez <sb@sergio.bz>"] 
    },
    LicenseInfo {
        name: "walkdir",
        version: "2.5.0",
        license: "Unlicense/MIT",
        authors: vec!["Andrew Gallant <jamslam@gmail.com>"] 
    },
    LicenseInfo {
        name: "want",
        version: "0.3.1",
        license: "MIT",
        authors: vec!["Sean McArthur <sean@seanmonstar.com>"] 
    },
    LicenseInfo {
        name: "wasi",
        version: "0.11.1+wasi-snapshot-preview1",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["The Cranelift Project Developers"] 
    },
    LicenseInfo {
        name: "wasip2",
        version: "1.0.2+wasi-0.2.9",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "wasip3",
        version: "0.4.0+wasi-0.3.0-rc-2026-01-06",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "wasm-bindgen",
        version: "0.2.108",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "wasm-bindgen-futures",
        version: "0.4.58",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "wasm-bindgen-macro",
        version: "0.2.108",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "wasm-bindgen-macro-support",
        version: "0.2.108",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "wasm-bindgen-shared",
        version: "0.2.108",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "wasm-encoder",
        version: "0.244.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Nick Fitzgerald <fitzgen@gmail.com>"] 
    },
    LicenseInfo {
        name: "wasm-metadata",
        version: "0.244.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "wasmparser",
        version: "0.244.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Yury Delendik <ydelendik@mozilla.com>"] 
    },
    LicenseInfo {
        name: "wasmtimer",
        version: "0.4.3",
        license: "MIT",
        authors: vec!["WhizSid <whizsid@aol.com>", "Pierre Krieger <pierre.krieger1708@gmail.com>"] 
    },
    LicenseInfo {
        name: "wayland-backend",
        version: "0.3.12",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-client",
        version: "0.31.12",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-csd-frame",
        version: "0.3.0",
        license: "MIT",
        authors: vec!["Kirill Chibisov <contact@kchibisov.com>"] 
    },
    LicenseInfo {
        name: "wayland-cursor",
        version: "0.31.12",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-protocols",
        version: "0.32.10",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-protocols-experimental",
        version: "20250721.0.1",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-protocols-misc",
        version: "0.3.10",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-protocols-plasma",
        version: "0.3.10",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-protocols-wlr",
        version: "0.3.10",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-scanner",
        version: "0.31.8",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "wayland-sys",
        version: "0.31.8",
        license: "MIT",
        authors: vec!["Elinor Berger <elinor@safaradeg.net>"] 
    },
    LicenseInfo {
        name: "web-sys",
        version: "0.3.85",
        license: "MIT OR Apache-2.0",
        authors: vec!["The wasm-bindgen Developers"] 
    },
    LicenseInfo {
        name: "web-time",
        version: "1.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "webpki-root-certs",
        version: "1.0.6",
        license: "CDLA-Permissive-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "wgpu",
        version: "27.0.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "wgpu-core",
        version: "27.0.3",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "wgpu-core-deps-apple",
        version: "27.0.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "wgpu-core-deps-emscripten",
        version: "27.0.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "wgpu-core-deps-windows-linux-android",
        version: "27.0.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "wgpu-hal",
        version: "27.0.4",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "wgpu-types",
        version: "27.0.1",
        license: "MIT OR Apache-2.0",
        authors: vec!["gfx-rs developers"] 
    },
    LicenseInfo {
        name: "winapi",
        version: "0.3.9",
        license: "MIT/Apache-2.0",
        authors: vec!["Peter Atashian <retep998@gmail.com>"] 
    },
    LicenseInfo {
        name: "winapi-i686-pc-windows-gnu",
        version: "0.4.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Peter Atashian <retep998@gmail.com>"] 
    },
    LicenseInfo {
        name: "winapi-util",
        version: "0.1.11",
        license: "Unlicense OR MIT",
        authors: vec!["Andrew Gallant <jamslam@gmail.com>"] 
    },
    LicenseInfo {
        name: "winapi-x86_64-pc-windows-gnu",
        version: "0.4.0",
        license: "MIT/Apache-2.0",
        authors: vec!["Peter Atashian <retep998@gmail.com>"] 
    },
    LicenseInfo {
        name: "window_clipboard",
        version: "0.5.1",
        license: "MIT",
        authors: vec!["Héctor Ramón Jiménez <hector0193@gmail.com>"] 
    },
    LicenseInfo {
        name: "windows",
        version: "0.58.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows",
        version: "0.62.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-collections",
        version: "0.3.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-core",
        version: "0.58.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-core",
        version: "0.62.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-future",
        version: "0.3.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-implement",
        version: "0.58.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-implement",
        version: "0.60.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-interface",
        version: "0.58.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-interface",
        version: "0.59.3",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-link",
        version: "0.2.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-numerics",
        version: "0.3.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-registry",
        version: "0.6.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-result",
        version: "0.2.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-result",
        version: "0.4.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-strings",
        version: "0.1.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-strings",
        version: "0.5.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-sys",
        version: "0.45.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-sys",
        version: "0.52.0",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-sys",
        version: "0.60.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-sys",
        version: "0.61.2",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-targets",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-targets",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows-targets",
        version: "0.53.5",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows-threading",
        version: "0.2.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_aarch64_gnullvm",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_aarch64_gnullvm",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_aarch64_gnullvm",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_aarch64_msvc",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_aarch64_msvc",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_aarch64_msvc",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_i686_gnu",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_i686_gnu",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_i686_gnu",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_i686_gnullvm",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_i686_gnullvm",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_i686_msvc",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_i686_msvc",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_i686_msvc",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_x86_64_gnu",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_x86_64_gnu",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_x86_64_gnu",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_x86_64_gnullvm",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_x86_64_gnullvm",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_x86_64_gnullvm",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "windows_x86_64_msvc",
        version: "0.42.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_x86_64_msvc",
        version: "0.52.6",
        license: "MIT OR Apache-2.0",
        authors: vec!["Microsoft"] 
    },
    LicenseInfo {
        name: "windows_x86_64_msvc",
        version: "0.53.1",
        license: "MIT OR Apache-2.0",
        authors: vec![] 
    },
    LicenseInfo {
        name: "winit",
        version: "0.30.12",
        license: "Apache-2.0",
        authors: vec!["The winit contributors", "Pierre Krieger <pierre.krieger1708@gmail.com>"] 
    },
    LicenseInfo {
        name: "winnow",
        version: "0.7.14",
        license: "MIT",
        authors: vec![] 
    },
    LicenseInfo {
        name: "wit-bindgen",
        version: "0.51.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "wit-bindgen-core",
        version: "0.51.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "wit-bindgen-rust",
        version: "0.51.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "wit-bindgen-rust-macro",
        version: "0.51.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "wit-component",
        version: "0.244.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Peter Huene <peter@huene.dev>"] 
    },
    LicenseInfo {
        name: "wit-parser",
        version: "0.244.0",
        license: "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        authors: vec!["Alex Crichton <alex@alexcrichton.com>"] 
    },
    LicenseInfo {
        name: "writeable",
        version: "0.6.2",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "x11-dl",
        version: "2.21.0",
        license: "MIT",
        authors: vec!["daggerbot <daggerbot@gmail.com>", "Erle Pereira <erle@erlepereira.com>", "AltF02 <contact@altf2.dev>"] 
    },
    LicenseInfo {
        name: "x11rb",
        version: "0.13.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Uli Schlachter <psychon@znc.in>", "Eduardo Sánchez Muñoz <eduardosm-dev@e64.io>", "notgull <jtnunley01@gmail.com>"] 
    },
    LicenseInfo {
        name: "x11rb-protocol",
        version: "0.13.2",
        license: "MIT OR Apache-2.0",
        authors: vec!["Uli Schlachter <psychon@znc.in>", "Eduardo Sánchez Muñoz <eduardosm-dev@e64.io>", "notgull <jtnunley01@gmail.com>"] 
    },
    LicenseInfo {
        name: "xcursor",
        version: "0.3.10",
        license: "MIT",
        authors: vec!["Samuele Esposito"] 
    },
    LicenseInfo {
        name: "xkbcommon-dl",
        version: "0.4.2",
        license: "MIT",
        authors: vec!["Francesca Frangipane <francesca@comfysoft.net>"] 
    },
    LicenseInfo {
        name: "xkeysym",
        version: "0.2.1",
        license: "MIT OR Apache-2.0 OR Zlib",
        authors: vec!["John Nunley <jtnunley01@gmail.com>"] 
    },
    LicenseInfo {
        name: "xml-rs",
        version: "0.8.28",
        license: "MIT",
        authors: vec!["Vladimir Matveev <vmatveev@citrine.cc>"] 
    },
    LicenseInfo {
        name: "yazi",
        version: "0.2.1",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Chad Brokaw <cbrokaw@gmail.com>"] 
    },
    LicenseInfo {
        name: "yoke",
        version: "0.8.1",
        license: "Unicode-3.0",
        authors: vec!["Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "yoke-derive",
        version: "0.8.1",
        license: "Unicode-3.0",
        authors: vec!["Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "zbus",
        version: "5.13.2",
        license: "MIT",
        authors: vec!["Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "zbus_macros",
        version: "5.13.2",
        license: "MIT",
        authors: vec!["Marc-André Lureau <marcandre.lureau@redhat.com>", "Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "zbus_names",
        version: "4.3.1",
        license: "MIT",
        authors: vec!["Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "zeno",
        version: "0.3.3",
        license: "Apache-2.0 OR MIT",
        authors: vec!["Chad Brokaw <cbrokaw@gmail.com>"] 
    },
    LicenseInfo {
        name: "zerocopy",
        version: "0.8.39",
        license: "BSD-2-Clause OR Apache-2.0 OR MIT",
        authors: vec!["Joshua Liebow-Feeser <joshlf@google.com>", "Jack Wrenn <jswrenn@amazon.com>"] 
    },
    LicenseInfo {
        name: "zerocopy-derive",
        version: "0.8.39",
        license: "BSD-2-Clause OR Apache-2.0 OR MIT",
        authors: vec!["Joshua Liebow-Feeser <joshlf@google.com>", "Jack Wrenn <jswrenn@amazon.com>"] 
    },
    LicenseInfo {
        name: "zerofrom",
        version: "0.1.6",
        license: "Unicode-3.0",
        authors: vec!["Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "zerofrom-derive",
        version: "0.1.6",
        license: "Unicode-3.0",
        authors: vec!["Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "zeroize",
        version: "1.8.2",
        license: "Apache-2.0 OR MIT",
        authors: vec!["The RustCrypto Project Developers"] 
    },
    LicenseInfo {
        name: "zerotrie",
        version: "0.2.3",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "zerovec",
        version: "0.11.5",
        license: "Unicode-3.0",
        authors: vec!["The ICU4X Project Developers"] 
    },
    LicenseInfo {
        name: "zerovec-derive",
        version: "0.11.2",
        license: "Unicode-3.0",
        authors: vec!["Manish Goregaokar <manishsmail@gmail.com>"] 
    },
    LicenseInfo {
        name: "zmij",
        version: "1.0.21",
        license: "MIT",
        authors: vec!["David Tolnay <dtolnay@gmail.com>"] 
    },
    LicenseInfo {
        name: "zvariant",
        version: "5.9.2",
        license: "MIT",
        authors: vec!["Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "zvariant_derive",
        version: "5.9.2",
        license: "MIT",
        authors: vec!["Zeeshan Ali Khan <zeeshanak@gnome.org>"] 
    },
    LicenseInfo {
        name: "zvariant_utils",
        version: "3.3.0",
        license: "MIT",
        authors: vec!["Zeeshan Ali Khan <zeeshanak@gnome.org>", "turbocooler <turbocooler@cocaine.ninja>"] 
    },
];
}
#[derive(Debug)]
pub struct LicenseInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub authors: Vec<&'static str>,
}

