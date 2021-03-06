// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use alloc::string::String;

// GENERATOR-BEGIN: SymbolTestFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct SymbolTestFlags;
#[allow(dead_code)]
impl SymbolTestFlags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	/// Symbol is found
	pub(crate) const SYMBOL: u32 = 0x0000_0001;
	/// Signed symbol
	pub(crate) const SIGNED: u32 = 0x0000_0002;
	/// `options.MasmSymbolDisplInBrackets`
	pub(crate) const SYMBOL_DISPL_IN_BRACKETS: u32 = 0x0000_0004;
	/// `options.MasmDisplInBrackets`
	pub(crate) const DISPL_IN_BRACKETS: u32 = 0x0000_0008;
	/// `options.RipRelativeAddresses`
	pub(crate) const RIP: u32 = 0x0000_0010;
	/// `options.ShowZeroDisplacements`
	pub(crate) const SHOW_ZERO_DISPLACEMENTS: u32 = 0x0000_0020;
	/// `!options.MasmAddDsPrefix32`
	pub(crate) const NO_ADD_DS_PREFIX32: u32 = 0x0000_0040;
}
// GENERATOR-END: SymbolTestFlags

pub(super) struct SymbolOptionsTestCase {
	pub(super) hex_bytes: String,
	pub(super) bitness: u32,
	pub(super) ip: u64,
	pub(super) formatted_string: String,
	pub(super) flags: u32, // SymbolTestFlags
}
