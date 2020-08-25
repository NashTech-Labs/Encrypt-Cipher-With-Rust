use encrypt_cipher::encryption_technique::md4;
use encrypt_cipher::encryption_technique::md5;

fn main() {
    println!(
        "The encrypted cipher from MD4 algorithm: {}",
        md4::encrypt_cipher(&md4::md4("Pankaj Chaudhary"))
    );

    println!(
        "The encrypted cipher from MD5 algorithm: {}",
        md5::encrypt_cipher("Pankaj Chaudhary")
    );
}
