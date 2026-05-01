pub mod writer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    IntLiteral,
    EndOfFile,
}

impl From<u8> for TokenKind {
    fn from(kind: u8) -> Self {
        match kind {
            0 => TokenKind::Whitespace,
            1 => TokenKind::IntLiteral,
            2 => TokenKind::EndOfFile,
            _ => unreachable!("should only ever have valid a token kind"),
        }
    }
}

/// A 32-bit integer that provides information about a given
/// token. See [`TokenizedBuffer`] for specific instantiations
/// for a token.
///
/// The structure of a `TokenInfo` is diagrammed below:
///
/// -----------------------------
/// |  kind  |     payload     |
/// ----------------------------
/// 0      7 8                32
///
/// - `kind`: the type of the token this information is tied to.
/// - `payload`:
///   a type-dependent value, although it is typically a lightweight
///   handle into a separate value store.
#[derive(Clone, Copy)]
pub struct TokenInfo(u32);

impl TokenInfo {
    pub(super) fn new(kind: TokenKind, payload: u32) -> Self {
        debug_assert!(payload < (1 << 24), "payload exceeds 24 bits");
        let masked = payload & 0x00FF_FFFF;
        Self((kind as u32) << 24 | masked)
    }

    /// Returns the kind of this token.
    ///
    /// If this token kind is not valid, this function
    /// will panic.
    #[inline]
    pub fn kind(self) -> TokenKind {
        TokenKind::from((self.0 >> 24) as u8)
    }

    pub fn int_id(self) -> IntId {
        debug_assert_eq!(self.kind(), TokenKind::IntLiteral);
        IntId(self.payload())
    }

    #[inline]
    pub(crate) fn payload(self) -> u32 {
        self.0 & 0x00FF_FFFF
    }
}

impl std::fmt::Debug for TokenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenInfo")
            .field("kind", &self.kind())
            .field("payload", &self.payload())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntId(pub(crate) u32);
