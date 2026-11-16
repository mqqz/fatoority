//! Invoice flags with a crate-owned public surface.
use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct FlagBits: u8 {
        const THIRD_PARTY = 0b00001;
        const NOMINAL = 0b00010;
        const EXPORT = 0b00100;
        const SUMMARY = 0b01000;
        const SELF_BILLED = 0b10000;
    }
}

/// Invoice boolean flags packed into a bitset.
///
/// ```
/// use fatoora_core::invoice::InvoiceFlags;
/// let flags = InvoiceFlags::EXPORT | InvoiceFlags::SELF_BILLED;
/// assert!(flags.contains(InvoiceFlags::EXPORT));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InvoiceFlags(FlagBits);

impl InvoiceFlags {
    /// Invoice issued by a third party on behalf of the seller.
    pub const THIRD_PARTY: Self = Self(FlagBits::THIRD_PARTY);
    /// Nominal supply (goods or services supplied for no or token consideration).
    pub const NOMINAL: Self = Self(FlagBits::NOMINAL);
    /// Export invoice.
    pub const EXPORT: Self = Self(FlagBits::EXPORT);
    /// Summary invoice covering multiple supplies.
    pub const SUMMARY: Self = Self(FlagBits::SUMMARY);
    /// Self-billed invoice issued by the buyer.
    pub const SELF_BILLED: Self = Self(FlagBits::SELF_BILLED);

    /// Returns a value with no flags set.
    pub const fn empty() -> Self {
        Self(FlagBits::empty())
    }
    /// Returns a value with every known flag set.
    pub const fn all() -> Self {
        Self(FlagBits::all())
    }
    /// Returns the raw bit representation.
    pub const fn bits(&self) -> u8 {
        self.0.bits()
    }
    /// Converts raw bits, returning `None` if any bit does not correspond to a known flag.
    pub const fn from_bits(bits: u8) -> Option<Self> {
        match FlagBits::from_bits(bits) {
            Some(flags) => Some(Self(flags)),
            None => None,
        }
    }
    /// Converts raw bits, silently dropping any unknown bits.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(FlagBits::from_bits_truncate(bits))
    }
    /// Converts raw bits, keeping any unknown bits as-is.
    pub const fn from_bits_retain(bits: u8) -> Self {
        Self(FlagBits::from_bits_retain(bits))
    }
    /// Looks up a single flag by its constant name (e.g. `"EXPORT"`), returning `None` if unknown.
    pub fn from_name(name: &str) -> Option<Self> {
        FlagBits::from_name(name).map(Self)
    }
    /// Returns `true` if no flags are set.
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns `true` if every known flag is set.
    pub const fn is_all(&self) -> bool {
        self.0.is_all()
    }
    /// Returns `true` if any flag in `other` is also set in `self`.
    pub const fn intersects(&self, other: Self) -> bool {
        self.0.intersects(other.0)
    }
    /// Returns `true` if every flag in `other` is also set in `self`.
    pub const fn contains(&self, other: Self) -> bool {
        self.0.contains(other.0)
    }
    /// Sets the flags in `other`.
    pub fn insert(&mut self, other: Self) {
        self.0.insert(other.0);
    }
    /// Clears the flags in `other`.
    pub fn remove(&mut self, other: Self) {
        self.0.remove(other.0);
    }
    /// Flips the flags in `other`.
    pub fn toggle(&mut self, other: Self) {
        self.0.toggle(other.0);
    }
    /// Sets the flags in `other` if `value` is `true`, otherwise clears them.
    pub fn set(&mut self, other: Self, value: bool) {
        self.0.set(other.0, value);
    }
    /// Returns the flags set in both `self` and `other`.
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0.intersection(other.0))
    }
    /// Returns the flags set in either `self` or `other`.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }
    /// Returns the flags set in `self` but not in `other`.
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0.difference(other.0))
    }
    /// Returns the flags set in exactly one of `self` and `other`.
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0.symmetric_difference(other.0))
    }
    /// Returns the known flags that are not set in `self` (unknown bits are cleared).
    pub const fn complement(self) -> Self {
        Self(self.0.complement())
    }
    /// Iterates over the set flags as individual [`InvoiceFlags`] values.
    pub const fn iter(&self) -> InvoiceFlagsIter {
        InvoiceFlagsIter(self.0.iter())
    }
    /// Iterates over the set flags as `(name, flag)` pairs.
    pub const fn iter_names(&self) -> InvoiceFlagNames {
        InvoiceFlagNames(self.0.iter_names())
    }
}

/// Iterator over set flags; any unknown bits are returned together at the end.
pub struct InvoiceFlagsIter(bitflags::iter::Iter<FlagBits>);
impl Iterator for InvoiceFlagsIter {
    type Item = InvoiceFlags;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(InvoiceFlags)
    }
}
impl std::iter::FusedIterator for InvoiceFlagsIter {}

