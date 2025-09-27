use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::noa::{
    environment::Environment,
    error::{NoaError, NoaTermination},
    types::{Number, Object},
};
