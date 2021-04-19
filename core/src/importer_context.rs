use futures_core::future::BoxFuture;

use crate::{AssetRef, AssetUuid};

pub trait ImporterContextHandle: Send + Sync {
    fn scope(&self, f: Box<dyn FnOnce()>);

    fn begin_serialize_asset(&mut self, asset: AssetUuid);
    /// Returns any registered dependencies
    fn end_serialize_asset(&mut self, asset: AssetUuid) -> std::collections::HashSet<AssetRef>;
    /// Resolves an AssetRef to a specific AssetUuid
    fn resolve_ref(&mut self, asset_ref: &AssetRef, asset: AssetUuid);
}

pub trait ImporterContext: 'static + Send + Sync {
    fn handle(&self) -> Box<dyn ImporterContextHandle>;
}
