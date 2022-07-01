//! This module stores all drivers strictly necessary for the kernel.
//! As we want the kernel as small as possible, each driver is hidden behind a feature flag in
//! order to get compiled only is the platform requires it

pub mod ns16550;

pub mod plic;


// TODO: - have a driver trait
//       - build a list of all the drivers we have compiled, list would countain `impl Driver`
//         stuff, or a functions pointers to instantiate drivers.
//         like a list of struct CompiledDriver {
//                            name: &'static
//                            driver: Fn(struct Driver &) -> impl Driver,
//                        }
