windows_core::imp::define_interface!(
    IWidgetActionInvokedArgs,
    IWidgetActionInvokedArgs_Vtbl,
    0xc593cc57_04b9_52ca_88ad_46fea21ea340
);
impl windows_core::RuntimeType for IWidgetActionInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetActionInvokedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetActionInvokedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetActionInvokedArgs";
}
pub trait IWidgetActionInvokedArgs_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Data(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWidgetActionInvokedArgs_Vtbl {
    pub const fn new<Identity: IWidgetActionInvokedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidgetContext<
            Identity: IWidgetActionInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetActionInvokedArgs_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Verb<
            Identity: IWidgetActionInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetActionInvokedArgs_Impl::Verb(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<
            Identity: IWidgetActionInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetActionInvokedArgs_Impl::Data(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CustomState<
            Identity: IWidgetActionInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetActionInvokedArgs_Impl::CustomState(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetActionInvokedArgs,
                OFFSET,
            >(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
            Verb: Verb::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            CustomState: CustomState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetActionInvokedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetActionInvokedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Verb: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetAnalyticsInfoReportedArgs,
    IWidgetAnalyticsInfoReportedArgs_Vtbl,
    0x1d9e5fb5_2bce_5350_87b1_d63199526639
);
impl windows_core::RuntimeType for IWidgetAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetAnalyticsInfoReportedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.IWidgetAnalyticsInfoReportedArgs";
}
pub trait IWidgetAnalyticsInfoReportedArgs_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
    fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWidgetAnalyticsInfoReportedArgs_Vtbl {
    pub const fn new<Identity: IWidgetAnalyticsInfoReportedArgs_Impl, const OFFSET: isize>() -> Self
    {
        unsafe extern "system" fn WidgetContext<
            Identity: IWidgetAnalyticsInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetAnalyticsInfoReportedArgs_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnalyticsJson<
            Identity: IWidgetAnalyticsInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetAnalyticsInfoReportedArgs_Impl::AnalyticsJson(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetAnalyticsInfoReportedArgs,
                OFFSET,
            >(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
            AnalyticsJson: AnalyticsJson::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetAnalyticsInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AnalyticsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetContext,
    IWidgetContext_Vtbl,
    0x903c518b_40bc_5bc6_88f7_af9d81c0cdc1
);
impl windows_core::RuntimeType for IWidgetContext {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetContext",
    );
}
impl windows_core::RuntimeName for IWidgetContext {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetContext";
}
pub trait IWidgetContext_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn DefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Size(&self) -> windows_core::Result<super::WidgetSize>;
    fn IsActive(&self) -> windows_core::Result<bool>;
}
impl IWidgetContext_Vtbl {
    pub const fn new<Identity: IWidgetContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IWidgetContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetContext_Impl::Id(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefinitionId<
            Identity: IWidgetContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetContext_Impl::DefinitionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IWidgetContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut super::WidgetSize,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetContext_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsActive<Identity: IWidgetContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetContext_Impl::IsActive(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetContext, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            DefinitionId: DefinitionId::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetContext as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut super::WidgetSize,
    ) -> windows_core::HRESULT,
    pub IsActive:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetContextChangedArgs,
    IWidgetContextChangedArgs_Vtbl,
    0x2c226d54_2252_576b_a197_370b28d25c2f
);
impl windows_core::RuntimeType for IWidgetContextChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetContextChangedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetContextChangedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetContextChangedArgs";
}
pub trait IWidgetContextChangedArgs_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
}
impl IWidgetContextChangedArgs_Vtbl {
    pub const fn new<Identity: IWidgetContextChangedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidgetContext<
            Identity: IWidgetContextChangedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetContextChangedArgs_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetContextChangedArgs,
                OFFSET,
            >(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetContextChangedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetContextChangedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetCustomizationRequestedArgs,
    IWidgetCustomizationRequestedArgs_Vtbl,
    0x41dea311_dd9b_5b8b_b493_3a30552116b8
);
impl windows_core::RuntimeType for IWidgetCustomizationRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetCustomizationRequestedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetCustomizationRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.IWidgetCustomizationRequestedArgs";
}
pub trait IWidgetCustomizationRequestedArgs_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWidgetCustomizationRequestedArgs_Vtbl {
    pub const fn new<Identity: IWidgetCustomizationRequestedArgs_Impl, const OFFSET: isize>() -> Self
    {
        unsafe extern "system" fn WidgetContext<
            Identity: IWidgetCustomizationRequestedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetCustomizationRequestedArgs_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CustomState<
            Identity: IWidgetCustomizationRequestedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetCustomizationRequestedArgs_Impl::CustomState(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetCustomizationRequestedArgs,
                OFFSET,
            >(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
            CustomState: CustomState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetCustomizationRequestedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetCustomizationRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetErrorInfoReportedArgs,
    IWidgetErrorInfoReportedArgs_Vtbl,
    0x30efa627_b21f_55d5_b91a_b23b4aa13645
);
impl windows_core::RuntimeType for IWidgetErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetErrorInfoReportedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetErrorInfoReportedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetErrorInfoReportedArgs";
}
pub trait IWidgetErrorInfoReportedArgs_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
    fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWidgetErrorInfoReportedArgs_Vtbl {
    pub const fn new<Identity: IWidgetErrorInfoReportedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidgetContext<
            Identity: IWidgetErrorInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetErrorInfoReportedArgs_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorJson<
            Identity: IWidgetErrorInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetErrorInfoReportedArgs_Impl::ErrorJson(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetErrorInfoReportedArgs,
                OFFSET,
            >(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
            ErrorJson: ErrorJson::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetErrorInfoReportedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetErrorInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ErrorJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetInfo,
    IWidgetInfo_Vtbl,
    0xcea11f42_a020_5db5_89e2_b7dece4ae5cb
);
impl windows_core::RuntimeType for IWidgetInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetInfo",
    );
}
impl windows_core::RuntimeName for IWidgetInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetInfo";
}
pub trait IWidgetInfo_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
    fn Template(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Data(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn LastUpdateTime(&self) -> windows_core::Result<windows_time::DateTime>;
}
impl IWidgetInfo_Vtbl {
    pub const fn new<Identity: IWidgetInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidgetContext<Identity: IWidgetInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Template<Identity: IWidgetInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo_Impl::Template(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<Identity: IWidgetInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo_Impl::Data(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CustomState<Identity: IWidgetInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo_Impl::CustomState(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastUpdateTime<
            Identity: IWidgetInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_time::DateTime,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo_Impl::LastUpdateTime(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetInfo, OFFSET>(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            CustomState: CustomState::<Identity, OFFSET>,
            LastUpdateTime: LastUpdateTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetInfo as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LastUpdateTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetInfo2,
    IWidgetInfo2_Vtbl,
    0x081b0a6f_d784_5408_bb29_252fef2926d4
);
impl windows_core::RuntimeType for IWidgetInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetInfo2",
    );
}
impl windows_core::RuntimeName for IWidgetInfo2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetInfo2";
}
pub trait IWidgetInfo2_Impl: windows_core::IUnknownImpl {
    fn IsPlaceholderContent(&self) -> windows_core::Result<bool>;
}
impl IWidgetInfo2_Vtbl {
    pub const fn new<Identity: IWidgetInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPlaceholderContent<
            Identity: IWidgetInfo2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo2_Impl::IsPlaceholderContent(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetInfo2, OFFSET>(),
            IsPlaceholderContent: IsPlaceholderContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetInfo2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPlaceholderContent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetInfo3,
    IWidgetInfo3_Vtbl,
    0x965538cd_289d_54ab_916e_9315ebf97ea4
);
impl windows_core::RuntimeType for IWidgetInfo3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetInfo3",
    );
}
impl windows_core::RuntimeName for IWidgetInfo3 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetInfo3";
}
pub trait IWidgetInfo3_Impl: windows_core::IUnknownImpl {
    fn Rank(&self) -> windows_core::Result<i32>;
}
impl IWidgetInfo3_Vtbl {
    pub const fn new<Identity: IWidgetInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Rank<Identity: IWidgetInfo3_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetInfo3_Impl::Rank(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetInfo3, OFFSET>(),
            Rank: Rank::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetInfo3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetInfo3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Rank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetManager,
    IWidgetManager_Vtbl,
    0x71cb10c0_671e_48e3_b995_207940397123
);
impl windows_core::RuntimeType for IWidgetManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetManager",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetManager {
    pub fn UpdateWidget<P0>(&self, widgetupdaterequestoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetUpdateRequestOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateWidget)(
                windows_core::Interface::as_raw(self),
                widgetupdaterequestoptions.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetWidgetIds(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetIds)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(
        &self,
        widgetid: &windows_core::HSTRING,
    ) -> windows_core::Result<WidgetInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfo)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetWidgetInfos(&self) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<WidgetInfo>::set_abi_len(core::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteWidget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetManager";
}
pub trait IWidgetManager_Impl: windows_core::IUnknownImpl {
    fn UpdateWidget(
        &self,
        widgetUpdateRequestOptions: windows_core::Ref<WidgetUpdateRequestOptions>,
    ) -> windows_core::Result<()>;
    fn GetWidgetIds(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>;
    fn GetWidgetInfo(&self, widgetId: &windows_core::HSTRING) -> windows_core::Result<WidgetInfo>;
    fn GetWidgetInfos(&self) -> windows_core::Result<windows_core::Array<WidgetInfo>>;
    fn DeleteWidget(&self, widgetId: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IWidgetManager_Vtbl {
    pub const fn new<Identity: IWidgetManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateWidget<
            Identity: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetupdaterequestoptions: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetManager_Impl::UpdateWidget(
                    this,
                    core::mem::transmute_copy(&widgetupdaterequestoptions),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetWidgetIds<
            Identity: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetManager_Impl::GetWidgetIds(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWidgetInfo<
            Identity: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetManager_Impl::GetWidgetInfo(this, core::mem::transmute(&widgetid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWidgetInfos<
            Identity: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetManager_Impl::GetWidgetInfos(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetManager_Impl::DeleteWidget(this, core::mem::transmute(&widgetid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetManager, OFFSET>(),
            UpdateWidget: UpdateWidget::<Identity, OFFSET>,
            GetWidgetIds: GetWidgetIds::<Identity, OFFSET>,
            GetWidgetInfo: GetWidgetInfo::<Identity, OFFSET>,
            GetWidgetInfos: GetWidgetInfos::<Identity, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetManager as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWidgetIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWidgetInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWidgetInfos: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetManager2,
    IWidgetManager2_Vtbl,
    0x55c65a27_8845_406c_9ee1_1e79f0556bef
);
impl windows_core::RuntimeType for IWidgetManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetManager2",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetManager2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetManager2 {
    pub fn SendMessageToContent(
        &self,
        widgetid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SendMessageToContent)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetManager2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetManager2";
}
pub trait IWidgetManager2_Impl: windows_core::IUnknownImpl {
    fn SendMessageToContent(
        &self,
        widgetId: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()>;
}
impl IWidgetManager2_Vtbl {
    pub const fn new<Identity: IWidgetManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendMessageToContent<
            Identity: IWidgetManager2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
            message: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetManager2_Impl::SendMessageToContent(
                    this,
                    core::mem::transmute(&widgetid),
                    core::mem::transmute(&message),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetManager2, OFFSET>(),
            SendMessageToContent: SendMessageToContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetManager2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessageToContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetManagerStatics,
    IWidgetManagerStatics_Vtbl,
    0x7f233b06_28e5_5e2b_8c04_a4fa747c28c7
);
impl windows_core::RuntimeType for IWidgetManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetManagerStatics",
    );
}
impl windows_core::RuntimeName for IWidgetManagerStatics {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetManagerStatics";
}
pub trait IWidgetManagerStatics_Impl: windows_core::IUnknownImpl {
    fn GetDefault(&self) -> windows_core::Result<WidgetManager>;
}
impl IWidgetManagerStatics_Vtbl {
    pub const fn new<Identity: IWidgetManagerStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDefault<
            Identity: IWidgetManagerStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetManagerStatics_Impl::GetDefault(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetManagerStatics, OFFSET>(
            ),
            GetDefault: GetDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetManagerStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetMessageReceivedArgs,
    IWidgetMessageReceivedArgs_Vtbl,
    0x2261cb2b_c741_5f96_9adb_fb3a7667bcb6
);
impl windows_core::RuntimeType for IWidgetMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetMessageReceivedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetMessageReceivedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetMessageReceivedArgs";
}
pub trait IWidgetMessageReceivedArgs_Impl: windows_core::IUnknownImpl {
    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
    fn Message(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWidgetMessageReceivedArgs_Vtbl {
    pub const fn new<Identity: IWidgetMessageReceivedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidgetContext<
            Identity: IWidgetMessageReceivedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetMessageReceivedArgs_Impl::WidgetContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Message<
            Identity: IWidgetMessageReceivedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetMessageReceivedArgs_Impl::Message(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetMessageReceivedArgs,
                OFFSET,
            >(),
            WidgetContext: WidgetContext::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetMessageReceivedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetMessageReceivedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProvider,
    IWidgetProvider_Vtbl,
    0x5c5774cc_72a0_452d_b9ed_075c0dd25eed
);
impl windows_core::RuntimeType for IWidgetProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProvider {
    pub fn CreateWidget<P0>(&self, widgetcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetContext>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateWidget)(
                windows_core::Interface::as_raw(self),
                widgetcontext.param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteWidget(
        &self,
        widgetid: &windows_core::HSTRING,
        customstate: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteWidget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                core::mem::transmute_copy(customstate),
            )
            .ok()
        }
    }
    pub fn OnActionInvoked<P0>(&self, actioninvokedargs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetActionInvokedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnActionInvoked)(
                windows_core::Interface::as_raw(self),
                actioninvokedargs.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnWidgetContextChanged<P0>(&self, contextchangedargs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetContextChangedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnWidgetContextChanged)(
                windows_core::Interface::as_raw(self),
                contextchangedargs.param().abi(),
            )
            .ok()
        }
    }
    pub fn Activate<P0>(&self, widgetcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetContext>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Activate)(
                windows_core::Interface::as_raw(self),
                widgetcontext.param().abi(),
            )
            .ok()
        }
    }
    pub fn Deactivate(&self, widgetid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Deactivate)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider";
}
pub trait IWidgetProvider_Impl: windows_core::IUnknownImpl {
    fn CreateWidget(
        &self,
        widgetContext: windows_core::Ref<WidgetContext>,
    ) -> windows_core::Result<()>;
    fn DeleteWidget(
        &self,
        widgetId: &windows_core::HSTRING,
        customState: &windows_core::HSTRING,
    ) -> windows_core::Result<()>;
    fn OnActionInvoked(
        &self,
        actionInvokedArgs: windows_core::Ref<WidgetActionInvokedArgs>,
    ) -> windows_core::Result<()>;
    fn OnWidgetContextChanged(
        &self,
        contextChangedArgs: windows_core::Ref<WidgetContextChangedArgs>,
    ) -> windows_core::Result<()>;
    fn Activate(&self, widgetContext: windows_core::Ref<WidgetContext>)
    -> windows_core::Result<()>;
    fn Deactivate(&self, widgetId: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IWidgetProvider_Vtbl {
    pub const fn new<Identity: IWidgetProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateWidget<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetcontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::CreateWidget(this, core::mem::transmute_copy(&widgetcontext))
                    .into()
            }
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
            customstate: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::DeleteWidget(
                    this,
                    core::mem::transmute(&widgetid),
                    core::mem::transmute(&customstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnActionInvoked<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            actioninvokedargs: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::OnActionInvoked(
                    this,
                    core::mem::transmute_copy(&actioninvokedargs),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnWidgetContextChanged<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            contextchangedargs: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::OnWidgetContextChanged(
                    this,
                    core::mem::transmute_copy(&contextchangedargs),
                )
                .into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IWidgetProvider_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            widgetcontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::Activate(this, core::mem::transmute_copy(&widgetcontext))
                    .into()
            }
        }
        unsafe extern "system" fn Deactivate<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::Deactivate(this, core::mem::transmute(&widgetid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProvider, OFFSET>(),
            CreateWidget: CreateWidget::<Identity, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, OFFSET>,
            OnActionInvoked: OnActionInvoked::<Identity, OFFSET>,
            OnWidgetContextChanged: OnWidgetContextChanged::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnActionInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnWidgetContextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProvider2,
    IWidgetProvider2_Vtbl,
    0x38c3a963_dd93_479d_9276_04bf84ee1816
);
impl windows_core::RuntimeType for IWidgetProvider2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProvider2",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProvider2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProvider2 {
    pub fn OnCustomizationRequested<P0>(
        &self,
        customizationrequestedargs: P0,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetCustomizationRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnCustomizationRequested)(
                windows_core::Interface::as_raw(self),
                customizationrequestedargs.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProvider2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider2";
}
pub trait IWidgetProvider2_Impl: windows_core::IUnknownImpl {
    fn OnCustomizationRequested(
        &self,
        customizationRequestedArgs: windows_core::Ref<WidgetCustomizationRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IWidgetProvider2_Vtbl {
    pub const fn new<Identity: IWidgetProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCustomizationRequested<
            Identity: IWidgetProvider2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            customizationrequestedargs: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider2_Impl::OnCustomizationRequested(
                    this,
                    core::mem::transmute_copy(&customizationrequestedargs),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProvider2, OFFSET>(),
            OnCustomizationRequested: OnCustomizationRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProvider2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProvider2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnCustomizationRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProviderAnalytics,
    IWidgetProviderAnalytics_Vtbl,
    0x661985a5_d187_482d_9eef_6fda05d21845
);
impl windows_core::RuntimeType for IWidgetProviderAnalytics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProviderAnalytics",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProviderAnalytics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProviderAnalytics {
    pub fn OnAnalyticsInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetAnalyticsInfoReportedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnAnalyticsInfoReported)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProviderAnalytics {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProviderAnalytics";
}
pub trait IWidgetProviderAnalytics_Impl: windows_core::IUnknownImpl {
    fn OnAnalyticsInfoReported(
        &self,
        args: windows_core::Ref<WidgetAnalyticsInfoReportedArgs>,
    ) -> windows_core::Result<()>;
}
impl IWidgetProviderAnalytics_Vtbl {
    pub const fn new<Identity: IWidgetProviderAnalytics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAnalyticsInfoReported<
            Identity: IWidgetProviderAnalytics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProviderAnalytics_Impl::OnAnalyticsInfoReported(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetProviderAnalytics,
                OFFSET,
            >(),
            OnAnalyticsInfoReported: OnAnalyticsInfoReported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProviderAnalytics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProviderAnalytics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProviderErrors,
    IWidgetProviderErrors_Vtbl,
    0x90c1b5f0_0d3a_4ac6_abb7_c97b367b8fcc
);
impl windows_core::RuntimeType for IWidgetProviderErrors {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProviderErrors",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProviderErrors,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProviderErrors {
    pub fn OnErrorInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetErrorInfoReportedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnErrorInfoReported)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProviderErrors {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProviderErrors";
}
pub trait IWidgetProviderErrors_Impl: windows_core::IUnknownImpl {
    fn OnErrorInfoReported(
        &self,
        args: windows_core::Ref<WidgetErrorInfoReportedArgs>,
    ) -> windows_core::Result<()>;
}
impl IWidgetProviderErrors_Vtbl {
    pub const fn new<Identity: IWidgetProviderErrors_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnErrorInfoReported<
            Identity: IWidgetProviderErrors_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProviderErrors_Impl::OnErrorInfoReported(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProviderErrors, OFFSET>(
            ),
            OnErrorInfoReported: OnErrorInfoReported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProviderErrors as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProviderErrors_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnErrorInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProviderMessage,
    IWidgetProviderMessage_Vtbl,
    0xea4dc186_9e24_4b35_a5ef_a9f5df72d6ac
);
impl windows_core::RuntimeType for IWidgetProviderMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProviderMessage",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProviderMessage,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProviderMessage {
    pub fn OnMessageReceived<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetMessageReceivedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnMessageReceived)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProviderMessage {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProviderMessage";
}
pub trait IWidgetProviderMessage_Impl: windows_core::IUnknownImpl {
    fn OnMessageReceived(
        &self,
        args: windows_core::Ref<WidgetMessageReceivedArgs>,
    ) -> windows_core::Result<()>;
}
impl IWidgetProviderMessage_Vtbl {
    pub const fn new<Identity: IWidgetProviderMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMessageReceived<
            Identity: IWidgetProviderMessage_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProviderMessage_Impl::OnMessageReceived(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProviderMessage, OFFSET>(
            ),
            OnMessageReceived: OnMessageReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProviderMessage as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProviderMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnMessageReceived: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceProvider,
    IWidgetResourceProvider_Vtbl,
    0xdcf328c0_012c_40f5_bb28_3a1c714d027d
);
impl windows_core::RuntimeType for IWidgetResourceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetResourceProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetResourceProvider {
    pub fn OnResourceRequested<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetResourceRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnResourceRequested)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetResourceProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetResourceProvider";
}
pub trait IWidgetResourceProvider_Impl: windows_core::IUnknownImpl {
    fn OnResourceRequested(
        &self,
        args: windows_core::Ref<WidgetResourceRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IWidgetResourceProvider_Vtbl {
    pub const fn new<Identity: IWidgetResourceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnResourceRequested<
            Identity: IWidgetResourceProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetResourceProvider_Impl::OnResourceRequested(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetResourceProvider, OFFSET>(
            ),
            OnResourceRequested: OnResourceRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetResourceProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetResourceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnResourceRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceRequest,
    IWidgetResourceRequest_Vtbl,
    0x113d249f_82d9_57cb_8cea_9a5291f2fe22
);
impl windows_core::RuntimeType for IWidgetResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequest",
    );
}
impl windows_core::RuntimeName for IWidgetResourceRequest {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetResourceRequest";
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetResourceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Method: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Content: usize,
    SetContent: usize,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceRequestedArgs,
    IWidgetResourceRequestedArgs_Vtbl,
    0x2bb30f4d_0166_58e3_aaf6_31b2ae970bcd
);
impl windows_core::RuntimeType for IWidgetResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequestedArgs",
    );
}
impl windows_core::RuntimeName for IWidgetResourceRequestedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetResourceRequestedArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetResourceRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceResponse,
    IWidgetResourceResponse_Vtbl,
    0x03a2d32c_2e9e_54a3_b084_1479d5060f80
);
impl windows_core::RuntimeType for IWidgetResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponse",
    );
}
impl windows_core::RuntimeName for IWidgetResourceResponse {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetResourceResponse";
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetResourceResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Content: usize,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub StatusCode:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceResponseFactory,
    IWidgetResourceResponseFactory_Vtbl,
    0x08881ef1_a78a_5804_b070_9153a8657f85
);
impl windows_core::RuntimeType for IWidgetResourceResponseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponseFactory",
    );
}
impl windows_core::RuntimeName for IWidgetResourceResponseFactory {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetResourceResponseFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetResourceResponseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptions,
    IWidgetUpdateRequestOptions_Vtbl,
    0xb09ca8f7_7424_5687_baaf_7dd6fa639672
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions",
    );
}
impl windows_core::RuntimeName for IWidgetUpdateRequestOptions {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions";
}
pub trait IWidgetUpdateRequestOptions_Impl: windows_core::IUnknownImpl {
    fn WidgetId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Template(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetTemplate(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Data(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetCustomState(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IWidgetUpdateRequestOptions_Vtbl {
    pub const fn new<Identity: IWidgetUpdateRequestOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidgetId<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptions_Impl::WidgetId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Template<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptions_Impl::Template(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTemplate<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetUpdateRequestOptions_Impl::SetTemplate(this, core::mem::transmute(&value))
                    .into()
            }
        }
        unsafe extern "system" fn Data<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptions_Impl::Data(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetUpdateRequestOptions_Impl::SetData(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn CustomState<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptions_Impl::CustomState(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCustomState<
            Identity: IWidgetUpdateRequestOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetUpdateRequestOptions_Impl::SetCustomState(this, core::mem::transmute(&value))
                    .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetUpdateRequestOptions,
                OFFSET,
            >(),
            WidgetId: WidgetId::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
            SetTemplate: SetTemplate::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            CustomState: CustomState::<Identity, OFFSET>,
            SetCustomState: SetCustomState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetUpdateRequestOptions as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptions2,
    IWidgetUpdateRequestOptions2_Vtbl,
    0x77c4efc4_38f3_57a5_aba1_f83f257b899e
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions2",
    );
}
impl windows_core::RuntimeName for IWidgetUpdateRequestOptions2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions2";
}
pub trait IWidgetUpdateRequestOptions2_Impl: windows_core::IUnknownImpl {
    fn IsPlaceholderContent(&self) -> windows_core::Result<windows_reference::IReference<bool>>;
    fn SetIsPlaceholderContent(
        &self,
        value: windows_core::Ref<windows_reference::IReference<bool>>,
    ) -> windows_core::Result<()>;
}
impl IWidgetUpdateRequestOptions2_Vtbl {
    pub const fn new<Identity: IWidgetUpdateRequestOptions2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPlaceholderContent<
            Identity: IWidgetUpdateRequestOptions2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptions2_Impl::IsPlaceholderContent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsPlaceholderContent<
            Identity: IWidgetUpdateRequestOptions2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetUpdateRequestOptions2_Impl::SetIsPlaceholderContent(
                    this,
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetUpdateRequestOptions2,
                OFFSET,
            >(),
            IsPlaceholderContent: IsPlaceholderContent::<Identity, OFFSET>,
            SetIsPlaceholderContent: SetIsPlaceholderContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetUpdateRequestOptions2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPlaceholderContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetIsPlaceholderContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptions3,
    IWidgetUpdateRequestOptions3_Vtbl,
    0xa78e2a8b_a26c_596a_ade3_db8f4c72fe02
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptions3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions3",
    );
}
impl windows_core::RuntimeName for IWidgetUpdateRequestOptions3 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions3";
}
pub trait IWidgetUpdateRequestOptions3_Impl: windows_core::IUnknownImpl {
    fn Rank(&self) -> windows_core::Result<windows_reference::IReference<i32>>;
    fn SetRank(
        &self,
        value: windows_core::Ref<windows_reference::IReference<i32>>,
    ) -> windows_core::Result<()>;
}
impl IWidgetUpdateRequestOptions3_Vtbl {
    pub const fn new<Identity: IWidgetUpdateRequestOptions3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Rank<
            Identity: IWidgetUpdateRequestOptions3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptions3_Impl::Rank(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRank<
            Identity: IWidgetUpdateRequestOptions3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetUpdateRequestOptions3_Impl::SetRank(this, core::mem::transmute_copy(&value))
                    .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetUpdateRequestOptions3,
                OFFSET,
            >(),
            Rank: Rank::<Identity, OFFSET>,
            SetRank: SetRank::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetUpdateRequestOptions3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptions3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Rank: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetRank: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptionsFactory,
    IWidgetUpdateRequestOptionsFactory_Vtbl,
    0xe0e00af8_1d10_57a8_9419_3f568e854daa
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsFactory",
    );
}
impl windows_core::RuntimeName for IWidgetUpdateRequestOptionsFactory {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsFactory";
}
pub trait IWidgetUpdateRequestOptionsFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        widgetId: &windows_core::HSTRING,
    ) -> windows_core::Result<WidgetUpdateRequestOptions>;
}
impl IWidgetUpdateRequestOptionsFactory_Vtbl {
    pub const fn new<Identity: IWidgetUpdateRequestOptionsFactory_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IWidgetUpdateRequestOptionsFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptionsFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute(&widgetid),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetUpdateRequestOptionsFactory,
                OFFSET,
            >(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetUpdateRequestOptionsFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptionsStatics,
    IWidgetUpdateRequestOptionsStatics_Vtbl,
    0x4645b5e3_d332_5d11_82f0_3607e5df6018
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsStatics",
    );
}
impl windows_core::RuntimeName for IWidgetUpdateRequestOptionsStatics {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsStatics";
}
pub trait IWidgetUpdateRequestOptionsStatics_Impl: windows_core::IUnknownImpl {
    fn UnsetValue(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWidgetUpdateRequestOptionsStatics_Vtbl {
    pub const fn new<Identity: IWidgetUpdateRequestOptionsStatics_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn UnsetValue<
            Identity: IWidgetUpdateRequestOptionsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetUpdateRequestOptionsStatics_Impl::UnsetValue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetUpdateRequestOptionsStatics,
                OFFSET,
            >(),
            UnsetValue: UnsetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetUpdateRequestOptionsStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptionsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnsetValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetActionInvokedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetActionInvokedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetActionInvokedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Verb)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetActionInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetActionInvokedArgs>();
}
unsafe impl windows_core::Interface for WidgetActionInvokedArgs {
    type Vtable = <IWidgetActionInvokedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetActionInvokedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetActionInvokedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs";
}
unsafe impl Send for WidgetActionInvokedArgs {}
unsafe impl Sync for WidgetActionInvokedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetAnalyticsInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetAnalyticsInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetAnalyticsInfoReportedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnalyticsJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetAnalyticsInfoReportedArgs>();
}
unsafe impl windows_core::Interface for WidgetAnalyticsInfoReportedArgs {
    type Vtable = <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.WidgetAnalyticsInfoReportedArgs";
}
unsafe impl Send for WidgetAnalyticsInfoReportedArgs {}
unsafe impl Sync for WidgetAnalyticsInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetContext,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetContext {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<super::WidgetSize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsActive)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WidgetContext {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContext>();
}
unsafe impl windows_core::Interface for WidgetContext {
    type Vtable = <IWidgetContext as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetContext {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContext";
}
unsafe impl Send for WidgetContext {}
unsafe impl Sync for WidgetContext {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetContextChangedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetContextChangedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetContextChangedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetContextChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContextChangedArgs>();
}
unsafe impl windows_core::Interface for WidgetContextChangedArgs {
    type Vtable = <IWidgetContextChangedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetContextChangedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetContextChangedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs";
}
unsafe impl Send for WidgetContextChangedArgs {}
unsafe impl Sync for WidgetContextChangedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetCustomizationRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetCustomizationRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetCustomizationRequestedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetCustomizationRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetCustomizationRequestedArgs>();
}
unsafe impl windows_core::Interface for WidgetCustomizationRequestedArgs {
    type Vtable = <IWidgetCustomizationRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IWidgetCustomizationRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetCustomizationRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.WidgetCustomizationRequestedArgs";
}
unsafe impl Send for WidgetCustomizationRequestedArgs {}
unsafe impl Sync for WidgetCustomizationRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetErrorInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetErrorInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetErrorInfoReportedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetErrorInfoReportedArgs>();
}
unsafe impl windows_core::Interface for WidgetErrorInfoReportedArgs {
    type Vtable = <IWidgetErrorInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetErrorInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetErrorInfoReportedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetErrorInfoReportedArgs";
}
unsafe impl Send for WidgetErrorInfoReportedArgs {}
unsafe impl Sync for WidgetErrorInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetInfo {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Template)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn LastUpdateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastUpdateTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IWidgetInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlaceholderContent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Rank(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IWidgetInfo3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rank)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WidgetInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetInfo>();
}
unsafe impl windows_core::Interface for WidgetInfo {
    type Vtable = <IWidgetInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetInfo";
}
unsafe impl Send for WidgetInfo {}
unsafe impl Sync for WidgetInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetManager,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IWidgetManager
);
windows_core::imp::required_hierarchy!(WidgetManager, IWidgetManager2);
impl WidgetManager {
    pub fn UpdateWidget<P0>(&self, widgetupdaterequestoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetUpdateRequestOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateWidget)(
                windows_core::Interface::as_raw(self),
                widgetupdaterequestoptions.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetWidgetIds(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetIds)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(
        &self,
        widgetid: &windows_core::HSTRING,
    ) -> windows_core::Result<WidgetInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfo)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetWidgetInfos(&self) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<WidgetInfo>::set_abi_len(core::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteWidget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
    pub fn SendMessageToContent(
        &self,
        widgetid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWidgetManager2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SendMessageToContent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(widgetid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
    pub fn GetDefault() -> windows_core::Result<Self> {
        Self::IWidgetManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IWidgetManagerStatics<R, F: FnOnce(&IWidgetManagerStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WidgetManager, IWidgetManagerStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WidgetManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetManager>();
}
unsafe impl windows_core::Interface for WidgetManager {
    type Vtable = <IWidgetManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetManager";
}
unsafe impl Send for WidgetManager {}
unsafe impl Sync for WidgetManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetMessageReceivedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetMessageReceivedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetMessageReceivedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetMessageReceivedArgs>();
}
unsafe impl windows_core::Interface for WidgetMessageReceivedArgs {
    type Vtable = <IWidgetMessageReceivedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetMessageReceivedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetMessageReceivedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetMessageReceivedArgs";
}
unsafe impl Send for WidgetMessageReceivedArgs {}
unsafe impl Sync for WidgetMessageReceivedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetResourceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetResourceRequest,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetResourceRequest {
    pub fn Uri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMethod)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Headers(
        &self,
    ) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceRequest>();
}
unsafe impl windows_core::Interface for WidgetResourceRequest {
    type Vtable = <IWidgetResourceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetResourceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetResourceRequest {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetResourceRequest";
}
unsafe impl Send for WidgetResourceRequest {}
unsafe impl Sync for WidgetResourceRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetResourceRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetResourceRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetResourceRequestedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Request(&self) -> windows_core::Result<WidgetResourceRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Response(&self) -> windows_core::Result<WidgetResourceResponse> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Response)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResponse<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetResourceResponse>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResponse)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for WidgetResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceRequestedArgs>();
}
unsafe impl windows_core::Interface for WidgetResourceRequestedArgs {
    type Vtable = <IWidgetResourceRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetResourceRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetResourceRequestedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetResourceRequestedArgs";
}
unsafe impl Send for WidgetResourceRequestedArgs {}
unsafe impl Sync for WidgetResourceRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetResourceResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetResourceResponse,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetResourceResponse {
    pub fn Headers(
        &self,
    ) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReasonPhrase(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReasonPhrase)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    fn IWidgetResourceResponseFactory<
        R,
        F: FnOnce(&IWidgetResourceResponseFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WidgetResourceResponse,
            IWidgetResourceResponseFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WidgetResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceResponse>();
}
unsafe impl windows_core::Interface for WidgetResourceResponse {
    type Vtable = <IWidgetResourceResponse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetResourceResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetResourceResponse {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetResourceResponse";
}
unsafe impl Send for WidgetResourceResponse {}
unsafe impl Sync for WidgetResourceResponse {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetUpdateRequestOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetUpdateRequestOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetUpdateRequestOptions {
    pub fn WidgetId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Template)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTemplate(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTemplate)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetData)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCustomState(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomState)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlaceholderContent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<bool>| r__.Value())
        }
    }
    pub fn SetIsPlaceholderContent(&self, value: Option<bool>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
        let value__ = value.map(<windows_reference::IReference<bool> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(this).SetIsPlaceholderContent)(
                windows_core::Interface::as_raw(this),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn Rank(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IWidgetUpdateRequestOptions3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rank)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
        }
    }
    pub fn SetRank(&self, value: Option<i32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWidgetUpdateRequestOptions3>(self)?;
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(this).SetRank)(
                windows_core::Interface::as_raw(this),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn CreateInstance(widgetid: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IWidgetUpdateRequestOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn UnsetValue() -> windows_core::Result<windows_core::HSTRING> {
        Self::IWidgetUpdateRequestOptionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsetValue)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    fn IWidgetUpdateRequestOptionsFactory<
        R,
        F: FnOnce(&IWidgetUpdateRequestOptionsFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WidgetUpdateRequestOptions,
            IWidgetUpdateRequestOptionsFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWidgetUpdateRequestOptionsStatics<
        R,
        F: FnOnce(&IWidgetUpdateRequestOptionsStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WidgetUpdateRequestOptions,
            IWidgetUpdateRequestOptionsStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WidgetUpdateRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetUpdateRequestOptions>();
}
unsafe impl windows_core::Interface for WidgetUpdateRequestOptions {
    type Vtable = <IWidgetUpdateRequestOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetUpdateRequestOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetUpdateRequestOptions {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions";
}
unsafe impl Send for WidgetUpdateRequestOptions {}
unsafe impl Sync for WidgetUpdateRequestOptions {}
