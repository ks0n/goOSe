#[cfg(drv_ns16550)]
pub mod ns16550;

#[cfg(drv_plic)]
pub mod plic;

pub trait Driver {
    fn init(&self) -> Result<(), ()>;
    fn stop(&self) -> Result<(), ()>;
}

#[derive(Debug)]
pub enum DriverError {
    Other(usize),
}

type DriverResult = Result<&'static dyn Driver, DriverError>;

const DRIVERS: &[fn() -> DriverResult] = &[
    #[cfg(drv_ns16550)]
    ns16550::Ns16550::global,
];

pub fn drivers_init() {
    for driver_initializer in DRIVERS.iter() {
        // FIXME: No unwrap
        driver_initializer().unwrap();
    }
}

// TODO: - have a driver trait
//       - build a list of all the drivers we have compiled, list would countain `impl Driver`
//         stuff, or a functions pointers to instantiate drivers.
//         like a list of struct CompiledDriver {
//                            name: &'static
//                            driver: Fn(struct Driver &) -> impl Driver,
//                        }
