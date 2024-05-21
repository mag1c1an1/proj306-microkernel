use super::ndks_boot;
use crate::common::{sel4_config::*, utils::convert_to_mut_type_ref};
use crate::config::CONFIG_ROOT_CNODE_SIZE_BITS;
use crate::cspace::interface::*;
use crate::structures::{p_region_t, region_t, v_region_t};
use crate::vspace::*;
use crate::{BIT, ROUND_DOWN, ROUND_UP};
use log::debug;
