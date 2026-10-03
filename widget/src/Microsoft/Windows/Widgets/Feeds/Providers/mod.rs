#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomQueryParametersRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CustomQueryParametersRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CustomQueryParametersRequestedArgs {
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
}
impl windows_core::RuntimeType for CustomQueryParametersRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICustomQueryParametersRequestedArgs>();
}
unsafe impl windows_core::Interface for CustomQueryParametersRequestedArgs {
    type Vtable = <ICustomQueryParametersRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICustomQueryParametersRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomQueryParametersRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.CustomQueryParametersRequestedArgs";
}
unsafe impl Send for CustomQueryParametersRequestedArgs {}
unsafe impl Sync for CustomQueryParametersRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomQueryParametersUpdateOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CustomQueryParametersUpdateOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CustomQueryParametersUpdateOptions {
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
    pub fn CustomQueryParameters(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomQueryParameters)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(
        feedproviderdefinitionid: &windows_core::HSTRING,
        customqueryparameters: &windows_core::HSTRING,
    ) -> windows_core::Result<Self> {
        Self::ICustomQueryParametersUpdateOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(customqueryparameters),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ICustomQueryParametersUpdateOptionsFactory<
        R,
        F: FnOnce(&ICustomQueryParametersUpdateOptionsFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CustomQueryParametersUpdateOptions,
            ICustomQueryParametersUpdateOptionsFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CustomQueryParametersUpdateOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICustomQueryParametersUpdateOptions>();
}
unsafe impl windows_core::Interface for CustomQueryParametersUpdateOptions {
    type Vtable = <ICustomQueryParametersUpdateOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICustomQueryParametersUpdateOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomQueryParametersUpdateOptions {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.CustomQueryParametersUpdateOptions";
}
unsafe impl Send for CustomQueryParametersUpdateOptions {}
unsafe impl Sync for CustomQueryParametersUpdateOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedAnalyticsInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedAnalyticsInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedAnalyticsInfoReportedArgs {
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
impl windows_core::RuntimeType for FeedAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnalyticsInfoReportedArgs>();
}
unsafe impl windows_core::Interface for FeedAnalyticsInfoReportedArgs {
    type Vtable = <IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.FeedAnalyticsInfoReportedArgs";
}
unsafe impl Send for FeedAnalyticsInfoReportedArgs {}
unsafe impl Sync for FeedAnalyticsInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedDisabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedDisabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedDisabledArgs {
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
}
impl windows_core::RuntimeType for FeedDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedDisabledArgs>();
}
unsafe impl windows_core::Interface for FeedDisabledArgs {
    type Vtable = <IFeedDisabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedDisabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedDisabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedDisabledArgs";
}
unsafe impl Send for FeedDisabledArgs {}
unsafe impl Sync for FeedDisabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedEnabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedEnabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedEnabledArgs {
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
}
impl windows_core::RuntimeType for FeedEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedEnabledArgs>();
}
unsafe impl windows_core::Interface for FeedEnabledArgs {
    type Vtable = <IFeedEnabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedEnabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedEnabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedEnabledArgs";
}
unsafe impl Send for FeedEnabledArgs {}
unsafe impl Sync for FeedEnabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedErrorInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedErrorInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedErrorInfoReportedArgs {
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
impl windows_core::RuntimeType for FeedErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedErrorInfoReportedArgs>();
}
unsafe impl windows_core::Interface for FeedErrorInfoReportedArgs {
    type Vtable = <IFeedErrorInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedErrorInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedErrorInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.FeedErrorInfoReportedArgs";
}
unsafe impl Send for FeedErrorInfoReportedArgs {}
unsafe impl Sync for FeedErrorInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedManager,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IFeedManager
);
windows_core::imp::required_hierarchy!(FeedManager, IFeedManager2, IFeedManager3);
impl FeedManager {
    pub fn GetEnabledFeedProviders(
        &self,
    ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetEnabledFeedProviders)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<FeedProviderInfo>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn SetCustomQueryParameters<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomQueryParametersUpdateOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomQueryParameters)(
                windows_core::Interface::as_raw(self),
                options.param().abi(),
            )
            .ok()
        }
    }
    pub fn SendMessageToContent(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFeedManager2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SendMessageToContent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
    #[cfg(feature = "Windows_Widgets_Notifications")]
    pub fn TryShowAnnouncement<P2>(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        announcement: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::Notifications::FeedAnnouncement>,
    {
        let this = &windows_core::Interface::cast::<IFeedManager2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryShowAnnouncement)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                announcement.param().abi(),
            )
            .ok()
        }
    }
    pub fn TryRemoveAnnouncementById(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        announcementid: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFeedManager3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryRemoveAnnouncementById)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                core::mem::transmute_copy(announcementid),
            )
            .ok()
        }
    }
    pub fn GetDefault() -> windows_core::Result<Self> {
        Self::IFeedManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFeedManagerStatics<R, F: FnOnce(&IFeedManagerStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FeedManager, IFeedManagerStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FeedManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedManager>();
}
unsafe impl windows_core::Interface for FeedManager {
    type Vtable = <IFeedManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedManager";
}
unsafe impl Send for FeedManager {}
unsafe impl Sync for FeedManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedMessageReceivedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedMessageReceivedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedMessageReceivedArgs {
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
impl windows_core::RuntimeType for FeedMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedMessageReceivedArgs>();
}
unsafe impl windows_core::Interface for FeedMessageReceivedArgs {
    type Vtable = <IFeedMessageReceivedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedMessageReceivedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedMessageReceivedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedMessageReceivedArgs";
}
unsafe impl Send for FeedMessageReceivedArgs {}
unsafe impl Sync for FeedMessageReceivedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedProviderDisabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedProviderDisabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedProviderDisabledArgs {
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
}
impl windows_core::RuntimeType for FeedProviderDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedProviderDisabledArgs>();
}
unsafe impl windows_core::Interface for FeedProviderDisabledArgs {
    type Vtable = <IFeedProviderDisabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedProviderDisabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedProviderDisabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderDisabledArgs";
}
unsafe impl Send for FeedProviderDisabledArgs {}
unsafe impl Sync for FeedProviderDisabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedProviderEnabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedProviderEnabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedProviderEnabledArgs {
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
}
impl windows_core::RuntimeType for FeedProviderEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedProviderEnabledArgs>();
}
unsafe impl windows_core::Interface for FeedProviderEnabledArgs {
    type Vtable = <IFeedProviderEnabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedProviderEnabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedProviderEnabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderEnabledArgs";
}
unsafe impl Send for FeedProviderEnabledArgs {}
unsafe impl Sync for FeedProviderEnabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedProviderInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedProviderInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedProviderInfo {
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
    pub fn EnabledFeedDefinitionIds(
        &self,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).EnabledFeedDefinitionIds)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for FeedProviderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedProviderInfo>();
}
unsafe impl windows_core::Interface for FeedProviderInfo {
    type Vtable = <IFeedProviderInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedProviderInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedProviderInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderInfo";
}
unsafe impl Send for FeedProviderInfo {}
unsafe impl Sync for FeedProviderInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedResourceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedResourceRequest,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedResourceRequest {
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
}
impl windows_core::RuntimeType for FeedResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceRequest>();
}
unsafe impl windows_core::Interface for FeedResourceRequest {
    type Vtable = <IFeedResourceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedResourceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedResourceRequest {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceRequest";
}
unsafe impl Send for FeedResourceRequest {}
unsafe impl Sync for FeedResourceRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedResourceRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedResourceRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedResourceRequestedArgs {
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
    pub fn Request(&self) -> windows_core::Result<FeedResourceRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Response(&self) -> windows_core::Result<FeedResourceResponse> {
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
        P0: windows_core::Param<FeedResourceResponse>,
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
impl windows_core::RuntimeType for FeedResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceRequestedArgs>();
}
unsafe impl windows_core::Interface for FeedResourceRequestedArgs {
    type Vtable = <IFeedResourceRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedResourceRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedResourceRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceRequestedArgs";
}
unsafe impl Send for FeedResourceRequestedArgs {}
unsafe impl Sync for FeedResourceRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedResourceResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedResourceResponse,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedResourceResponse {
    pub fn Headers(
        &self,
    ) -> windows_core::Result<
        windows_collections::IIterable<
            windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>,
        >,
    > {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetHeaders<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<
                windows_collections::IIterable<
                    windows_collections::IKeyValuePair<
                        windows_core::HSTRING,
                        windows_core::HSTRING,
                    >,
                >,
            >,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeaders)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
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
    fn IFeedResourceResponseFactory<
        R,
        F: FnOnce(&IFeedResourceResponseFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FeedResourceResponse,
            IFeedResourceResponseFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FeedResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceResponse>();
}
unsafe impl windows_core::Interface for FeedResourceResponse {
    type Vtable = <IFeedResourceResponse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedResourceResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedResourceResponse {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceResponse";
}
unsafe impl Send for FeedResourceResponse {}
unsafe impl Sync for FeedResourceResponse {}
windows_core::imp::define_interface!(
    ICustomQueryParametersRequestedArgs,
    ICustomQueryParametersRequestedArgs_Vtbl,
    0xdc2b0cd8_7936_5346_9371_b21484c7d859
);
impl windows_core::RuntimeType for ICustomQueryParametersRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersRequestedArgs",
    );
}
impl windows_core::RuntimeName for ICustomQueryParametersRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersRequestedArgs";
}
pub trait ICustomQueryParametersRequestedArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ICustomQueryParametersRequestedArgs_Vtbl {
    pub const fn new<Identity: ICustomQueryParametersRequestedArgs_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: ICustomQueryParametersRequestedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICustomQueryParametersRequestedArgs_Impl::FeedProviderDefinitionId(this) {
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
                ICustomQueryParametersRequestedArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomQueryParametersRequestedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomQueryParametersRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICustomQueryParametersUpdateOptions,
    ICustomQueryParametersUpdateOptions_Vtbl,
    0x753f1177_4909_568a_b070_98a3139205ec
);
impl windows_core::RuntimeType for ICustomQueryParametersUpdateOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptions",
    );
}
impl windows_core::RuntimeName for ICustomQueryParametersUpdateOptions {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptions";
}
pub trait ICustomQueryParametersUpdateOptions_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CustomQueryParameters(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ICustomQueryParametersUpdateOptions_Vtbl {
    pub const fn new<Identity: ICustomQueryParametersUpdateOptions_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: ICustomQueryParametersUpdateOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICustomQueryParametersUpdateOptions_Impl::FeedProviderDefinitionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CustomQueryParameters<
            Identity: ICustomQueryParametersUpdateOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICustomQueryParametersUpdateOptions_Impl::CustomQueryParameters(this) {
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
                ICustomQueryParametersUpdateOptions,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            CustomQueryParameters: CustomQueryParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomQueryParametersUpdateOptions as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomQueryParametersUpdateOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomQueryParameters: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICustomQueryParametersUpdateOptionsFactory,
    ICustomQueryParametersUpdateOptionsFactory_Vtbl,
    0x34e318cd_3884_53c0_911c_225f32228fae
);
impl windows_core::RuntimeType for ICustomQueryParametersUpdateOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptionsFactory",
    );
}
impl windows_core::RuntimeName for ICustomQueryParametersUpdateOptionsFactory {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptionsFactory";
}
pub trait ICustomQueryParametersUpdateOptionsFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        feedProviderDefinitionId: &windows_core::HSTRING,
        customQueryParameters: &windows_core::HSTRING,
    ) -> windows_core::Result<CustomQueryParametersUpdateOptions>;
}
impl ICustomQueryParametersUpdateOptionsFactory_Vtbl {
    pub const fn new<
        Identity: ICustomQueryParametersUpdateOptionsFactory_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: ICustomQueryParametersUpdateOptionsFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            feedproviderdefinitionid: *mut core::ffi::c_void,
            customqueryparameters: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICustomQueryParametersUpdateOptionsFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute(&feedproviderdefinitionid),
                    core::mem::transmute(&customqueryparameters),
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
                ICustomQueryParametersUpdateOptionsFactory,
                OFFSET,
            >(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomQueryParametersUpdateOptionsFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomQueryParametersUpdateOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnalyticsInfoReportedArgs,
    IFeedAnalyticsInfoReportedArgs_Vtbl,
    0x3c0e3d65_ed47_5b8a_b650_39a7edf18942
);
impl windows_core::RuntimeType for IFeedAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnalyticsInfoReportedArgs",
    );
}
impl windows_core::RuntimeName for IFeedAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnalyticsInfoReportedArgs";
}
pub trait IFeedAnalyticsInfoReportedArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedAnalyticsInfoReportedArgs_Vtbl {
    pub const fn new<Identity: IFeedAnalyticsInfoReportedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedAnalyticsInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnalyticsInfoReportedArgs_Impl::FeedProviderDefinitionId(this) {
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
            Identity: IFeedAnalyticsInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnalyticsInfoReportedArgs_Impl::FeedDefinitionId(this) {
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
            Identity: IFeedAnalyticsInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedAnalyticsInfoReportedArgs_Impl::AnalyticsJson(this) {
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
                IFeedAnalyticsInfoReportedArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
            AnalyticsJson: AnalyticsJson::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedAnalyticsInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AnalyticsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnnouncementInvokedTarget,
    IFeedAnnouncementInvokedTarget_Vtbl,
    0x5d44ae2a_072c_4df9_9fe5_34d5d2e9ff63
);
impl windows_core::RuntimeType for IFeedAnnouncementInvokedTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedAnnouncementInvokedTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedAnnouncementInvokedTarget {
    #[cfg(feature = "Windows_Widgets_Notifications")]
    pub fn OnAnnouncementInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Notifications::FeedAnnouncementInvokedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnAnnouncementInvoked)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Windows_Widgets_Notifications")]
impl windows_core::RuntimeName for IFeedAnnouncementInvokedTarget {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget";
}
#[cfg(feature = "Windows_Widgets_Notifications")]
pub trait IFeedAnnouncementInvokedTarget_Impl: windows_core::IUnknownImpl {
    fn OnAnnouncementInvoked(
        &self,
        args: windows_core::Ref<super::super::Notifications::FeedAnnouncementInvokedArgs>,
    ) -> windows_core::Result<()>;
}
#[cfg(feature = "Windows_Widgets_Notifications")]
impl IFeedAnnouncementInvokedTarget_Vtbl {
    pub const fn new<Identity: IFeedAnnouncementInvokedTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAnnouncementInvoked<
            Identity: IFeedAnnouncementInvokedTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedAnnouncementInvokedTarget_Impl::OnAnnouncementInvoked(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFeedAnnouncementInvokedTarget,
                OFFSET,
            >(),
            OnAnnouncementInvoked: OnAnnouncementInvoked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedAnnouncementInvokedTarget as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedAnnouncementInvokedTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Widgets_Notifications")]
    pub OnAnnouncementInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Widgets_Notifications"))]
    OnAnnouncementInvoked: usize,
}
windows_core::imp::define_interface!(
    IFeedDisabledArgs,
    IFeedDisabledArgs_Vtbl,
    0x95300612_aca7_53c0_9cf6_d803689132c1
);
impl windows_core::RuntimeType for IFeedDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedDisabledArgs",
    );
}
impl windows_core::RuntimeName for IFeedDisabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedDisabledArgs";
}
pub trait IFeedDisabledArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedDisabledArgs_Vtbl {
    pub const fn new<Identity: IFeedDisabledArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedDisabledArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedDisabledArgs_Impl::FeedProviderDefinitionId(this) {
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
            Identity: IFeedDisabledArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedDisabledArgs_Impl::FeedDefinitionId(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedDisabledArgs, OFFSET>(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedDisabledArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedDisabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedEnabledArgs,
    IFeedEnabledArgs_Vtbl,
    0xeff4b2d7_7347_5969_a77d_cac433f0fdae
);
impl windows_core::RuntimeType for IFeedEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedEnabledArgs",
    );
}
impl windows_core::RuntimeName for IFeedEnabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedEnabledArgs";
}
pub trait IFeedEnabledArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedEnabledArgs_Vtbl {
    pub const fn new<Identity: IFeedEnabledArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedEnabledArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedEnabledArgs_Impl::FeedProviderDefinitionId(this) {
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
            Identity: IFeedEnabledArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedEnabledArgs_Impl::FeedDefinitionId(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedEnabledArgs, OFFSET>(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedEnabledArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedEnabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedErrorInfoReportedArgs,
    IFeedErrorInfoReportedArgs_Vtbl,
    0x62de018c_161e_52d0_9dbe_aec106fb6600
);
impl windows_core::RuntimeType for IFeedErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedErrorInfoReportedArgs",
    );
}
impl windows_core::RuntimeName for IFeedErrorInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedErrorInfoReportedArgs";
}
pub trait IFeedErrorInfoReportedArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedErrorInfoReportedArgs_Vtbl {
    pub const fn new<Identity: IFeedErrorInfoReportedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedErrorInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedErrorInfoReportedArgs_Impl::FeedProviderDefinitionId(this) {
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
            Identity: IFeedErrorInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedErrorInfoReportedArgs_Impl::FeedDefinitionId(this) {
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
            Identity: IFeedErrorInfoReportedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedErrorInfoReportedArgs_Impl::ErrorJson(this) {
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
                IFeedErrorInfoReportedArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
            ErrorJson: ErrorJson::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedErrorInfoReportedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedErrorInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ErrorJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedManager,
    IFeedManager_Vtbl,
    0x87df6a84_15aa_45cb_8911_5cafab57f723
);
impl windows_core::RuntimeType for IFeedManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedManager {
    pub fn GetEnabledFeedProviders(
        &self,
    ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetEnabledFeedProviders)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<FeedProviderInfo>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn SetCustomQueryParameters<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomQueryParametersUpdateOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomQueryParameters)(
                windows_core::Interface::as_raw(self),
                options.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IFeedManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager";
}
pub trait IFeedManager_Impl: windows_core::IUnknownImpl {
    fn GetEnabledFeedProviders(
        &self,
    ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>>;
    fn SetCustomQueryParameters(
        &self,
        options: windows_core::Ref<CustomQueryParametersUpdateOptions>,
    ) -> windows_core::Result<()>;
}
impl IFeedManager_Vtbl {
    pub const fn new<Identity: IFeedManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabledFeedProviders<
            Identity: IFeedManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedManager_Impl::GetEnabledFeedProviders(this) {
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
        unsafe extern "system" fn SetCustomQueryParameters<
            Identity: IFeedManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            options: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedManager_Impl::SetCustomQueryParameters(
                    this,
                    core::mem::transmute_copy(&options),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedManager, OFFSET>(),
            GetEnabledFeedProviders: GetEnabledFeedProviders::<Identity, OFFSET>,
            SetCustomQueryParameters: SetCustomQueryParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedManager as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetEnabledFeedProviders: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCustomQueryParameters: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedManager2,
    IFeedManager2_Vtbl,
    0x5838300a_a069_455d_9943_ba078ada00d8
);
impl windows_core::RuntimeType for IFeedManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager2",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedManager2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedManager2 {
    pub fn SendMessageToContent(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SendMessageToContent)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
    #[cfg(feature = "Windows_Widgets_Notifications")]
    pub fn TryShowAnnouncement<P2>(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        announcement: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::Notifications::FeedAnnouncement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).TryShowAnnouncement)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                announcement.param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Windows_Widgets_Notifications")]
impl windows_core::RuntimeName for IFeedManager2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager2";
}
#[cfg(feature = "Windows_Widgets_Notifications")]
pub trait IFeedManager2_Impl: windows_core::IUnknownImpl {
    fn SendMessageToContent(
        &self,
        feedProviderDefinitionId: &windows_core::HSTRING,
        feedDefinitionId: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()>;
    fn TryShowAnnouncement(
        &self,
        feedProviderDefinitionId: &windows_core::HSTRING,
        feedDefinitionId: &windows_core::HSTRING,
        announcement: windows_core::Ref<super::super::Notifications::FeedAnnouncement>,
    ) -> windows_core::Result<()>;
}
#[cfg(feature = "Windows_Widgets_Notifications")]
impl IFeedManager2_Vtbl {
    pub const fn new<Identity: IFeedManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendMessageToContent<
            Identity: IFeedManager2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            feedproviderdefinitionid: *mut core::ffi::c_void,
            feeddefinitionid: *mut core::ffi::c_void,
            message: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedManager2_Impl::SendMessageToContent(
                    this,
                    core::mem::transmute(&feedproviderdefinitionid),
                    core::mem::transmute(&feeddefinitionid),
                    core::mem::transmute(&message),
                )
                .into()
            }
        }
        unsafe extern "system" fn TryShowAnnouncement<
            Identity: IFeedManager2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            feedproviderdefinitionid: *mut core::ffi::c_void,
            feeddefinitionid: *mut core::ffi::c_void,
            announcement: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedManager2_Impl::TryShowAnnouncement(
                    this,
                    core::mem::transmute(&feedproviderdefinitionid),
                    core::mem::transmute(&feeddefinitionid),
                    core::mem::transmute_copy(&announcement),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedManager2, OFFSET>(),
            SendMessageToContent: SendMessageToContent::<Identity, OFFSET>,
            TryShowAnnouncement: TryShowAnnouncement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedManager2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessageToContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    #[cfg(feature = "Windows_Widgets_Notifications")]
    pub TryShowAnnouncement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Widgets_Notifications"))]
    TryShowAnnouncement: usize,
}
windows_core::imp::define_interface!(
    IFeedManager3,
    IFeedManager3_Vtbl,
    0xa6af915b_0cdc_46f1_a4d6_10d8c644984a
);
impl windows_core::RuntimeType for IFeedManager3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager3",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedManager3,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedManager3 {
    pub fn TryRemoveAnnouncementById(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        announcementid: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).TryRemoveAnnouncementById)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                core::mem::transmute_copy(announcementid),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IFeedManager3 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager3";
}
pub trait IFeedManager3_Impl: windows_core::IUnknownImpl {
    fn TryRemoveAnnouncementById(
        &self,
        feedProviderDefinitionId: &windows_core::HSTRING,
        feedDefinitionId: &windows_core::HSTRING,
        announcementId: &windows_core::HSTRING,
    ) -> windows_core::Result<()>;
}
impl IFeedManager3_Vtbl {
    pub const fn new<Identity: IFeedManager3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TryRemoveAnnouncementById<
            Identity: IFeedManager3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            feedproviderdefinitionid: *mut core::ffi::c_void,
            feeddefinitionid: *mut core::ffi::c_void,
            announcementid: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedManager3_Impl::TryRemoveAnnouncementById(
                    this,
                    core::mem::transmute(&feedproviderdefinitionid),
                    core::mem::transmute(&feeddefinitionid),
                    core::mem::transmute(&announcementid),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedManager3, OFFSET>(),
            TryRemoveAnnouncementById: TryRemoveAnnouncementById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedManager3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedManager3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryRemoveAnnouncementById: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedManagerStatics,
    IFeedManagerStatics_Vtbl,
    0x4baf5174_77d6_5e2a_94ea_4f14ccdb1f2c
);
impl windows_core::RuntimeType for IFeedManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManagerStatics",
    );
}
impl windows_core::RuntimeName for IFeedManagerStatics {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManagerStatics";
}
pub trait IFeedManagerStatics_Impl: windows_core::IUnknownImpl {
    fn GetDefault(&self) -> windows_core::Result<FeedManager>;
}
impl IFeedManagerStatics_Vtbl {
    pub const fn new<Identity: IFeedManagerStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDefault<
            Identity: IFeedManagerStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedManagerStatics_Impl::GetDefault(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedManagerStatics, OFFSET>(),
            GetDefault: GetDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedManagerStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedMessageReceivedArgs,
    IFeedMessageReceivedArgs_Vtbl,
    0x4ed6ecf9_4c74_5a0b_ae04_bef6dd776f8a
);
impl windows_core::RuntimeType for IFeedMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedMessageReceivedArgs",
    );
}
impl windows_core::RuntimeName for IFeedMessageReceivedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedMessageReceivedArgs";
}
pub trait IFeedMessageReceivedArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Message(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedMessageReceivedArgs_Vtbl {
    pub const fn new<Identity: IFeedMessageReceivedArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedMessageReceivedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedMessageReceivedArgs_Impl::FeedProviderDefinitionId(this) {
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
            Identity: IFeedMessageReceivedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedMessageReceivedArgs_Impl::FeedDefinitionId(this) {
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
            Identity: IFeedMessageReceivedArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedMessageReceivedArgs_Impl::Message(this) {
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
                IFeedMessageReceivedArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedMessageReceivedArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedMessageReceivedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProvider,
    IFeedProvider_Vtbl,
    0x7293a12b_0329_458d_ac25_5332be478fde
);
impl windows_core::RuntimeType for IFeedProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProvider {
    pub fn OnFeedProviderEnabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedProviderEnabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedProviderEnabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFeedProviderDisabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedProviderDisabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedProviderDisabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFeedEnabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedEnabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedEnabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFeedDisabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedDisabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedDisabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCustomQueryParametersRequested<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomQueryParametersRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnCustomQueryParametersRequested)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IFeedProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider";
}
pub trait IFeedProvider_Impl: windows_core::IUnknownImpl {
    fn OnFeedProviderEnabled(
        &self,
        args: windows_core::Ref<FeedProviderEnabledArgs>,
    ) -> windows_core::Result<()>;
    fn OnFeedProviderDisabled(
        &self,
        args: windows_core::Ref<FeedProviderDisabledArgs>,
    ) -> windows_core::Result<()>;
    fn OnFeedEnabled(&self, args: windows_core::Ref<FeedEnabledArgs>) -> windows_core::Result<()>;
    fn OnFeedDisabled(&self, args: windows_core::Ref<FeedDisabledArgs>)
    -> windows_core::Result<()>;
    fn OnCustomQueryParametersRequested(
        &self,
        args: windows_core::Ref<CustomQueryParametersRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedProvider_Vtbl {
    pub const fn new<Identity: IFeedProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFeedProviderEnabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedProviderEnabled(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnFeedProviderDisabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedProviderDisabled(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnFeedEnabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedEnabled(this, core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn OnFeedDisabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedDisabled(this, core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn OnCustomQueryParametersRequested<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnCustomQueryParametersRequested(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedProvider, OFFSET>(),
            OnFeedProviderEnabled: OnFeedProviderEnabled::<Identity, OFFSET>,
            OnFeedProviderDisabled: OnFeedProviderDisabled::<Identity, OFFSET>,
            OnFeedEnabled: OnFeedEnabled::<Identity, OFFSET>,
            OnFeedDisabled: OnFeedDisabled::<Identity, OFFSET>,
            OnCustomQueryParametersRequested: OnCustomQueryParametersRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnFeedProviderEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFeedProviderDisabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFeedEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFeedDisabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnCustomQueryParametersRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderAnalytics,
    IFeedProviderAnalytics_Vtbl,
    0xf6885791_3085_4bd7_9cb1_4f1354c3a687
);
impl windows_core::RuntimeType for IFeedProviderAnalytics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderAnalytics",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProviderAnalytics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProviderAnalytics {
    pub fn OnAnalyticsInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedAnalyticsInfoReportedArgs>,
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
impl windows_core::RuntimeName for IFeedProviderAnalytics {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderAnalytics";
}
pub trait IFeedProviderAnalytics_Impl: windows_core::IUnknownImpl {
    fn OnAnalyticsInfoReported(
        &self,
        args: windows_core::Ref<FeedAnalyticsInfoReportedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedProviderAnalytics_Vtbl {
    pub const fn new<Identity: IFeedProviderAnalytics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAnalyticsInfoReported<
            Identity: IFeedProviderAnalytics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProviderAnalytics_Impl::OnAnalyticsInfoReported(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedProviderAnalytics, OFFSET>(
            ),
            OnAnalyticsInfoReported: OnAnalyticsInfoReported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProviderAnalytics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProviderAnalytics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderDisabledArgs,
    IFeedProviderDisabledArgs_Vtbl,
    0x19b65aec_e01d_5e8c_ab5f_324212e7cd30
);
impl windows_core::RuntimeType for IFeedProviderDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderDisabledArgs",
    );
}
impl windows_core::RuntimeName for IFeedProviderDisabledArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderDisabledArgs";
}
pub trait IFeedProviderDisabledArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedProviderDisabledArgs_Vtbl {
    pub const fn new<Identity: IFeedProviderDisabledArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedProviderDisabledArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedProviderDisabledArgs_Impl::FeedProviderDefinitionId(this) {
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
                IFeedProviderDisabledArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProviderDisabledArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProviderDisabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderEnabledArgs,
    IFeedProviderEnabledArgs_Vtbl,
    0x821fc9af_0de6_5a9b_9ae6_e179117b40e4
);
impl windows_core::RuntimeType for IFeedProviderEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderEnabledArgs",
    );
}
impl windows_core::RuntimeName for IFeedProviderEnabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderEnabledArgs";
}
pub trait IFeedProviderEnabledArgs_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFeedProviderEnabledArgs_Vtbl {
    pub const fn new<Identity: IFeedProviderEnabledArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedProviderEnabledArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedProviderEnabledArgs_Impl::FeedProviderDefinitionId(this) {
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
                IFeedProviderEnabledArgs,
                OFFSET,
            >(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProviderEnabledArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProviderEnabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderErrors,
    IFeedProviderErrors_Vtbl,
    0x6611e00a_d86c_49a3_9381_b7da67ee62dc
);
impl windows_core::RuntimeType for IFeedProviderErrors {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderErrors",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProviderErrors,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProviderErrors {
    pub fn OnErrorInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedErrorInfoReportedArgs>,
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
impl windows_core::RuntimeName for IFeedProviderErrors {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderErrors";
}
pub trait IFeedProviderErrors_Impl: windows_core::IUnknownImpl {
    fn OnErrorInfoReported(
        &self,
        args: windows_core::Ref<FeedErrorInfoReportedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedProviderErrors_Vtbl {
    pub const fn new<Identity: IFeedProviderErrors_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnErrorInfoReported<
            Identity: IFeedProviderErrors_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProviderErrors_Impl::OnErrorInfoReported(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedProviderErrors, OFFSET>(),
            OnErrorInfoReported: OnErrorInfoReported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProviderErrors as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProviderErrors_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnErrorInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderInfo,
    IFeedProviderInfo_Vtbl,
    0x73c37049_3c03_5896_8532_f9dfdaeb723f
);
impl windows_core::RuntimeType for IFeedProviderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderInfo",
    );
}
impl windows_core::RuntimeName for IFeedProviderInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderInfo";
}
pub trait IFeedProviderInfo_Impl: windows_core::IUnknownImpl {
    fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn EnabledFeedDefinitionIds(
        &self,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>;
}
impl IFeedProviderInfo_Vtbl {
    pub const fn new<Identity: IFeedProviderInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FeedProviderDefinitionId<
            Identity: IFeedProviderInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedProviderInfo_Impl::FeedProviderDefinitionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnabledFeedDefinitionIds<
            Identity: IFeedProviderInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFeedProviderInfo_Impl::EnabledFeedDefinitionIds(this) {
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
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedProviderInfo, OFFSET>(),
            FeedProviderDefinitionId: FeedProviderDefinitionId::<Identity, OFFSET>,
            EnabledFeedDefinitionIds: EnabledFeedDefinitionIds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProviderInfo as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProviderInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EnabledFeedDefinitionIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderMessage,
    IFeedProviderMessage_Vtbl,
    0x60c2442a_4c9d_4880_ba26_caca9e567dd4
);
impl windows_core::RuntimeType for IFeedProviderMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderMessage",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProviderMessage,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProviderMessage {
    pub fn OnMessageReceived<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedMessageReceivedArgs>,
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
impl windows_core::RuntimeName for IFeedProviderMessage {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderMessage";
}
pub trait IFeedProviderMessage_Impl: windows_core::IUnknownImpl {
    fn OnMessageReceived(
        &self,
        args: windows_core::Ref<FeedMessageReceivedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedProviderMessage_Vtbl {
    pub const fn new<Identity: IFeedProviderMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMessageReceived<
            Identity: IFeedProviderMessage_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProviderMessage_Impl::OnMessageReceived(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedProviderMessage, OFFSET>(
            ),
            OnMessageReceived: OnMessageReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProviderMessage as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedProviderMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnMessageReceived: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceProvider,
    IFeedResourceProvider_Vtbl,
    0xe1b6266d_88a0_416c_9440_e341cb047cd3
);
impl windows_core::RuntimeType for IFeedResourceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedResourceProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedResourceProvider {
    pub fn OnResourceRequested<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedResourceRequestedArgs>,
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
impl windows_core::RuntimeName for IFeedResourceProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceProvider";
}
pub trait IFeedResourceProvider_Impl: windows_core::IUnknownImpl {
    fn OnResourceRequested(
        &self,
        args: windows_core::Ref<FeedResourceRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedResourceProvider_Vtbl {
    pub const fn new<Identity: IFeedResourceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnResourceRequested<
            Identity: IFeedResourceProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedResourceProvider_Impl::OnResourceRequested(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedResourceProvider, OFFSET>(
            ),
            OnResourceRequested: OnResourceRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedResourceProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedResourceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnResourceRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceRequest,
    IFeedResourceRequest_Vtbl,
    0xe62e479c_e21f_5863_b4c9_df1be2227981
);
impl windows_core::RuntimeType for IFeedResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequest",
    );
}
impl windows_core::RuntimeName for IFeedResourceRequest {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequest";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedResourceRequest_Vtbl {
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
}
windows_core::imp::define_interface!(
    IFeedResourceRequestedArgs,
    IFeedResourceRequestedArgs_Vtbl,
    0x360eb709_0bc9_52c1_9c70_3c7d413173d8
);
impl windows_core::RuntimeType for IFeedResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequestedArgs",
    );
}
impl windows_core::RuntimeName for IFeedResourceRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequestedArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedResourceRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
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
    IFeedResourceResponse,
    IFeedResourceResponse_Vtbl,
    0xf831824e_7aef_53fc_b7ee_32ade675a3ad
);
impl windows_core::RuntimeType for IFeedResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponse",
    );
}
impl windows_core::RuntimeName for IFeedResourceResponse {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponse";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedResourceResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Content: usize,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetHeaders: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub StatusCode:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceResponseFactory,
    IFeedResourceResponseFactory_Vtbl,
    0xdb01690d_2547_5d7a_b625_d1629f443c5c
);
impl windows_core::RuntimeType for IFeedResourceResponseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponseFactory",
    );
}
impl windows_core::RuntimeName for IFeedResourceResponseFactory {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponseFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedResourceResponseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
