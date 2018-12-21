use std::path::Path;

use clap::{App, Arg};
use pelite::resources::version_info::VersionInfo;
use pelite::resources::FindError;
use pelite::FileMap;

mod error;

use crate::error::Error;

fn main() {
    let matches = App::new("version-check")
        .arg(
            Arg::with_name("EXECUTABLE")
                .help("A .exe or .dll file.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("EXECUTABLE").unwrap();
    let path = Path::new(&file);
    read_version(&path).expect("Failed to read executable versions.");
}

fn read_version(file_path: &Path) -> Result<(), Error> {
    let file_map =
        FileMap::open(file_path).map_err(|e| Error::IoError(file_path.to_path_buf(), e))?;
    let version_info = get_pe_version_info(file_map.as_ref())
        .map_err(|e| Error::PeParsingError(file_path.to_path_buf(), e.into()))?;

    println!("{:?}", version_info.fixed());
    println!("{:?}", version_info.to_hash_map());

    Ok(())
}

fn get_pe_version_info(bytes: &[u8]) -> Result<VersionInfo, FindError> {
    use pelite;
    use pelite::pe64;
    match pe64::PeFile::from_bytes(bytes) {
        Ok(file) => {
            use pelite::pe64::Pe;

            file.resources()?.version_info()
        }
        Err(pelite::Error::PeMagic) => {
            use pelite::pe32::{Pe, PeFile};

            PeFile::from_bytes(bytes)?.resources()?.version_info()
        }
        Err(e) => Err(e.into()),
    }
}
