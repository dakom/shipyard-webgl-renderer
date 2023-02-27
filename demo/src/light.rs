use crate::prelude::*;
use awsm_renderer::light::Light;
use nalgebra_glm::Vec3;

impl GltfId {
    // these can be customized per model as needed
    pub fn default_lights(&self) -> Vec<(Vec3, Light)> {
        match self {
            _ => {
                vec![
                    // // left
                    // (
                    //     Vec3::new(-10.0, -10.0, 10.0),
                    //     Light::Point{ 
                    //         color: Vec3::new(1.0, 1.0, 1.0), 
                    //         intensity: 700.0,
                    //         range: 100000.0,
                    //     }
                    // ),
                    // // right
                    // (
                    //     Vec3::new(10.0, -10.0, -10.0),
                    //     Light::Point{ 
                    //         color: Vec3::new(1.0, 1.0, 1.0), 
                    //         intensity: 400.0,
                    //         range: 100000.0,
                    //     }
                    // ),
                    // // top
                    // (
                    //     Vec3::new(0.0, 10.0, 0.0),
                    //     Light::Point{ 
                    //         color: Vec3::new(1.0, 1.0, 1.0), 
                    //         intensity: 300.0,
                    //         range: 100000.0,
                    //     }
                    // ),


                    (
                        Vec3::default(),
                        Light::Directional { 
                            direction: Vec3::new(1.0, -1.0, -1.0), 
                            color: Vec3::new(1.0, 1.0, 1.0), 
                            intensity: 3.0 
                        }
                    ),
                    (
                        Vec3::default(),
                        Light::Directional { 
                            direction: Vec3::new(-1.0, 1.0, 1.0), 
                            color: Vec3::new(1.0, 1.0, 1.0), 
                            intensity: 3.0 
                        }
                    ),
                ]
            }
        }
    }
}

pub fn add_demo_lights(world: &mut World, id: GltfId) -> Result<()> {

    let lights = id.default_lights();

    for (translation, light) in lights {
        let entity = world.borrow::<SceneGraphStoragesMut>()?.spawn_child_trs(None, Some(translation), None, None);

        world.run(|
            entities: EntitiesViewMut,
            mut awsm_items: ViewMut<AwsmRendererItem>,
            mut lights: ViewMut<Light>
        | {
            entities.add_component(
                entity, 
                (&mut awsm_items, &mut lights), 
                (AwsmRendererItem {}, light)
            );
        });
    }

    Ok(())
}
