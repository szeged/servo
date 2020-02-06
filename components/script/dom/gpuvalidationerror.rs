/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(non_snake_case)]

use crate::dom::bindings::reflector::Reflector;
use crate::dom::bindings::codegen::Bindings::GPUValidationErrorBinding::{GPUValidationErrorBinding, GPUValidationErrorMethods};
use crate::dom::globalscope::GlobalScope;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::reflector::reflect_dom_object;
use dom_struct::dom_struct;

#[dom_struct]
pub struct GPUValidationError {
    reflector_: Reflector,
    message: DOMString,
}

impl GPUValidationError {
    fn new_inherited(message: DOMString) -> GPUValidationError {
        Self {
            reflector_: Reflector::new(),
            message,
        }
    }

    pub fn new(global: &GlobalScope, message: DOMString) -> DomRoot<GPUValidationError> {
        reflect_dom_object(
            Box::new(GPUValidationError::new_inherited(message)),
            global,
            GPUValidationErrorBinding::Wrap,
        )
    }

    pub fn Constructor(global: &GlobalScope, message: DOMString) -> DomRoot<GPUValidationError> {
        Self::new(global, message)
    }
}

impl GPUValidationErrorMethods for GPUValidationError {
    fn Message(&self) -> DOMString {
        self.message.clone()
    }
}