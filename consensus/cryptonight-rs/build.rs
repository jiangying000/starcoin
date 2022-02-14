use cc::Build;

fn main() {
    Build::new()
        .include("ext/")
        .file("ext/aesb.c")
        .file("ext/c_blake256.c")
        .file("ext/c_groestl.c")
        .file("ext/c_jh.c")
        .file("ext/c_keccak.c")
        .file("ext/c_skein.c")
        .file("ext/cryptonight.c")
        .file("ext/hash-extra-blake.c")
        .file("ext/hash-extra-groestl.c")
        .file("ext/hash-extra-skein.c")
        .file("ext/hash-extra-jh.c")
        .file("ext/hash.c")
        .file("ext/oaes_lib.c")
        .file("ext/slow-hash.c")
        .flag("-maes")
        .flag("-msse2")
        .flag("-Ofast")
        .flag("-fexceptions")
        .compile("cryptonight");
}
