// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts vertical tab (`VT`, `\v`) delay.
///
/// Defaults to zero.
///
/// The value of one is only supported on Android, ?Fuschia, iOS, Linux and macos.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum VerticalTabDelay
{
	/// Zero.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] Zero = VT0,
	#[cfg(not(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos")))] Zero = 0,

	/// One.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] One = VT1,
}

impl Into<tcflag_t> for VerticalTabDelay
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl Default for VerticalTabDelay
{
	#[inline(always)]
	fn default() -> Self
	{
		VerticalTabDelay::Zero
	}
}

impl MultipleBits for VerticalTabDelay
{
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] const Bitmask: tcflag_t = VTDLY;

	#[inline(always)]
	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(clean_mode_flags) }
	}
}
