use crate::prelude::*;
use awsm_renderer::gltf::loader::load_gltf;

pub async fn switch_gltf(renderer: Rc<RefCell<AwsmRenderer>>, world: Rc<RefCell<World>>, id: GltfId) -> Result<()> {
    renderer.borrow_mut().free_all(&mut *world.borrow_mut())?;

    let url = format!("{}/{}", CONFIG.gltf_url, id.filepath());
    let res = load_gltf(&url, None).await?;


    renderer.borrow_mut().populate_gltf(&mut *world.borrow_mut(), &res, None)?;

    Ok(())
}
