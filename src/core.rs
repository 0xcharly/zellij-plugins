use std::{iter::Map, ops::BitOr};

pub(crate) enum FrameBuffer {
    NoUpdates,
    MarkDirty,
}

pub(crate) type EventLoopResult = anyhow::Result<FrameBuffer>;

impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer::NoUpdates
    }
}

impl FrameBuffer {
    pub fn as_bool(&self) -> bool {
        match self {
            FrameBuffer::MarkDirty => true,
            FrameBuffer::NoUpdates => false,
        }
    }
}

/// A trait for utility functions on iterators of [Result].
pub(crate) trait ResultIterator: Iterator {
    /// An iterator method that reduces [Result]s as long as they represent a successful value,
    /// producing a single, final value.
    ///
    /// The reducing closure either returns successfully, with the value that the accumulator
    /// should have for the next iteration, or it returns failure, with an error value that is
    /// propagated back to the caller immediately (short-circuiting).
    ///
    /// If the iterator is empty, returns `Ok(Default::default())`.
    fn try_consume(self: &mut Self) -> EventLoopResult
    where
        Self: Sized;
}

impl From<bool> for FrameBuffer {
    fn from(value: bool) -> Self {
        match value {
            true => Self::MarkDirty,
            false => Self::NoUpdates,
        }
    }
}

impl BitOr for FrameBuffer {
    type Output = FrameBuffer;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self.as_bool() | rhs.as_bool()).into()
    }
}

impl BitOr<EventLoopResult> for FrameBuffer {
    type Output = EventLoopResult;

    fn bitor(self, rhs: EventLoopResult) -> Self::Output {
        match rhs {
            Ok(value) => Ok(self | value),
            err => err,
        }
    }
}

impl<I: Iterator, F> ResultIterator for Map<I, F>
where
    F: FnMut(I::Item) -> EventLoopResult,
{
    fn try_consume(self: &mut Self) -> EventLoopResult {
        self.try_fold(Default::default(), std::ops::BitOr::bitor)
    }
}
