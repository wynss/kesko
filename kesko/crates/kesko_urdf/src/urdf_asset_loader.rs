use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};

#[derive(Debug, TypeUuid)]
#[uuid = "ed6c93a1-a100-4e1d-814e-e662def3064d"]
pub struct UrdfAsset {
    pub robot: urdf_rs::Robot,
}

#[derive(Default)]
pub struct UrdfAssetLoader;

impl AssetLoader for UrdfAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let robot = urdf_rs::read_from_string(std::str::from_utf8(bytes).unwrap()).unwrap();
            let asset = UrdfAsset { robot };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["urdf"]
    }
}
