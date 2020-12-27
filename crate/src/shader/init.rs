use awsm_web::webgl::{WebGl2Renderer, Id};

pub struct StaticShaders {
    pub vertex: VertexShaders,
    pub fragment: FragmentShaders,
    pub programs: Programs
}

pub struct VertexShaders {
    unit: Id
}

pub struct FragmentShaders {
    unlit_diffuse: Id
}

pub struct Programs {
    v_unit_f_unlit_diffuse: Id
}

impl StaticShaders {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {
        //waiting on https://github.com/dakom/awsm-web/issues/18
        panic!("TODO!");

    }
}
