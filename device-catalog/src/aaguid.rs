use crate::certificate_authority::Authority;
use std::rc::Rc;
use uuid::Uuid;

use std::fmt;

#[derive(Debug)]
pub struct Aaguid {
    pub id: Uuid,
    pub ca: Vec<Rc<Authority>>,
}
