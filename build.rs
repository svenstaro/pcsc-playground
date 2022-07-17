use std::env;
use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let body =
        ureq::get("https://github.com/LudovicRousseau/pcsc-tools/raw/master/smartcard_list.txt")
            .call()?
            .into_string()?;

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(out_dir + "/smartcard_list.txt", body)?;

    Ok(())
}
