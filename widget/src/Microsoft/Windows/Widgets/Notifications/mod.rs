#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnnouncementActionKind(pub i32);
impl AnnouncementActionKind {
    pub const Shown: Self = Self(0);
    pub const Engaged: Self = Self(1);
}
impl windows_core::imp::TypeKind for AnnouncementActionKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for AnnouncementActionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.Widgets.Notifications.AnnouncementActionKind;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.AnnouncementActionKind",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnnouncementTextColor(pub i32);
impl AnnouncementTextColor {
    pub const Default: Self = Self(0);
    pub const Dark: Self = Self(1);
    pub const Light: Self = Self(2);
    pub const Accent: Self = Self(3);
    pub const Good: Self = Self(4);
    pub const Warning: Self = Self(5);
    pub const Attention: Self = Self(6);
}
impl windows_core::imp::TypeKind for AnnouncementTextColor {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for AnnouncementTextColor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.Widgets.Notifications.AnnouncementTextColor;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.AnnouncementTextColor",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedAnnouncement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedAnnouncement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedAnnouncement {
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
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetId)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn PrimaryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPrimaryText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimaryText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn SecondaryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSecondaryText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSecondaryText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn PrimaryTextColor(&self) -> windows_core::Result<AnnouncementTextColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryTextColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPrimaryTextColor(&self, value: AnnouncementTextColor) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimaryTextColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SecondaryTextColor(&self) -> windows_core::Result<AnnouncementTextColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryTextColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSecondaryTextColor(&self, value: AnnouncementTextColor) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSecondaryTextColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CustomAccessibilityText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomAccessibilityText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCustomAccessibilityText(
        &self,
        value: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomAccessibilityText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsSecondaryTextSubtle(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecondaryTextSubtle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSecondaryTextSubtle(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSecondaryTextSubtle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ShowBadgeIfUserNotEngaged(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowBadgeIfUserNotEngaged)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetShowBadgeIfUserNotEngaged(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetShowBadgeIfUserNotEngaged)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpirationTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExpirationTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetExpirationTime)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Duration(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDuration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    fn IFeedAnnouncementFactory<
        R,
        F: FnOnce(&IFeedAnnouncementFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FeedAnnouncement, IFeedAnnouncementFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FeedAnnouncement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnnouncement>();
}
unsafe impl windows_core::Interface for FeedAnnouncement {
    type Vtable = <IFeedAnnouncement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedAnnouncement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedAnnouncement {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Notifications.FeedAnnouncement";
}
unsafe impl Send for FeedAnnouncement {}
unsafe impl Sync for FeedAnnouncement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedAnnouncementInvokedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedAnnouncementInvokedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedAnnouncementInvokedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn AnnouncementId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnnouncementId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActionKind(&self) -> windows_core::Result<AnnouncementActionKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FeedAnnouncementInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnnouncementInvokedArgs>();
}
unsafe impl windows_core::Interface for FeedAnnouncementInvokedArgs {
    type Vtable = <IFeedAnnouncementInvokedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedAnnouncementInvokedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedAnnouncementInvokedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Notifications.FeedAnnouncementInvokedArgs";
}
unsafe impl Send for FeedAnnouncementInvokedArgs {}
unsafe impl Sync for FeedAnnouncementInvokedArgs {}
windows_core::imp::define_interface!(
    IFeedAnnouncement,
    IFeedAnnouncement_Vtbl,
    0xb88e8c2c_d251_5344_acc2_8cf9ba07ec15
);
impl windows_core::RuntimeType for IFeedAnnouncement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncement",
    );
}
impl windows_core::RuntimeName for IFeedAnnouncement {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Notifications.IFeedAnnouncement";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedAnnouncement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PrimaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrimaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SecondaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSecondaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LightModeIconUri: usize,
    SetLightModeIconUri: usize,
    DarkModeIconUri: usize,
    SetDarkModeIconUri: usize,
    pub PrimaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub SetPrimaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub SecondaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub SetSecondaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub CustomAccessibilityText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCustomAccessibilityText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsSecondaryTextSubtle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSecondaryTextSubtle:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShowBadgeIfUserNotEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowBadgeIfUserNotEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnnouncementFactory,
    IFeedAnnouncementFactory_Vtbl,
    0x22074243_46d8_5af2_8715_1c76d1cb774c
);
impl windows_core::RuntimeType for IFeedAnnouncementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementFactory",
    );
}
impl windows_core::RuntimeName for IFeedAnnouncementFactory {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedAnnouncementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFeedAnnouncementInvokedArgs,
    IFeedAnnouncementInvokedArgs_Vtbl,
    0x70a48d98_323d_5f19_a1e1_b63fe36edbf2
);
impl windows_core::RuntimeType for IFeedAnnouncementInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementInvokedArgs",
    );
}
impl windows_core::RuntimeName for IFeedAnnouncementInvokedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementInvokedArgs";
}
pub trait IFeedAnnouncementInvokedArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn AnnouncementId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ActionKind(&self) -> windows_core::Result<AnnouncementActionKind>;
}
impl IFeedAnnouncementInvokedArgs_Vtbl {
    pub const fn new<Identity: IFeedAnnouncementInvokedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedAnnouncementInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnnouncementInvokedArgs_Impl::FeedProviderDefinitionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FeedDefinitionId<
            Identity: IFeedAnnouncementInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnnouncementInvokedArgs_Impl::FeedDefinitionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnnouncementId<
            Identity: IFeedAnnouncementInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnnouncementInvokedArgs_Impl::AnnouncementId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionKind<
            Identity: IFeedAnnouncementInvokedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut AnnouncementActionKind,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnnouncementInvokedArgs_Impl::ActionKind(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFeedAnnouncementInvokedArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
            AnnouncementId: AnnouncementId::<Identity, OFFSET>,
            ActionKind: ActionKind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedAnnouncementInvokedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedAnnouncementInvokedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AnnouncementId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ActionKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AnnouncementActionKind,
    ) -> windows_core::HRESULT,
}
