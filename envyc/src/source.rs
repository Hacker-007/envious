use crate::{dense::DenseVec, lex::buffer::TokenizedBuffer};

#[derive(Debug, Clone, Copy)]
pub struct SourceId(u32);

#[derive(Debug, Default)]
pub struct SourceMap<'src> {
    sources: DenseVec<Source<'src>>,
}

impl<'src> SourceMap<'src> {
    pub(crate) fn register(&mut self, bytes: &'src [u8], buffer: TokenizedBuffer) -> SourceId {
        let idx = self.sources.push(Source::new(bytes, buffer));
        SourceId(idx)
    }

    pub fn get_mut(&mut self, id: SourceId) -> &mut Source<'src> {
        self.sources.get_mut(id.0)
    }

    pub fn buffer(&self, id: SourceId) -> &TokenizedBuffer {
        self.sources.get(id.0).buffer()
    }

    pub fn buffer_mut(&mut self, id: SourceId) -> &mut TokenizedBuffer {
        self.sources.get_mut(id.0).buffer_mut()
    }
}

/// A reference to the original source bytes
/// being compiled.
#[derive(Debug)]
pub struct Source<'src> {
    bytes: &'src [u8],
    newline_offsets: Vec<u32>,
    buffer: Box<TokenizedBuffer>,
}

impl<'src> Source<'src> {
    pub fn new(bytes: &'src [u8], buffer: TokenizedBuffer) -> Self {
        Self {
            bytes,
            newline_offsets: vec![],
            buffer: Box::new(buffer),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn get(&self, idx: usize) -> Option<u8> {
        self.bytes.get(idx).copied()
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    #[inline]
    pub fn buffer(&self) -> &TokenizedBuffer {
        &self.buffer
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut TokenizedBuffer {
        &mut self.buffer
    }

    pub(crate) fn record_newline(&mut self, position: u32) {
        self.newline_offsets.push(position);
    }
}
