use crate::{prelude::*, world::TRANSFORMS, gltf::component::{GltfResourceWrapper, GltfResourceWrapperView}};
use awsm_renderer::{gltf::{loader::{load_gltf, GltfResource}, component::GltfPrimitive}, light::Light};
use nalgebra_glm::Vec3;
use crate::light::add_demo_lights;

use super::component::GltfResourceWrapperViewMut;

pub async fn switch_gltf(renderer: Rc<RefCell<AwsmRenderer>>, world: Rc<RefCell<World>>, id: GltfId) -> Result<()> {
    renderer.borrow_mut().free_all(&mut *world.borrow_mut())?;

    let url = format!("{}/{}", CONFIG.gltf_url, id.filepath());
    let res = load_gltf(&url, None).await?;

    add_demo_lights(&mut world.borrow_mut(), id)?;

    renderer.borrow_mut().populate_gltf(&mut *world.borrow_mut(), &res, None)?;

    world.borrow_mut().run(move |mut wrapper: GltfResourceWrapperViewMut| {
        wrapper.0 = Some(res);
    });

    Ok(())

}



#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub x_min: f32,
    pub y_min: f32,
    pub z_min: f32,
    pub x_max: f32,
    pub y_max: f32,
    pub z_max: f32,
}

impl Bounds {
    fn default() -> Self {
        Self { 
            x_min: f32::MAX, 
            y_min: f32::MAX, 
            z_min: f32::MAX, 
            x_max: f32::MIN, 
            y_max: f32::MIN, 
            z_max: f32::MIN, 
        }
    }

    pub fn mid(&self) -> [f32;3] {
        [
            (self.x_min + self.x_max)/2.0,
            (self.y_min + self.y_max)/2.0,
            (self.z_min + self.z_max)/2.0,
        ]
    }

    pub fn update_min(&mut self, values: [f32;3]) {
        self.update_x_min(values[0]);
        self.update_y_min(values[1]);
        self.update_z_min(values[2]);
    }

    pub fn update_max(&mut self, values: [f32;3]) {
        self.update_x_max(values[0]);
        self.update_y_max(values[1]);
        self.update_z_max(values[2]);
    }

    pub fn update_x_min(&mut self, value: f32) {
        if value < self.x_min {
            self.x_min = value;
        }
    }

    pub fn update_x_max(&mut self, value: f32) {
        if value > self.x_max {
            self.x_max= value;
        }
    }

    pub fn update_y_min(&mut self, value: f32) {
        if value < self.y_min {
            self.y_min = value;
        }
    }

    pub fn update_y_max(&mut self, value: f32) {
        if value > self.y_max {
            self.y_max= value;
        }
    }

    pub fn update_z_min(&mut self, value: f32) {
        if value < self.z_min {
            self.z_min = value;
        }
    }

    pub fn update_z_max(&mut self, value: f32) {
        if value > self.z_max {
            self.z_max= value;
        }
    }
}
pub fn calculate_gltf_bounds(world: &mut World) -> Result<Option<Bounds>> {
    let mut bounds:Option<Bounds> = None;

    world.run_workload(TRANSFORMS)?;


    let (res, transforms, gltf_primitives) 
        = world.borrow::<(
            GltfResourceWrapperView,
            View<WorldTransform>,
            View<GltfPrimitive>,
        )>()?;

    match res.0.as_ref() {
        None => Ok(None),
        Some(res) => {
            for (transform, primitive) in (&transforms, &gltf_primitives).iter() {

                let x = transform[0];
                let y = transform[1];
                let z = transform[2];

                if let Some(mesh) = res.gltf.meshes().nth(primitive.mesh_index) {
                    if let Some(primitive) = mesh.primitives().nth(primitive.index) {
                        if bounds.is_none() {
                            bounds = Some(Bounds::default());
                        }
                        let b = bounds.as_mut().unwrap_ext();
                        let bounding_box = primitive.bounding_box();

                        b.update_min([
                            bounding_box.min[0],
                            bounding_box.min[1],
                            bounding_box.min[2],
                        ]);
                        b.update_max([
                            bounding_box.max[0],
                            bounding_box.max[1],
                            bounding_box.max[2]
                        ]);
                    } else {
                        log::warn!("no primitive #{} on mesh!", primitive.index);
                    }
                
                } else {
                    log::warn!("no mesh #{}!", primitive.mesh_index);
                }
            }


            Ok(bounds)
        }
    }
}