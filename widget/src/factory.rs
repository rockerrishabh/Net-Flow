use std::sync::{Arc, Mutex};
use windows_core::{Error, GUID, HRESULT, IUnknown, Interface, Ref, RuntimeName, implement};

use crate::provider::{NetFlowWidgetProvider, ProviderState};

windows_core::imp::define_interface!(
    IClassFactory,
    IClassFactory_Vtbl,
    0x00000001_0000_0000_c000_000000000046
);

windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);

impl RuntimeName for IClassFactory {
    const NAME: &'static str = "IClassFactory";
}

#[repr(C)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub LockServer: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HRESULT,
}

pub trait IclassFactoryImpl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        punkouter: Ref<IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn LockServer(&self, flock: i32) -> windows_core::Result<()>;
}

impl IClassFactory_Vtbl {
    pub const fn new<Identity: IclassFactoryImpl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IclassFactoryImpl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            punkouter: *mut core::ffi::c_void,
            riid: *const GUID,
            ppvobject: *mut *mut core::ffi::c_void,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*(((this as *const *const ()).offset(OFFSET)) as *const Identity);
                Identity::CreateInstance(
                    this,
                    core::mem::transmute_copy(&punkouter),
                    riid,
                    ppvobject,
                )
                .into()
            }
        }

        unsafe extern "system" fn LockServer<Identity: IclassFactoryImpl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            flock: i32,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*(((this as *const *const ()).offset(OFFSET)) as *const Identity);
                Identity::LockServer(this, flock).into()
            }
        }

        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            LockServer: LockServer::<Identity, OFFSET>,
        }
    }

    pub fn matches(iid: &GUID) -> bool {
        iid == &<IClassFactory as Interface>::IID
    }
}

#[implement(IClassFactory)]
pub struct NetFlowClassFactory {
    pub state: Arc<Mutex<ProviderState>>,
}

impl NetFlowClassFactory {
    pub fn new(state: Arc<Mutex<ProviderState>>) -> Self {
        Self { state }
    }
}

impl IclassFactoryImpl for NetFlowClassFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Ref<IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        if ppvobject.is_null() {
            return Err(Error::from_hresult(HRESULT(0x80004003_u32 as i32))); // E_POINTER
        }
        unsafe {
            *ppvobject = core::ptr::null_mut();
        }

        if punkouter.as_ref().is_some() {
            return Err(Error::from_hresult(HRESULT(0x80040110_u32 as i32))); // CLASS_E_NOAGGREGATION
        }

        let provider = NetFlowWidgetProvider::new(Arc::clone(&self.state));
        let unknown: IUnknown = provider.into();

        let hr = unsafe {
            (Interface::vtable(&unknown).QueryInterface)(
                Interface::as_raw(&unknown),
                riid,
                ppvobject,
            )
        };

        hr.ok()
    }

    fn LockServer(&self, _flock: i32) -> windows_core::Result<()> {
        Ok(())
    }
}
