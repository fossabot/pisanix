// Copyright 2022 SphereEx Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    circuit_breaker::{CircuitBreaker, CircuitBreakerLayer, self},
    config,
    err::PluginError,
    layer::*,
    concurrency_control::{ConcurrencyControl, ConcurrencyControlLayer, self},
};

/// concurrency control service, some logic may be added in the future, eg: metrics...
fn concurrency_control_phase(_input: String) -> Result<(), PluginError> {
    Ok(())
}

/// circuit breaker service, some logic may be added in the future, eg: metrics...
fn circuit_breaker_phase(_input: String) -> Result<(), PluginError> {
    Ok(())
}

#[derive(Clone)]
pub struct PluginPhase {
    pub concurrency_control: ConcurrencyControl<ServiceFn<fn(String) -> Result<(), PluginError>>>,
    pub circuit_breaker: CircuitBreaker<ServiceFn<fn(String) -> Result<(), PluginError>>>,
}

impl PluginPhase {
    pub fn new(config: config::Plugin) -> PluginPhase {
        let concurrency_control= ServiceBuilder::new()
            .with_layer(ConcurrencyControlLayer::with_opt(config.concurrency_control))
            // issue https://users.rust-lang.org/t/puzzling-expected-fn-pointer-found-fn-item/46423/4
            .build(service_fn(concurrency_control_phase as fn(String) -> Result<(), PluginError>));

        let circuit_breaker= ServiceBuilder::new()
            .with_layer(CircuitBreakerLayer::with_opt(config.circuit_breaker))
            .build(service_fn(circuit_breaker_phase as fn(String) -> Result<(), PluginError>));

        PluginPhase { concurrency_control, circuit_breaker }
    }
}
