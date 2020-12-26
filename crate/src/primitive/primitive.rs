use crate::prelude::*;
use awsm_web::webgl::Id;
use crate::geom::quad::Quad;

pub enum Primitive {
    Quad(Quad)
}