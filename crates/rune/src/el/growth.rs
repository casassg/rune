//! Growth stacks and their rows: `El::growth`, `El::streaming` and `El::arrives_whole`. The
//! heights are tracked in `crate::growth` and stepped in the runtime.

use super::*;

impl El {
    /// Make this column a growth stack (see [`GrowthSpec`]). Required alongside
    /// [`El::virtualize`] on the same element.
    pub fn growth(mut self, spec: GrowthSpec) -> Self {
        self.cold_mut().growth = Some(Box::new(spec));
        self
    }
    /// Stacks its flowing children on whole points, as a browser virtualizer that rounds each
    /// row's measured height does (TanStack Virtual's `measureElement` returning
    /// `Math.round(borderBoxSize.blockSize)`, placing each row at the running total): each
    /// child's top rounds to a whole point and its box to its rounded height, so its rows' tops,
    /// the heights a virtual list stores and the list's extent are whole points, which are whole
    /// CSS px, at any scale. A child's content keeps its own layout from the child's top, so up
    /// to half a point of it hangs below its box or is left empty. The list's own box grows or
    /// shrinks by what the rounding adds up to; nothing after it in its parent moves, so a list
    /// that opts in should end its scroll content. Rows land whole only on a column with whole
    /// padding and gap: the shift runs down its children, so a row-direction list would add up
    /// its children's error. Off by default.
    pub fn whole_rows(mut self) -> Self {
        self.cold_mut().whole_rows = true;
        self
    }
    /// A growth-stack row that is streaming: slows near its target and reveals through the
    /// feathered edge.
    pub fn streaming(mut self, on: bool) -> Self {
        self.flags.set(ElFlags::STREAMING, on);
        self
    }
    /// A growth-stack row whose content arrives complete, like a sent message: its height
    /// changes glide on the growth spring alone, without the speed limits that pace streaming
    /// text, so a large insert and the pinned viewport following it move in one quick glide.
    pub fn arrives_whole(mut self) -> Self {
        self.flags.set(ElFlags::WHOLE, true);
        self
    }
    /// A growth-stack row that folds open and shut, like a disclosure: its height changes glide
    /// on its own `spring`, in place of the stack's, for the spring's own time however far
    /// they go, keeping their velocity when a change reverses one mid-glide. Only what shows
    /// moves: the part of a change below the window's bottom edge (where the row last drew)
    /// lands at once, before the glide when shutting and after it when opening. Hold the row's
    /// header with [`crate::Tx::keep_on_screen`] so its top stays put. Implies
    /// [`El::arrives_whole`].
    pub fn fold_glide(mut self, spring: rune_motion::Spring) -> Self {
        self.flags.set(ElFlags::WHOLE, true);
        self.cold_mut().glide = Some(crate::growth::FoldGlide { spring, feather: self.cold.glide.and_then(|g| g.feather) });
        self
    }
    /// How deep, in points, a [`El::fold_glide`] row's moving bottom edge feathers as it folds
    /// open or shut, in place of its stack's `StreamEdge::mask_max`. It deepens as the edge
    /// starts moving and thins as it lands, so nothing is feathered at rest. Call after
    /// `fold_glide`.
    pub fn fold_feather(mut self, pt: f32) -> Self {
        debug_assert!(self.cold.glide.is_some(), "fold_feather without fold_glide");
        if let Some(g) = self.cold.get_mut_if_some().and_then(|c| c.glide.as_mut()) {
            g.feather = Some(pt.max(0.0));
        }
        self
    }
}
