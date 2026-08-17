//! Wrappers for Clay's transition (animation) API.

use crate::bindings::*;
use crate::Declaration;

/// Bit-flag set of element properties to animate. Combine with `|`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionProperty(pub Clay_TransitionProperty);

impl TransitionProperty {
    pub const NONE: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_NONE);
    pub const X: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_X);
    pub const Y: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_Y);
    pub const POSITION: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_POSITION);
    pub const WIDTH: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_WIDTH);
    pub const HEIGHT: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_HEIGHT);
    pub const DIMENSIONS: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_DIMENSIONS);
    pub const BOUNDING_BOX: Self =
        Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_BOUNDING_BOX);
    pub const BACKGROUND_COLOR: Self =
        Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_BACKGROUND_COLOR);
    pub const OVERLAY_COLOR: Self =
        Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_OVERLAY_COLOR);
    pub const CORNER_RADIUS: Self =
        Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_CORNER_RADIUS);
    pub const BORDER_COLOR: Self =
        Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_BORDER_COLOR);
    pub const BORDER_WIDTH: Self =
        Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_BORDER_WIDTH);
    pub const BORDER: Self = Self(Clay_TransitionProperty_CLAY_TRANSITION_PROPERTY_BORDER);
}

impl core::ops::BitOr for TransitionProperty {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Z-ordering of an element while it plays its exit transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitSiblingOrdering {
    UnderneathSiblings =
        Clay_ExitTransitionSiblingOrdering_CLAY_EXIT_TRANSITION_ORDERING_UNDERNEATH_SIBLINGS,
    NaturalOrder = Clay_ExitTransitionSiblingOrdering_CLAY_EXIT_TRANSITION_ORDERING_NATURAL_ORDER,
    AboveSiblings =
        Clay_ExitTransitionSiblingOrdering_CLAY_EXIT_TRANSITION_ORDERING_ABOVE_SIBLINGS,
}

/// Per-frame interpolation callback; return `true` when the transition finished.
/// Clay provides [`ease_out`] (`Clay_EaseOut`) as a ready-made handler.
pub type TransitionHandlerFn = unsafe extern "C" fn(Clay_TransitionCallbackArguments) -> bool;

/// Enter/exit state modifier: receives the first/last-frame state and the
/// animated property flags; returns the adjusted state to start from / end at.
pub type TransitionStateFn =
    unsafe extern "C" fn(Clay_TransitionData, Clay_TransitionProperty) -> Clay_TransitionData;

/// Clay's built-in ease-out handler, usable as the default `handler`.
pub const EASE_OUT: TransitionHandlerFn = Clay_EaseOut;

/// Builder for a `Declaration`'s `.transition` config.
pub struct TransitionBuilder<
    'declaration,
    'render,
    ImageElementData: 'render,
    CustomElementData: 'render,
> {
    parent: &'declaration mut Declaration<'render, ImageElementData, CustomElementData>,
}

impl<'declaration, 'render, ImageElementData: 'render, CustomElementData: 'render>
    TransitionBuilder<'declaration, 'render, ImageElementData, CustomElementData>
{
    #[inline]
    pub fn new(
        parent: &'declaration mut Declaration<'render, ImageElementData, CustomElementData>,
    ) -> Self {
        TransitionBuilder { parent }
    }

    /// Sets the per-frame interpolation handler. Transitions only activate once
    /// a handler is set; see [`EASE_OUT`] or [`Self::ease_out`].
    #[inline]
    pub fn handler(&mut self, handler: TransitionHandlerFn) -> &mut Self {
        self.parent.inner.transition.handler = Some(handler);
        self
    }

    /// Uses Clay's built-in ease-out curve as the handler.
    #[inline]
    pub fn ease_out(&mut self) -> &mut Self {
        self.handler(EASE_OUT)
    }

    /// Transition duration in seconds.
    #[inline]
    pub fn duration(&mut self, seconds: f32) -> &mut Self {
        self.parent.inner.transition.duration = seconds;
        self
    }

    /// Which properties to animate; combine [`TransitionProperty`] flags with `|`.
    #[inline]
    pub fn properties(&mut self, properties: TransitionProperty) -> &mut Self {
        self.parent.inner.transition.properties = properties.0;
        self
    }

    /// Allow hover/click interactions while the element is moving
    /// (disabled by default in Clay).
    #[inline]
    pub fn allow_interactions_while_transitioning(&mut self, allow: bool) -> &mut Self {
        self.parent.inner.transition.interactionHandling = if allow {
            Clay_TransitionInteractionHandlingType_CLAY_TRANSITION_ALLOW_INTERACTIONS_WHILE_TRANSITIONING_POSITION
        } else {
            Clay_TransitionInteractionHandlingType_CLAY_TRANSITION_DISABLE_INTERACTIONS_WHILE_TRANSITIONING_POSITION
        };
        self
    }

    /// Enables the enter transition by providing the initial-state modifier.
    #[inline]
    pub fn enter_state(&mut self, f: TransitionStateFn) -> &mut Self {
        self.parent.inner.transition.enter.setInitialState = Some(f);
        self
    }

    /// Whether the enter transition also triggers the first frame its parent appears.
    #[inline]
    pub fn enter_on_first_parent_frame(&mut self, trigger: bool) -> &mut Self {
        self.parent.inner.transition.enter.trigger = if trigger {
            Clay_TransitionEnterTriggerType_CLAY_TRANSITION_ENTER_TRIGGER_ON_FIRST_PARENT_FRAME
        } else {
            Clay_TransitionEnterTriggerType_CLAY_TRANSITION_ENTER_SKIP_ON_FIRST_PARENT_FRAME
        };
        self
    }

    /// Enables the exit transition by providing the final-state modifier.
    #[inline]
    pub fn exit_state(&mut self, f: TransitionStateFn) -> &mut Self {
        self.parent.inner.transition.exit.setFinalState = Some(f);
        self
    }

    /// Whether the exit transition also triggers the frame its parent exits.
    #[inline]
    pub fn exit_when_parent_exits(&mut self, trigger: bool) -> &mut Self {
        self.parent.inner.transition.exit.trigger = if trigger {
            Clay_TransitionExitTriggerType_CLAY_TRANSITION_EXIT_TRIGGER_WHEN_PARENT_EXITS
        } else {
            Clay_TransitionExitTriggerType_CLAY_TRANSITION_EXIT_SKIP_WHEN_PARENT_EXITS
        };
        self
    }

    /// Z-ordering of the element relative to siblings during its exit transition.
    #[inline]
    pub fn exit_sibling_ordering(&mut self, ordering: ExitSiblingOrdering) -> &mut Self {
        self.parent.inner.transition.exit.siblingOrdering = ordering as _;
        self
    }

    /// Finalizes the transition config, returning the parent declaration.
    #[inline]
    pub fn end(
        &mut self,
    ) -> &mut Declaration<'render, ImageElementData, CustomElementData> {
        self.parent
    }
}
