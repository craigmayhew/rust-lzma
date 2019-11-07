extern crate pkg_config;

use pkg_config::probe_library;


fn main() {
	if probe_library("liblzma").is_ok() {
		return
	} else {
		panic!("Could not find liblzma using pkg-config")
	}
}
