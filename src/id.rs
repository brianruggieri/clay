use crate::bindings::*;

#[derive(Debug, Copy, Clone)]
pub struct Id {
    pub id: Clay_ElementId,
}

impl Id {
    /// Creates a clay id using the `label`. Matches C's `CLAY_ID`, so it also
    /// matches lookups via `Clay_GetElementId` / hit-test results.
    #[inline]
    pub(crate) fn new(label: &str) -> Id {
        let id = unsafe { Clay__HashString(label.into(), 0) };
        Id { id }
    }

    /// Creates a clay id using the `label` and the `index` (C's `CLAY_IDI`).
    /// Note: like in C, `new_index(label, 0)` hashes differently from `new(label)`.
    #[inline]
    pub(crate) fn new_index(label: &str, index: u32) -> Id {
        let id = unsafe { Clay__HashStringWithOffset(label.into(), index, 0) };
        Id { id }
    }

    /// Parent-scoped id (C's `CLAY_ID_LOCAL`).
    #[inline]
    pub(crate) fn new_local(label: &str) -> Id {
        let id = unsafe { Clay__HashString(label.into(), Clay_GetOpenElementId()) };
        Id { id }
    }

    /// Parent-scoped indexed id (C's `CLAY_IDI_LOCAL`).
    #[inline]
    pub(crate) fn new_index_local(label: &str, index: u32) -> Id {
        let id = unsafe {
            Clay__HashStringWithOffset(label.into(), index, Clay_GetOpenElementId())
        };
        Id { id }
    }
}
