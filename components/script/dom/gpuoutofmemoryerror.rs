/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(non_snake_case)]

use crate::dom::bindings::reflector::Reflector;
use crate::dom::bindings::codegen::Bindings::GPUOutOfMemoryErrorBinding::GPUOutOfMemoryErrorBinding;
use crate::dom::globalscope::GlobalScope;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::reflector::reflect_dom_object;
use dom_struct::dom_struct;

#[dom_struct]
pub struct GPUOutOfMemoryError {
    reflector_: Reflector,
}

impl GPUOutOfMemoryError {
    fn new_inherited() -> GPUOutOfMemoryError {
        Self {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: &GlobalScope) -> DomRoot<GPUOutOfMemoryError> {
        reflect_dom_object(
            Box::new(GPUOutOfMemoryError::new_inherited()),
            global,
            GPUOutOfMemoryErrorBinding::Wrap,
        )
    }

    pub fn Constructor(global: &GlobalScope) -> DomRoot<Self> {
        Self::new(global)
    }
}