/// Iterator over names and values of known set flags.
pub struct InvoiceFlagNames(bitflags::iter::IterNames<FlagBits>);
impl Iterator for InvoiceFlagNames {
    type Item = (&'static str, InvoiceFlags);
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(name, flags)| (name, InvoiceFlags(flags)))
    }
}
impl std::iter::FusedIterator for InvoiceFlagNames {}

impl IntoIterator for InvoiceFlags {
    type Item = Self;
    type IntoIter = InvoiceFlagsIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl Extend<InvoiceFlags> for InvoiceFlags {
    fn extend<T: IntoIterator<Item = Self>>(&mut self, values: T) {
        for value in values {
            self.insert(value);
        }
    }
}
impl FromIterator<InvoiceFlags> for InvoiceFlags {
    fn from_iter<T: IntoIterator<Item = Self>>(values: T) -> Self {
        let mut flags = Self::empty();
        flags.extend(values);
        flags
    }
}
impl std::ops::BitOr for InvoiceFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitOrAssign for InvoiceFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl std::ops::BitAnd for InvoiceFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitAndAssign for InvoiceFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl std::ops::BitXor for InvoiceFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::BitXorAssign for InvoiceFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl std::ops::Sub for InvoiceFlags {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl std::ops::SubAssign for InvoiceFlags {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl std::ops::Not for InvoiceFlags {
    type Output = Self;
    fn not(self) -> Self {
        self.complement()
    }
}

impl std::fmt::Binary for InvoiceFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, formatter)
    }
}

impl std::fmt::Octal for InvoiceFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.0, formatter)
    }
}

impl std::fmt::LowerHex for InvoiceFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, formatter)
    }
}

impl std::fmt::UpperHex for InvoiceFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::InvoiceFlags as F;

    #[test]
    fn unknown_bits_survive_iteration_but_have_no_names() {
        assert!(F::from_bits(0x80).is_none());
        assert!(F::from_bits_truncate(0x80).is_empty());
        let flags = F::from_bits_retain(0x85);
        assert_eq!(flags.bits(), 0x85);
        let mut values = flags.into_iter();
        assert_eq!(values.next(), Some(F::THIRD_PARTY));
        assert_eq!(values.next(), Some(F::EXPORT));
        assert_eq!(values.next(), Some(F::from_bits_retain(0x80)));
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(flags.iter().collect::<F>(), flags);
        let mut names = flags.iter_names();
        assert_eq!(names.next(), Some(("THIRD_PARTY", F::THIRD_PARTY)));
        assert_eq!(names.next(), Some(("EXPORT", F::EXPORT)));
        assert_eq!(names.next(), None);
        assert_eq!(names.next(), None);
        assert_eq!(F::from_name("EXPORT"), Some(F::EXPORT));
        assert_eq!(F::from_name("export"), None);
        assert_eq!(
            format!("{flags:b} {flags:o} {flags:x} {flags:X}"),
            "10000101 205 85 85"
        );
    }

    #[test]
    fn flag_updates_preserve_unrelated_capabilities() {
        let mut flags = F::empty();
        flags.extend([F::EXPORT, F::SELF_BILLED, F::EXPORT]);
        assert_eq!(flags.bits(), 20);
        flags.set(F::SUMMARY, true);
        flags.set(F::EXPORT, false);
        flags.toggle(F::NOMINAL);
        flags.remove(F::SELF_BILLED);
        assert_eq!(flags.bits(), 10);
        flags |= F::THIRD_PARTY;
        flags &= F::THIRD_PARTY | F::SUMMARY;
        flags ^= F::SELF_BILLED;
        flags -= F::SUMMARY;
        assert_eq!(flags.bits(), 17);
        assert!(flags.intersects(F::SELF_BILLED | F::NOMINAL));
        assert!(!flags.contains(F::SELF_BILLED | F::NOMINAL));
        assert!(F::all().is_all());
        assert_eq!(F::all().bits(), 31);
    }

    #[test]
    fn set_operations_match_independent_membership_model() {
        for left in 0..32 {
            for right in 0..32 {
                let a = F::from_bits(left).unwrap();
                let b = F::from_bits(right).unwrap();
                let expected = |predicate: fn(bool, bool) -> bool| {
                    (0..5)
                        .filter(|bit| {
                            predicate(left / (1 << bit) % 2 == 1, right / (1 << bit) % 2 == 1)
                        })
                        .map(|bit| 1 << bit)
                        .sum::<u8>()
                };
                let union = expected(|a, b| a || b);
                let intersection = expected(|a, b| a && b);
                let difference = expected(|a, b| a && !b);
                let symmetric = expected(|a, b| a != b);
                assert_eq!((a | b).bits(), union);
                assert_eq!(a.union(b).bits(), union);
                assert_eq!((a & b).bits(), intersection);
                assert_eq!(a.intersection(b).bits(), intersection);
                assert_eq!((a - b).bits(), difference);
                assert_eq!(a.difference(b).bits(), difference);
                assert_eq!((a ^ b).bits(), symmetric);
                assert_eq!(a.symmetric_difference(b).bits(), symmetric);
                assert_eq!((!a).bits(), 31 - left);
                assert_eq!(a.complement().bits(), 31 - left);
            }
        }
    }
}
