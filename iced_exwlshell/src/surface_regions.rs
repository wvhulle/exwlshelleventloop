use std::any::Any;

use exwlshellev::{InputRectangle, RegionAction, blur::BlurRegion};
use iced_core::{
    Rectangle,
    widget::{Id, Operation, operation::Outcome},
};

use crate::error::RegionActionCastError;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlurRegions(pub Vec<BlurRegion>);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputRegionActions(pub Vec<RegionAction>);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurfaceRegionsCollector {
    pub blur: Vec<BlurRegion>,
    pub input: Vec<RegionAction>,
}

pub trait IntoRegionAction {
    fn to_add(self) -> Result<RegionAction, RegionActionCastError>;
    fn to_subtract(self) -> Result<RegionAction, RegionActionCastError>;
}

impl IntoRegionAction for Rectangle {
    fn to_add(self) -> Result<RegionAction, RegionActionCastError> {
        if self.width < 0. {
            return Err(RegionActionCastError::IllegelWidth);
        }
        if self.height < 0. {
            return Err(RegionActionCastError::IllegelHeight);
        }
        Ok(RegionAction::Add(InputRectangle {
            x: self.x as _,
            y: self.y as _,
            width: self.width.ceil() as _,
            height: self.height.ceil() as _,
        }))
    }
    fn to_subtract(self) -> Result<RegionAction, RegionActionCastError> {
        if self.width < 0. {
            return Err(RegionActionCastError::IllegelWidth);
        }
        if self.height < 0. {
            return Err(RegionActionCastError::IllegelHeight);
        }
        Ok(RegionAction::Subtract(InputRectangle {
            x: self.x as _,
            y: self.y as _,
            width: self.width.ceil() as _,
            height: self.height.ceil() as _,
        }))
    }
}

impl SurfaceRegionsCollector {
    fn collect(&mut self, state: &mut dyn Any) {
        if let Some(BlurRegions(regions)) = state.downcast_ref::<BlurRegions>() {
            self.blur.extend_from_slice(regions);
        }
        if let Some(InputRegionActions(actions)) = state.downcast_ref::<InputRegionActions>() {
            self.input.extend(actions);
        }
    }
}

impl Operation for SurfaceRegionsCollector {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Any) {
        self.collect(state);
    }
}

impl Operation<SurfaceRegionsCollector> for SurfaceRegionsCollector {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<SurfaceRegionsCollector>)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Any) {
        self.collect(state);
    }

    fn finish(&self) -> Outcome<SurfaceRegionsCollector> {
        Outcome::Some(self.clone())
    }
}
