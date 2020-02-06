/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::reflector::Reflector;
use crate::dom::bindings::codegen::Bindings::GPUDeviceLostInfoBinding::GPUDeviceLostInfoBinding::GPUDeviceLostInfoMethods;
use dom_struct::dom_struct;
use crate::dom::bindings::str::DOMString;

#[dom_struct]
pub struct GPUDeviceLostInfo {
    reflector_: Reflector,
    message: DOMString
}

impl GPUDeviceLostInfoMethods for GPUDeviceLostInfo {
    fn Message(&self) -> DOMString {
        self.message.clone()
    }
}