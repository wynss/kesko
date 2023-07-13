use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use roxmltree::Document;

#[derive(Debug, TypeUuid)]
#[uuid = "e32e7241-f6e6-489d-9ff5-8e8528019c2c"]
pub struct SdfAsset {
    pub name: String,
    pub visual_path: String,
}

#[derive(Default)]
pub struct SdfAssetLoader;

impl AssetLoader for SdfAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let xml_str = std::str::from_utf8(bytes).unwrap();
            let doc = Document::parse(&xml_str).unwrap();

            let name = doc
                .descendants()
                .find(|n| n.has_tag_name("model"))
                .and_then(|model| model.attribute("name"))
                .expect("No model name found")
                .to_string();

            let relative_visual_path = doc
                .descendants()
                .find(|n| n.has_tag_name("uri"))
                .and_then(|uri| uri.text())
                .expect("No visual path found")
                .to_string();
            let visual_path = load_context
                .path()
                .parent()
                .unwrap()
                .join(relative_visual_path)
                .to_str()
                .unwrap()
                .to_string();

            let asset = SdfAsset { visual_path, name };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sdf"]
    }
}
