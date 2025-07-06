//ISSUE #15116 <https://github.com/rust-lang/rust-clippy/issues/15116> - C-bug, I-false-positive

#![warn(clippy::disallowed_script_idents)]
fn main() {
	const ÄÖÜ: u8 = 0;
	const _ÄÖÜ: u8 = 0;
	const Ä_ÖÜ: u8 = 0;
	const ÄÖ_Ü: u8 = 0;
	const ÄÖÜ_: u8 = 0;
	let äöüß = 1;
	let _äöüß = 1;
	let ä_öüß = 1;
	let äö_üß = 1;
	let äöü_ß = 1;
	let äöüß_ = 1;
}
