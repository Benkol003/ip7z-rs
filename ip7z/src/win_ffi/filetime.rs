/// [`FILETIME`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
/// struct.
///
/// Can be converted to [`SYSTEMTIME`](crate::SYSTEMTIME) with
/// [`FileTimeToSystemTime`](crate::FileTimeToSystemTime) function.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FILETIME {
	pub dwLowDateTime: u32,
	pub dwHighDateTime: u32,
}

impl FILETIME {
	pub const fn to_u64(self) -> u64 {
		(self.dwHighDateTime as u64) << 32 | (self.dwLowDateTime as u64)
	}

	pub const fn from_u64(v: u64) -> Self {
		Self {
			dwHighDateTime: (v>>32) as u32,
			dwLowDateTime: v as u32
		}
	}

}

impl From<u64> for FILETIME {
	fn from(v: u64) -> Self {
		Self {
			dwLowDateTime: v as u32,
			dwHighDateTime: (v>>32) as u32,
		}
	}
}

impl From<FILETIME> for u64 {
	fn from(v: FILETIME) -> Self {
		((v.dwHighDateTime as u64) << 32) | (v.dwLowDateTime as u64)
	}
}

impl FILETIME {
	/// Returns a new `FILETIME` with the milliseconds difference.
	#[must_use]
	pub const fn add_ms(self, ms: i64) -> Self {
		let self64 =  self.to_u64() as i64;
		let new_self64 = self64 + (ms * 10_000);
		Self::from_u64(new_self64 as u64)
	}

	/// Returns a new `FILETIME` with the seconds difference.
	#[must_use]
	pub const fn add_secs(self, secs: i64) -> Self {
		self.add_ms(secs * 1000)
	}

	/// Returns a new `FILETIME` with the minutes difference.
	#[must_use]
	pub const fn add_mins(self, mins: i64) -> Self {
		self.add_secs(mins * 60)
	}

	/// Returns a new `FILETIME` with the hours difference.
	#[must_use]
	pub const fn add_hours(self, hours: i64) -> Self {
		self.add_mins(hours * 60)
	}

	/// Returns a new `FILETIME` with the days difference.
	#[must_use]
	pub const fn add_days(self, days: i64) -> Self {
		self.add_hours(days * 24)
	}
}