use web_sys::{HtmlCanvasElement, window};
use std::ops::{Deref, DerefMut};
use crate::prelude::*;
use awsm_web::{webgl::{WebGlContextOptions, ResizeStrategy}, tick::{Raf, MainLoop, MainLoopOptions}, dom::resize::ResizeObserver};
use shipyard_scenegraph::prelude::*;
use awsm_renderer::{
    camera::{arc_ball::ArcBall, screen_static::ScreenStatic},
    renderer::{
        systems::{
            render_sys,
            update_skin_joints_sys
        },
        CanvasOrGl,
        Config
    }, 
    animation::systems::{
        animation_clock_sys,
        animation_update_translation_sys,
        animation_update_rotation_sys,
        animation_update_scale_sys,
        animation_update_morph_sys,
    },
};

const TRANSFORMS:&'static str = "TRANSFORMS";
const ANIMATION_UPDATES:&'static str = "ANIMATION_UPDATES";

pub async fn init_world(canvas:HtmlCanvasElement) -> Result<(Rc<RefCell<World>>, Rc<RefCell<AwsmRenderer>>)> {
    let world = Rc::new(RefCell::new(World::new()));

    let renderer = Rc::new(RefCell::new(AwsmRenderer::new(
        &*world.borrow(), 
        CanvasOrGl::Canvas(&canvas, 
            Some(&WebGlContextOptions {
                alpha: false,
                antialias: false,
                ..WebGlContextOptions::default()
            })
        ),
        Config {
            clear_color: [0.5, 0.5, 0.5, 1.0]
        }
    )?));

    Workload::new(TRANSFORMS)
        .with_system(local_transform_sys)
        .with_system(world_transform_sys)
        .with_system(update_skin_joints_sys)
        .add_to_world(&*world.borrow())
        .unwrap_ext();

    Workload::new(ANIMATION_UPDATES)
        .with_system(animation_update_translation_sys)
        .with_system(animation_update_rotation_sys)
        .with_system(animation_update_scale_sys)
        .with_system(animation_update_morph_sys)
        .add_to_world(&*world.borrow())
        .unwrap_ext();

    let main_loop_opts = MainLoopOptions::default();
    let timestep = main_loop_opts.simulation_timestep;

    let mut main_loop = MainLoop::new(
        main_loop_opts,
        {
            let world = Rc::clone(&world);
            move |time, delta| {
            }
        },
        {
            let world = Rc::clone(&world);
            move |delta| {
                let world = &*world.borrow();
                // animation clock is updated without actually affecting the animations
                world.run_with_data(animation_clock_sys, delta);
            }
        },
        {
            let world = Rc::clone(&world);
            let renderer = Rc::clone(&renderer);
            move |interpolation| {

                let world = &*world.borrow();
                if let Ok(evt) = world.borrow::<UniqueView<ResizeEvent>>() {
                    renderer.borrow_mut().resize(evt.0).unwrap_ext();
                    drop(evt); // so we can remove within this borrow scope
                    world.remove_unique::<ResizeEvent>().unwrap_ext();
                }

                let start_time = window().unwrap_ext().performance().unwrap_ext().now();
                world.run_workload(ANIMATION_UPDATES).unwrap_ext();
                let animations_time = window().unwrap_ext().performance().unwrap_ext().now();
                world.run_workload(TRANSFORMS).unwrap_ext();
                let transform_time = window().unwrap_ext().performance().unwrap_ext().now();
                if let Err(err) = world.run_with_data(render_sys, &mut *renderer.borrow_mut()) {
                    log::error!("{}", err);
                }
                let end_time = window().unwrap_ext().performance().unwrap_ext().now();

                //log::info!("animations_time: {}, transform time: {}, render time: {}, total_time: {}", 
                    //animations_time - start_time,
                    //animations_time - transform_time,
                    //end_time - transform_time,
                    //end_time - start_time
                //);
            }
        },
        {
            let world = Rc::clone(&world);
            move |fps, abort| {
            }
        },
    );

    let tick = Raf::new({
        move |ts| {
            main_loop.tick(ts);
        }
    });

    world.borrow().add_unique_non_send_sync(TickWrapper(tick));

    let resize = ResizeObserver::new({
        let world = Rc::clone(&world);
        move |entries| {
            let world = &*world.borrow();
            let entry = entries.get(0).unwrap_ext();
            let rect = &entry.content_rect;
            world.add_unique(ResizeEvent(ResizeStrategy::All(rect.width() as u32, rect.height() as u32)));
        }
    }, None);

    resize.observe(&canvas);

    world.borrow().add_unique_non_send_sync(ResizeWrapper(resize));

    Ok((world, renderer))
}


#[derive(Component, Unique)]
pub struct TickWrapper(Raf);

#[derive(Component, Unique)]
pub struct ResizeWrapper(ResizeObserver);

#[derive(Component, Unique)]
pub struct ResizeEvent(ResizeStrategy);
