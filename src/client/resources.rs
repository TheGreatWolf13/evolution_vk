use crate::client::model::{ModelLoader, ModelManager};
use crate::client::texture::TextureManager;

pub struct ResourceManager {
    texture_manager: TextureManager,
    model_manager: ModelManager,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut model_loader = ModelLoader::new();
        model_loader.load_models();
        let textures = model_loader.gather_textures();
        let (texture_manager, textures) = TextureManager::new(textures);
        Self {
            model_manager: ModelManager::bake(model_loader, textures, &texture_manager),
            texture_manager,
        }
    }

    pub fn get_model_manager(&self) -> &ModelManager {
        &self.model_manager
    }

    pub fn get_texture_manager(&self) -> &TextureManager {
        &self.texture_manager
    }
}