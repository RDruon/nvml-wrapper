use crate::error::{nvml_try, NvmlError};
use crate::ffi::bindings::*;
use crate::NVML;

use std::{
    io::{self, Write},
    marker::PhantomData,
    mem,
};

use crate::struct_wrappers::event::EventData;

/**
Handle to a set of events.

**Operations on a set are not thread-safe.** It does not, therefore, implement `Sync`.

You can get yourself an `EventSet` via `NVML.create_event_set()`. Once again, Rust's
lifetimes will ensure that it does not outlive the `NVML` instance that it was created
from.
*/
// Checked against local
#[derive(Debug)]
pub struct EventSet<'nvml> {
    set: nvmlEventSet_t,
    _phantom: PhantomData<&'nvml NVML>,
}

unsafe impl<'nvml> Send for EventSet<'nvml> {}

impl<'nvml> From<nvmlEventSet_t> for EventSet<'nvml> {
    fn from(set: nvmlEventSet_t) -> Self {
        EventSet {
            set,
            _phantom: PhantomData,
        }
    }
}

impl<'nvml> EventSet<'nvml> {
    /**
    Use this to release the set's events if you care about handling
    potential errors (*the `Drop` implementation ignores errors!*).

    # Errors

    * `Uninitialized`, if the library has not been successfully initialized
    * `Unknown`, on any unexpected error
    */
    // Checked against local
    pub fn release_events(self) -> Result<(), NvmlError> {
        unsafe {
            nvml_try(nvmlEventSetFree(self.set))?;
        }

        mem::forget(self);
        Ok(())
    }

    /**
    Waits on events for the given timeout (in ms) and delivers one when it arrives.

    See the `high_level::event_loop` module for an abstracted version of this.

    This method returns immediately if an event is ready to be delivered when it
    is called. If no events are ready it will sleep until an event arrives, but
    not longer than the specified timeout. In certain conditions, this method
    could return before the timeout passes (e.g. when an interrupt arrives).

    In the case of an XID error, the function returns the most recent XID error
    type seen by the system. If there are multiple XID errors generated before
    this method is called, the last seen XID error type will be returned for
    all XID error events.

    # Errors

    * `Uninitialized`, if the library has not been successfully initialized
    * `Timeout`, if no event arrived in the specified timeout or an interrupt
    arrived
    * `GpuLost`, if a GPU has fallen off the bus or is otherwise inaccessible
    * `Unknown`, on any unexpected error

    # Device Support

    Supports Fermi and newer fully supported devices.
    */
    // Checked against local
    pub fn wait(&self, timeout_ms: u32) -> Result<EventData<'nvml>, NvmlError> {
        unsafe {
            let mut data: nvmlEventData_t = mem::zeroed();
            nvml_try(nvmlEventSetWait(self.set, &mut data, timeout_ms))?;

            Ok(data.into())
        }
    }

    /// Get the raw device handle contained in this struct
    ///
    /// Sometimes necessary for C interop.
    ///
    /// # Safety
    ///
    /// This is unsafe to prevent it from being used without care. In
    /// particular, you must avoid creating a new `EventSet` from this handle
    /// and allowing both this `EventSet` and the newly created one to drop
    /// (which would result in a double-free).
    pub unsafe fn handle(&self) -> nvmlEventSet_t {
        self.set
    }
}

/// This `Drop` implementation ignores errors! Use the `.release_events()`
/// method on the `EventSet` struct if you care about handling them.
impl<'nvml> Drop for EventSet<'nvml> {
    fn drop(&mut self) {
        #[allow(unused_must_use)]
        unsafe {
            match nvml_try(nvmlEventSetFree(self.set)) {
                Ok(()) => (),
                Err(e) => {
                    io::stderr().write(
                        format!(
                            "WARNING: Error returned by `nvmlEventSetFree()` in Drop \
                             implementation: {:?}",
                            e
                        )
                        .as_bytes(),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[cfg(target_os = "linux")]
    use crate::bitmasks::event::*;
    use crate::test_utils::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn release_events() {
        let nvml = nvml();
        test_with_device(3, &nvml, |device| {
            let set = nvml.create_event_set()?;
            let set = device
                .register_events(
                    EventTypes::PSTATE_CHANGE
                        | EventTypes::CRITICAL_XID_ERROR
                        | EventTypes::CLOCK_CHANGE,
                    set,
                )
                .map_err(|e| e.error)?;

            set.release_events()
        })
    }

    #[cfg(target_os = "linux")]
    #[cfg(feature = "test-local")]
    #[test]
    fn wait() {
        use crate::error::NvmlError;

        let nvml = nvml();
        let device = device(&nvml);
        let set = nvml.create_event_set().expect("event set");
        let set = device
            .register_events(
                EventTypes::PSTATE_CHANGE
                    | EventTypes::CRITICAL_XID_ERROR
                    | EventTypes::CLOCK_CHANGE,
                set,
            )
            .expect("registration");

        let data = match set.wait(10_000) {
            Err(NvmlError::Timeout) => return (),
            Ok(d) => d,
            _ => panic!("An error other than `Timeout` occurred"),
        };

        print!("{:?} ...", data);
    }
}
