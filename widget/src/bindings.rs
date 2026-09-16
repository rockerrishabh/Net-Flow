pub mod Microsoft {
    pub mod Windows {
        pub mod Widgets {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct WidgetSize(pub i32);
            impl WidgetSize {
                pub const Small: Self = Self(0);
                pub const Medium: Self = Self(1);
                pub const Large: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for WidgetSize {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for WidgetSize {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Microsoft.Windows.Widgets.WidgetSize;i4)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.Widgets.WidgetSize",
                    );
            }
            pub mod Feeds {
                pub mod Providers {
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct CustomQueryParametersRequestedArgs(windows_core::IUnknown);
                    windows_core::imp::interface_hierarchy!(
                        CustomQueryParametersRequestedArgs,
                        windows_core::IUnknown,
                        windows_core::IInspectable
                    );
                    impl CustomQueryParametersRequestedArgs {
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                ICustomQueryParametersRequestedArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for CustomQueryParametersRequestedArgs {
                        type Vtable = < ICustomQueryParametersRequestedArgs as windows_core::Interface >::Vtable ;
                        const IID: windows_core::GUID =
                            <ICustomQueryParametersRequestedArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for CustomQueryParametersRequestedArgs {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.CustomQueryParametersRequestedArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn CustomQueryParameters(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            F: FnOnce(
                                &ICustomQueryParametersUpdateOptionsFactory,
                            ) -> windows_core::Result<R>,
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                ICustomQueryParametersUpdateOptions,
                            >();
                    }
                    unsafe impl windows_core::Interface for CustomQueryParametersUpdateOptions {
                        type Vtable = < ICustomQueryParametersUpdateOptions as windows_core::Interface >::Vtable ;
                        const IID: windows_core::GUID =
                            <ICustomQueryParametersUpdateOptions as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for CustomQueryParametersUpdateOptions {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.CustomQueryParametersUpdateOptions";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn FeedDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                IFeedAnalyticsInfoReportedArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for FeedAnalyticsInfoReportedArgs {
                        type Vtable =
                            <IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedAnalyticsInfoReportedArgs {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedAnalyticsInfoReportedArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn FeedDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                        const IID: windows_core::GUID =
                            <IFeedDisabledArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedDisabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedDisabledArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn FeedDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                        const IID: windows_core::GUID =
                            <IFeedEnabledArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedEnabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedEnabledArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn FeedDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                IFeedErrorInfoReportedArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for FeedErrorInfoReportedArgs {
                        type Vtable =
                            <IFeedErrorInfoReportedArgs as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedErrorInfoReportedArgs as windows_core::Interface>::IID;
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
                    windows_core::imp::required_hierarchy!(FeedManager, IFeedManager2);
                    impl FeedManager {
                        pub fn GetEnabledFeedProviders(
                            &self,
                        ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>>
                        {
                            unsafe {
                                let mut result__ = core::mem::MaybeUninit::zeroed();
                                (windows_core::Interface::vtable(self).GetEnabledFeedProviders)(
                                    windows_core::Interface::as_raw(self),
                                    windows_core::Array::<FeedProviderInfo>::set_abi_len(
                                        core::mem::transmute(&mut result__),
                                    ),
                                    result__.as_mut_ptr() as *mut _ as _,
                                )
                                .map(|| result__.assume_init())
                            }
                        }
                        pub fn SetCustomQueryParameters<P0>(
                            &self,
                            options: P0,
                        ) -> windows_core::Result<()>
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
                        fn IFeedManagerStatics<
                            R,
                            F: FnOnce(&IFeedManagerStatics) -> windows_core::Result<R>,
                        >(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<
                                FeedManager,
                                IFeedManagerStatics,
                            > = windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    }
                    impl windows_core::RuntimeType for FeedManager {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_class::<Self, IFeedManager>();
                    }
                    unsafe impl windows_core::Interface for FeedManager {
                        type Vtable = <IFeedManager as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedManager as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedManager {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedManager";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn FeedDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                IFeedMessageReceivedArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for FeedMessageReceivedArgs {
                        type Vtable = <IFeedMessageReceivedArgs as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedMessageReceivedArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedMessageReceivedArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedMessageReceivedArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                IFeedProviderDisabledArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for FeedProviderDisabledArgs {
                        type Vtable =
                            <IFeedProviderDisabledArgs as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedProviderDisabledArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedProviderDisabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderDisabledArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                IFeedProviderEnabledArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for FeedProviderEnabledArgs {
                        type Vtable = <IFeedProviderEnabledArgs as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedProviderEnabledArgs as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedProviderEnabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderEnabledArgs";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                        ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>
                        {
                            unsafe {
                                let mut result__ = core::mem::MaybeUninit::zeroed();
                                (windows_core::Interface::vtable(self).EnabledFeedDefinitionIds)(
                                    windows_core::Interface::as_raw(self),
                                    windows_core::Array::<windows_core::HSTRING>::set_abi_len(
                                        core::mem::transmute(&mut result__),
                                    ),
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
                        const IID: windows_core::GUID =
                            <IFeedProviderInfo as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedProviderInfo {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderInfo";
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
                        pub fn SetMethod(
                            &self,
                            value: &windows_core::HSTRING,
                        ) -> windows_core::Result<()> {
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
                            windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceRequest>(
                            );
                    }
                    unsafe impl windows_core::Interface for FeedResourceRequest {
                        type Vtable = <IFeedResourceRequest as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedResourceRequest as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedResourceRequest {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceRequest";
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
                        pub fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                                    windows_core::Interface::as_raw(self),
                                    &mut result__,
                                )
                                .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub fn FeedDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING> {
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
                            windows_core::imp::ConstBuffer::for_class::<
                                Self,
                                IFeedResourceRequestedArgs,
                            >();
                    }
                    unsafe impl windows_core::Interface for FeedResourceRequestedArgs {
                        type Vtable =
                            <IFeedResourceRequestedArgs as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedResourceRequestedArgs as windows_core::Interface>::IID;
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
                                windows_collections::IKeyValuePair<
                                    windows_core::HSTRING,
                                    windows_core::HSTRING,
                                >,
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
                            windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceResponse>(
                            );
                    }
                    unsafe impl windows_core::Interface for FeedResourceResponse {
                        type Vtable = <IFeedResourceResponse as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID =
                            <IFeedResourceResponse as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for FeedResourceResponse {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceResponse";
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
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersRequestedArgs") ;
                    }
                    impl windows_core::RuntimeName for ICustomQueryParametersRequestedArgs {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersRequestedArgs";
                    }
                    pub trait ICustomQueryParametersRequestedArgs_Impl:
                        windows_core::IUnknownImpl
                    {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl ICustomQueryParametersRequestedArgs_Vtbl {
                        pub const fn new<
                            Identity: ICustomQueryParametersRequestedArgs_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: ICustomQueryParametersRequestedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match ICustomQueryParametersRequestedArgs_Impl::FeedProviderDefinitionId (this ,) { Ok (ok__) => { result__ . write (core::mem::transmute_copy (& ok__)) ; core::mem::forget (ok__) ; windows_core::HRESULT (0) } Err (err) => err . into () }
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    ICustomQueryParametersRequestedArgs,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == & < ICustomQueryParametersRequestedArgs as windows_core::Interface >::IID
                        }
                    }
                    #[repr(C)]
                    pub struct ICustomQueryParametersRequestedArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        ICustomQueryParametersUpdateOptions,
                        ICustomQueryParametersUpdateOptions_Vtbl,
                        0x753f1177_4909_568a_b070_98a3139205ec
                    );
                    impl windows_core::RuntimeType for ICustomQueryParametersUpdateOptions {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptions") ;
                    }
                    impl windows_core::RuntimeName for ICustomQueryParametersUpdateOptions {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptions";
                    }
                    pub trait ICustomQueryParametersUpdateOptions_Impl:
                        windows_core::IUnknownImpl
                    {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn CustomQueryParameters(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl ICustomQueryParametersUpdateOptions_Vtbl {
                        pub const fn new<
                            Identity: ICustomQueryParametersUpdateOptions_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: ICustomQueryParametersUpdateOptions_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match ICustomQueryParametersUpdateOptions_Impl::FeedProviderDefinitionId (this ,) { Ok (ok__) => { result__ . write (core::mem::transmute_copy (& ok__)) ; core::mem::forget (ok__) ; windows_core::HRESULT (0) } Err (err) => err . into () }
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match ICustomQueryParametersUpdateOptions_Impl::CustomQueryParameters (this ,) { Ok (ok__) => { result__ . write (core::mem::transmute_copy (& ok__)) ; core::mem::forget (ok__) ; windows_core::HRESULT (0) } Err (err) => err . into () }
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    ICustomQueryParametersUpdateOptions,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                CustomQueryParameters: CustomQueryParameters::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == & < ICustomQueryParametersUpdateOptions as windows_core::Interface >::IID
                        }
                    }
                    #[repr(C)]
                    pub struct ICustomQueryParametersUpdateOptions_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub CustomQueryParameters:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        ICustomQueryParametersUpdateOptionsFactory,
                        ICustomQueryParametersUpdateOptionsFactory_Vtbl,
                        0x34e318cd_3884_53c0_911c_225f32228fae
                    );
                    impl windows_core::RuntimeType for ICustomQueryParametersUpdateOptionsFactory {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptionsFactory") ;
                    }
                    impl windows_core::RuntimeName for ICustomQueryParametersUpdateOptionsFactory {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptionsFactory";
                    }
                    pub trait ICustomQueryParametersUpdateOptionsFactory_Impl:
                        windows_core::IUnknownImpl
                    {
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match ICustomQueryParametersUpdateOptionsFactory_Impl::CreateInstance (this , core::mem::transmute (& feedproviderdefinitionid) , core::mem::transmute (& customqueryparameters) ,) { Ok (ok__) => { result__ . write (core::mem::transmute_copy (& ok__)) ; core::mem::forget (ok__) ; windows_core::HRESULT (0) } Err (err) => err . into () }
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
                            iid == & < ICustomQueryParametersUpdateOptionsFactory as windows_core::Interface >::IID
                        }
                    }
                    #[repr(C)]
                    pub struct ICustomQueryParametersUpdateOptionsFactory_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub CreateInstance: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedAnalyticsInfoReportedArgs,
                        IFeedAnalyticsInfoReportedArgs_Vtbl,
                        0x3c0e3d65_ed47_5b8a_b650_39a7edf18942
                    );
                    impl windows_core::RuntimeType for IFeedAnalyticsInfoReportedArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnalyticsInfoReportedArgs") ;
                    }
                    impl windows_core::RuntimeName for IFeedAnalyticsInfoReportedArgs {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnalyticsInfoReportedArgs";
                    }
                    pub trait IFeedAnalyticsInfoReportedArgs_Impl:
                        windows_core::IUnknownImpl
                    {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
                        fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedAnalyticsInfoReportedArgs_Vtbl {
                        pub const fn new<
                            Identity: IFeedAnalyticsInfoReportedArgs_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedAnalyticsInfoReportedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match IFeedAnalyticsInfoReportedArgs_Impl::FeedProviderDefinitionId (this ,) { Ok (ok__) => { result__ . write (core::mem::transmute_copy (& ok__)) ; core::mem::forget (ok__) ; windows_core::HRESULT (0) } Err (err) => err . into () }
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match IFeedAnalyticsInfoReportedArgs_Impl::FeedDefinitionId(
                                        this,
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
                            unsafe extern "system" fn AnalyticsJson<
                                Identity: IFeedAnalyticsInfoReportedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
                                AnalyticsJson: AnalyticsJson::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedAnalyticsInfoReportedArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub FeedDefinitionId: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub AnalyticsJson: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedAnnouncementInvokedTarget,
                        IFeedAnnouncementInvokedTarget_Vtbl,
                        0x5d44ae2a_072c_4df9_9fe5_34d5d2e9ff63
                    );
                    impl windows_core::RuntimeType for IFeedAnnouncementInvokedTarget {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget") ;
                    }
                    windows_core::imp::interface_hierarchy!(
                        IFeedAnnouncementInvokedTarget,
                        windows_core::IUnknown,
                        windows_core::IInspectable
                    );
                    impl IFeedAnnouncementInvokedTarget {
                        pub fn OnAnnouncementInvoked<P0>(
                            &self,
                            args: P0,
                        ) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<
                                    super::super::Notifications::FeedAnnouncementInvokedArgs,
                                >,
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
                    impl windows_core::RuntimeName for IFeedAnnouncementInvokedTarget {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget";
                    }
                    pub trait IFeedAnnouncementInvokedTarget_Impl:
                        windows_core::IUnknownImpl
                    {
                        fn OnAnnouncementInvoked(
                            &self,
                            args: windows_core::Ref<
                                super::super::Notifications::FeedAnnouncementInvokedArgs,
                            >,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedAnnouncementInvokedTarget_Vtbl {
                        pub const fn new<
                            Identity: IFeedAnnouncementInvokedTarget_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn OnAnnouncementInvoked<
                                Identity: IFeedAnnouncementInvokedTarget_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                args: *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                    pub struct IFeedAnnouncementInvokedTarget_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub OnAnnouncementInvoked:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedDisabledArgs,
                        IFeedDisabledArgs_Vtbl,
                        0x95300612_aca7_53c0_9cf6_d803689132c1
                    );
                    impl windows_core::RuntimeType for IFeedDisabledArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedDisabledArgs",
                            );
                    }
                    impl windows_core::RuntimeName for IFeedDisabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedDisabledArgs";
                    }
                    pub trait IFeedDisabledArgs_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedDisabledArgs_Vtbl {
                        pub const fn new<Identity: IFeedDisabledArgs_Impl, const OFFSET: isize>()
                        -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedDisabledArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedDisabledArgs,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedDisabledArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedDisabledArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub FeedDefinitionId: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedEnabledArgs,
                        IFeedEnabledArgs_Vtbl,
                        0xeff4b2d7_7347_5969_a77d_cac433f0fdae
                    );
                    impl windows_core::RuntimeType for IFeedEnabledArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedEnabledArgs",
                            );
                    }
                    impl windows_core::RuntimeName for IFeedEnabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedEnabledArgs";
                    }
                    pub trait IFeedEnabledArgs_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedEnabledArgs_Vtbl {
                        pub const fn new<Identity: IFeedEnabledArgs_Impl, const OFFSET: isize>()
                        -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedEnabledArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedEnabledArgs,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedEnabledArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedEnabledArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub FeedDefinitionId: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedErrorInfoReportedArgs,
                        IFeedErrorInfoReportedArgs_Vtbl,
                        0x62de018c_161e_52d0_9dbe_aec106fb6600
                    );
                    impl windows_core::RuntimeType for IFeedErrorInfoReportedArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedErrorInfoReportedArgs") ;
                    }
                    impl windows_core::RuntimeName for IFeedErrorInfoReportedArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedErrorInfoReportedArgs";
                    }
                    pub trait IFeedErrorInfoReportedArgs_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
                        fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedErrorInfoReportedArgs_Vtbl {
                        pub const fn new<
                            Identity: IFeedErrorInfoReportedArgs_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedErrorInfoReportedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match IFeedErrorInfoReportedArgs_Impl::FeedProviderDefinitionId(
                                        this,
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
                            unsafe extern "system" fn FeedDefinitionId<
                                Identity: IFeedErrorInfoReportedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
                                ErrorJson: ErrorJson::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedErrorInfoReportedArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedErrorInfoReportedArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub FeedDefinitionId: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub ErrorJson: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedManager,
                        IFeedManager_Vtbl,
                        0x87df6a84_15aa_45cb_8911_5cafab57f723
                    );
                    impl windows_core::RuntimeType for IFeedManager {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
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
                        ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>>
                        {
                            unsafe {
                                let mut result__ = core::mem::MaybeUninit::zeroed();
                                (windows_core::Interface::vtable(self).GetEnabledFeedProviders)(
                                    windows_core::Interface::as_raw(self),
                                    windows_core::Array::<FeedProviderInfo>::set_abi_len(
                                        core::mem::transmute(&mut result__),
                                    ),
                                    result__.as_mut_ptr() as *mut _ as _,
                                )
                                .map(|| result__.assume_init())
                            }
                        }
                        pub fn SetCustomQueryParameters<P0>(
                            &self,
                            options: P0,
                        ) -> windows_core::Result<()>
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
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager";
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
                        pub const fn new<Identity: IFeedManager_Impl, const OFFSET: isize>() -> Self
                        {
                            unsafe extern "system" fn GetEnabledFeedProviders<
                                Identity: IFeedManager_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result_size__: *mut u32,
                                result__: *mut *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedManager_Impl::SetCustomQueryParameters(
                                        this,
                                        core::mem::transmute_copy(&options),
                                    )
                                    .into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedManager,
                                    OFFSET,
                                >(),
                                GetEnabledFeedProviders: GetEnabledFeedProviders::<Identity, OFFSET>,
                                SetCustomQueryParameters: SetCustomQueryParameters::<
                                    Identity,
                                    OFFSET,
                                >,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedManager as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedManager_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub GetEnabledFeedProviders:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut u32,
                                *mut *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub SetCustomQueryParameters:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedManager2,
                        IFeedManager2_Vtbl,
                        0x5838300a_a069_455d_9943_ba078ada00d8
                    );
                    impl windows_core::RuntimeType for IFeedManager2 {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
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
                    impl windows_core::RuntimeName for IFeedManager2 {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager2";
                    }
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
                            announcement: windows_core::Ref<
                                super::super::Notifications::FeedAnnouncement,
                            >,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedManager2_Vtbl {
                        pub const fn new<Identity: IFeedManager2_Impl, const OFFSET: isize>() -> Self
                        {
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedManager2,
                                    OFFSET,
                                >(),
                                SendMessageToContent: SendMessageToContent::<Identity, OFFSET>,
                                TryShowAnnouncement: TryShowAnnouncement::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedManager2 as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedManager2_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub SendMessageToContent:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub TryShowAnnouncement: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedManagerStatics,
                        IFeedManagerStatics_Vtbl,
                        0x4baf5174_77d6_5e2a_94ea_4f14ccdb1f2c
                    );
                    impl windows_core::RuntimeType for IFeedManagerStatics {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManagerStatics",
                            );
                    }
                    impl windows_core::RuntimeName for IFeedManagerStatics {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedManagerStatics";
                    }
                    pub trait IFeedManagerStatics_Impl: windows_core::IUnknownImpl {
                        fn GetDefault(&self) -> windows_core::Result<FeedManager>;
                    }
                    impl IFeedManagerStatics_Vtbl {
                        pub const fn new<
                            Identity: IFeedManagerStatics_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn GetDefault<
                                Identity: IFeedManagerStatics_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedManagerStatics,
                                    OFFSET,
                                >(),
                                GetDefault: GetDefault::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedManagerStatics as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedManagerStatics_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub GetDefault: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedMessageReceivedArgs,
                        IFeedMessageReceivedArgs_Vtbl,
                        0x4ed6ecf9_4c74_5a0b_ae04_bef6dd776f8a
                    );
                    impl windows_core::RuntimeType for IFeedMessageReceivedArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedMessageReceivedArgs") ;
                    }
                    impl windows_core::RuntimeName for IFeedMessageReceivedArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedMessageReceivedArgs";
                    }
                    pub trait IFeedMessageReceivedArgs_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
                        fn Message(&self) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedMessageReceivedArgs_Vtbl {
                        pub const fn new<
                            Identity: IFeedMessageReceivedArgs_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedMessageReceivedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match IFeedMessageReceivedArgs_Impl::FeedProviderDefinitionId(
                                        this,
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
                            unsafe extern "system" fn FeedDefinitionId<
                                Identity: IFeedMessageReceivedArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                FeedDefinitionId: FeedDefinitionId::<Identity, OFFSET>,
                                Message: Message::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedMessageReceivedArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedMessageReceivedArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub FeedDefinitionId: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub Message: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProvider,
                        IFeedProvider_Vtbl,
                        0x7293a12b_0329_458d_ac25_5332be478fde
                    );
                    impl windows_core::RuntimeType for IFeedProvider {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider",
                            );
                    }
                    windows_core::imp::interface_hierarchy!(
                        IFeedProvider,
                        windows_core::IUnknown,
                        windows_core::IInspectable
                    );
                    impl IFeedProvider {
                        pub fn OnFeedProviderEnabled<P0>(
                            &self,
                            args: P0,
                        ) -> windows_core::Result<()>
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
                        pub fn OnFeedProviderDisabled<P0>(
                            &self,
                            args: P0,
                        ) -> windows_core::Result<()>
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
                        pub fn OnCustomQueryParametersRequested<P0>(
                            &self,
                            args: P0,
                        ) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<CustomQueryParametersRequestedArgs>,
                        {
                            unsafe {
                                (windows_core::Interface::vtable(self)
                                    .OnCustomQueryParametersRequested)(
                                    windows_core::Interface::as_raw(self),
                                    args.param().abi(),
                                )
                                .ok()
                            }
                        }
                    }
                    impl windows_core::RuntimeName for IFeedProvider {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider";
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
                        fn OnFeedEnabled(
                            &self,
                            args: windows_core::Ref<FeedEnabledArgs>,
                        ) -> windows_core::Result<()>;
                        fn OnFeedDisabled(
                            &self,
                            args: windows_core::Ref<FeedDisabledArgs>,
                        ) -> windows_core::Result<()>;
                        fn OnCustomQueryParametersRequested(
                            &self,
                            args: windows_core::Ref<CustomQueryParametersRequestedArgs>,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedProvider_Vtbl {
                        pub const fn new<Identity: IFeedProvider_Impl, const OFFSET: isize>() -> Self
                        {
                            unsafe extern "system" fn OnFeedProviderEnabled<
                                Identity: IFeedProvider_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                args: *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProvider_Impl::OnFeedProviderEnabled(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProvider_Impl::OnFeedProviderDisabled(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProvider_Impl::OnFeedEnabled(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProvider_Impl::OnFeedDisabled(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProvider_Impl::OnCustomQueryParametersRequested(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedProvider,
                                    OFFSET,
                                >(),
                                OnFeedProviderEnabled: OnFeedProviderEnabled::<Identity, OFFSET>,
                                OnFeedProviderDisabled: OnFeedProviderDisabled::<Identity, OFFSET>,
                                OnFeedEnabled: OnFeedEnabled::<Identity, OFFSET>,
                                OnFeedDisabled: OnFeedDisabled::<Identity, OFFSET>,
                                OnCustomQueryParametersRequested: OnCustomQueryParametersRequested::<
                                    Identity,
                                    OFFSET,
                                >,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProvider as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProvider_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub OnFeedProviderEnabled:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub OnFeedProviderDisabled:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub OnFeedEnabled: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub OnFeedDisabled: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub OnCustomQueryParametersRequested:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProviderAnalytics,
                        IFeedProviderAnalytics_Vtbl,
                        0xf6885791_3085_4bd7_9cb1_4f1354c3a687
                    );
                    impl windows_core::RuntimeType for IFeedProviderAnalytics {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderAnalytics",
                            );
                    }
                    windows_core::imp::interface_hierarchy!(
                        IFeedProviderAnalytics,
                        windows_core::IUnknown,
                        windows_core::IInspectable
                    );
                    impl IFeedProviderAnalytics {
                        pub fn OnAnalyticsInfoReported<P0>(
                            &self,
                            args: P0,
                        ) -> windows_core::Result<()>
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
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderAnalytics";
                    }
                    pub trait IFeedProviderAnalytics_Impl: windows_core::IUnknownImpl {
                        fn OnAnalyticsInfoReported(
                            &self,
                            args: windows_core::Ref<FeedAnalyticsInfoReportedArgs>,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedProviderAnalytics_Vtbl {
                        pub const fn new<
                            Identity: IFeedProviderAnalytics_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn OnAnalyticsInfoReported<
                                Identity: IFeedProviderAnalytics_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                args: *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProviderAnalytics_Impl::OnAnalyticsInfoReported(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedProviderAnalytics,
                                    OFFSET,
                                >(),
                                OnAnalyticsInfoReported: OnAnalyticsInfoReported::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProviderAnalytics as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProviderAnalytics_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub OnAnalyticsInfoReported:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProviderDisabledArgs,
                        IFeedProviderDisabledArgs_Vtbl,
                        0x19b65aec_e01d_5e8c_ab5f_324212e7cd30
                    );
                    impl windows_core::RuntimeType for IFeedProviderDisabledArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderDisabledArgs") ;
                    }
                    impl windows_core::RuntimeName for IFeedProviderDisabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderDisabledArgs";
                    }
                    pub trait IFeedProviderDisabledArgs_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedProviderDisabledArgs_Vtbl {
                        pub const fn new<
                            Identity: IFeedProviderDisabledArgs_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedProviderDisabledArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match IFeedProviderDisabledArgs_Impl::FeedProviderDefinitionId(
                                        this,
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
                                    IFeedProviderDisabledArgs,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProviderDisabledArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProviderDisabledArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProviderEnabledArgs,
                        IFeedProviderEnabledArgs_Vtbl,
                        0x821fc9af_0de6_5a9b_9ae6_e179117b40e4
                    );
                    impl windows_core::RuntimeType for IFeedProviderEnabledArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderEnabledArgs") ;
                    }
                    impl windows_core::RuntimeName for IFeedProviderEnabledArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderEnabledArgs";
                    }
                    pub trait IFeedProviderEnabledArgs_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                    }
                    impl IFeedProviderEnabledArgs_Vtbl {
                        pub const fn new<
                            Identity: IFeedProviderEnabledArgs_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedProviderEnabledArgs_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    match IFeedProviderEnabledArgs_Impl::FeedProviderDefinitionId(
                                        this,
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
                                    IFeedProviderEnabledArgs,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProviderEnabledArgs as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProviderEnabledArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProviderErrors,
                        IFeedProviderErrors_Vtbl,
                        0x6611e00a_d86c_49a3_9381_b7da67ee62dc
                    );
                    impl windows_core::RuntimeType for IFeedProviderErrors {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
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
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderErrors";
                    }
                    pub trait IFeedProviderErrors_Impl: windows_core::IUnknownImpl {
                        fn OnErrorInfoReported(
                            &self,
                            args: windows_core::Ref<FeedErrorInfoReportedArgs>,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedProviderErrors_Vtbl {
                        pub const fn new<
                            Identity: IFeedProviderErrors_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn OnErrorInfoReported<
                                Identity: IFeedProviderErrors_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                args: *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProviderErrors_Impl::OnErrorInfoReported(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedProviderErrors,
                                    OFFSET,
                                >(),
                                OnErrorInfoReported: OnErrorInfoReported::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProviderErrors as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProviderErrors_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub OnErrorInfoReported: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProviderInfo,
                        IFeedProviderInfo_Vtbl,
                        0x73c37049_3c03_5896_8532_f9dfdaeb723f
                    );
                    impl windows_core::RuntimeType for IFeedProviderInfo {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderInfo",
                            );
                    }
                    impl windows_core::RuntimeName for IFeedProviderInfo {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderInfo";
                    }
                    pub trait IFeedProviderInfo_Impl: windows_core::IUnknownImpl {
                        fn FeedProviderDefinitionId(
                            &self,
                        ) -> windows_core::Result<windows_core::HSTRING>;
                        fn EnabledFeedDefinitionIds(
                            &self,
                        ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>;
                    }
                    impl IFeedProviderInfo_Vtbl {
                        pub const fn new<Identity: IFeedProviderInfo_Impl, const OFFSET: isize>()
                        -> Self {
                            unsafe extern "system" fn FeedProviderDefinitionId<
                                Identity: IFeedProviderInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
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
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedProviderInfo,
                                    OFFSET,
                                >(),
                                FeedProviderDefinitionId: FeedProviderDefinitionId::<
                                    Identity,
                                    OFFSET,
                                >,
                                EnabledFeedDefinitionIds: EnabledFeedDefinitionIds::<
                                    Identity,
                                    OFFSET,
                                >,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProviderInfo as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProviderInfo_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub EnabledFeedDefinitionIds:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut u32,
                                *mut *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedProviderMessage,
                        IFeedProviderMessage_Vtbl,
                        0x60c2442a_4c9d_4880_ba26_caca9e567dd4
                    );
                    impl windows_core::RuntimeType for IFeedProviderMessage {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
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
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderMessage";
                    }
                    pub trait IFeedProviderMessage_Impl: windows_core::IUnknownImpl {
                        fn OnMessageReceived(
                            &self,
                            args: windows_core::Ref<FeedMessageReceivedArgs>,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedProviderMessage_Vtbl {
                        pub const fn new<
                            Identity: IFeedProviderMessage_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn OnMessageReceived<
                                Identity: IFeedProviderMessage_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                args: *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedProviderMessage_Impl::OnMessageReceived(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedProviderMessage,
                                    OFFSET,
                                >(),
                                OnMessageReceived: OnMessageReceived::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedProviderMessage as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedProviderMessage_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub OnMessageReceived: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedResourceProvider,
                        IFeedResourceProvider_Vtbl,
                        0xe1b6266d_88a0_416c_9440_e341cb047cd3
                    );
                    impl windows_core::RuntimeType for IFeedResourceProvider {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
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
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceProvider";
                    }
                    pub trait IFeedResourceProvider_Impl: windows_core::IUnknownImpl {
                        fn OnResourceRequested(
                            &self,
                            args: windows_core::Ref<FeedResourceRequestedArgs>,
                        ) -> windows_core::Result<()>;
                    }
                    impl IFeedResourceProvider_Vtbl {
                        pub const fn new<
                            Identity: IFeedResourceProvider_Impl,
                            const OFFSET: isize,
                        >() -> Self {
                            unsafe extern "system" fn OnResourceRequested<
                                Identity: IFeedResourceProvider_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                args: *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ())
                                        .offset(OFFSET)
                                        as *const Identity);
                                    IFeedResourceProvider_Impl::OnResourceRequested(
                                        this,
                                        core::mem::transmute_copy(&args),
                                    )
                                    .into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<
                                    Identity,
                                    IFeedResourceProvider,
                                    OFFSET,
                                >(),
                                OnResourceRequested: OnResourceRequested::<Identity, OFFSET>,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IFeedResourceProvider as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    pub struct IFeedResourceProvider_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub OnResourceRequested: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedResourceRequest,
                        IFeedResourceRequest_Vtbl,
                        0xe62e479c_e21f_5863_b4c9_df1be2227981
                    );
                    impl windows_core::RuntimeType for IFeedResourceRequest {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequest",
                            );
                    }
                    impl windows_core::RuntimeName for IFeedResourceRequest {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequest";
                    }
                    #[repr(C)]
                    pub struct IFeedResourceRequest_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub Uri: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub Method: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub SetMethod: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedResourceRequestedArgs,
                        IFeedResourceRequestedArgs_Vtbl,
                        0x360eb709_0bc9_52c1_9c70_3c7d413173d8
                    );
                    impl windows_core::RuntimeType for IFeedResourceRequestedArgs {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequestedArgs") ;
                    }
                    impl windows_core::RuntimeName for IFeedResourceRequestedArgs {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequestedArgs";
                    }
                    #[repr(C)]
                    pub struct IFeedResourceRequestedArgs_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub FeedProviderDefinitionId:
                            unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                *mut *mut core::ffi::c_void,
                            )
                                -> windows_core::HRESULT,
                        pub FeedDefinitionId: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub Request: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub Response: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub SetResponse: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedResourceResponse,
                        IFeedResourceResponse_Vtbl,
                        0xf831824e_7aef_53fc_b7ee_32ade675a3ad
                    );
                    impl windows_core::RuntimeType for IFeedResourceResponse {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponse",
                            );
                    }
                    impl windows_core::RuntimeName for IFeedResourceResponse {
                        const NAME: &'static str =
                            "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponse";
                    }
                    #[repr(C)]
                    pub struct IFeedResourceResponse_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        Content: usize,
                        pub Headers: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub SetHeaders: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub ReasonPhrase: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        )
                            -> windows_core::HRESULT,
                        pub StatusCode: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut i32,
                        )
                            -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IFeedResourceResponseFactory,
                        IFeedResourceResponseFactory_Vtbl,
                        0xdb01690d_2547_5d7a_b625_d1629f443c5c
                    );
                    impl windows_core::RuntimeType for IFeedResourceResponseFactory {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_interface::<Self>();
                        const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponseFactory") ;
                    }
                    impl windows_core::RuntimeName for IFeedResourceResponseFactory {
                        const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponseFactory";
                    }
                    #[repr(C)]
                    pub struct IFeedResourceResponseFactory_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                    }
                }
            }
            pub mod Notifications {
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
                    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"enum(Microsoft.Windows.Widgets.Notifications.AnnouncementActionKind;i4)") ;
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"enum(Microsoft.Windows.Widgets.Notifications.AnnouncementTextColor;i4)") ;
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    pub fn SetPrimaryText(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetSecondaryText(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetPrimaryTextColor(
                        &self,
                        value: AnnouncementTextColor,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetPrimaryTextColor)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SecondaryTextColor(
                        &self,
                    ) -> windows_core::Result<AnnouncementTextColor> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).SecondaryTextColor)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetSecondaryTextColor(
                        &self,
                        value: AnnouncementTextColor,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetSecondaryTextColor)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn CustomAccessibilityText(
                        &self,
                    ) -> windows_core::Result<windows_core::HSTRING> {
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
                    pub fn SetIsSecondaryTextSubtle(
                        &self,
                        value: bool,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetShowBadgeIfUserNotEngaged(
                        &self,
                        value: bool,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetExpirationTime(
                        &self,
                        value: windows_time::DateTime,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetDuration(
                        &self,
                        value: windows_time::TimeSpan,
                    ) -> windows_core::Result<()> {
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
                        static SHARED: windows_core::imp::FactoryCache<
                            FeedAnnouncement,
                            IFeedAnnouncementFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for FeedAnnouncement {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnnouncement>();
                }
                unsafe impl windows_core::Interface for FeedAnnouncement {
                    type Vtable = <IFeedAnnouncement as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IFeedAnnouncement as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for FeedAnnouncement {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Notifications.FeedAnnouncement";
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
                    pub fn FeedProviderDefinitionId(
                        &self,
                    ) -> windows_core::Result<windows_core::HSTRING> {
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
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IFeedAnnouncementInvokedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for FeedAnnouncementInvokedArgs {
                    type Vtable = <IFeedAnnouncementInvokedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IFeedAnnouncementInvokedArgs as windows_core::Interface>::IID;
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
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncement",
                        );
                }
                impl windows_core::RuntimeName for IFeedAnnouncement {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Notifications.IFeedAnnouncement";
                }
                #[repr(C)]
                pub struct IFeedAnnouncement_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Id: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub PrimaryText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetPrimaryText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SecondaryText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetSecondaryText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    LightModeIconUri: usize,
                    SetLightModeIconUri: usize,
                    DarkModeIconUri: usize,
                    SetDarkModeIconUri: usize,
                    pub PrimaryTextColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut AnnouncementTextColor,
                    )
                        -> windows_core::HRESULT,
                    pub SetPrimaryTextColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        AnnouncementTextColor,
                    )
                        -> windows_core::HRESULT,
                    pub SecondaryTextColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut AnnouncementTextColor,
                    )
                        -> windows_core::HRESULT,
                    pub SetSecondaryTextColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        AnnouncementTextColor,
                    )
                        -> windows_core::HRESULT,
                    pub CustomAccessibilityText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetCustomAccessibilityText:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    pub IsSecondaryTextSubtle: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut bool,
                    )
                        -> windows_core::HRESULT,
                    pub SetIsSecondaryTextSubtle:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            bool,
                        ) -> windows_core::HRESULT,
                    pub ShowBadgeIfUserNotEngaged:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut bool,
                        ) -> windows_core::HRESULT,
                    pub SetShowBadgeIfUserNotEngaged:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            bool,
                        ) -> windows_core::HRESULT,
                    pub ExpirationTime: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut windows_time::DateTime,
                    )
                        -> windows_core::HRESULT,
                    pub SetExpirationTime: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_time::DateTime,
                    )
                        -> windows_core::HRESULT,
                    pub Duration: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut windows_time::TimeSpan,
                    )
                        -> windows_core::HRESULT,
                    pub SetDuration: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_time::TimeSpan,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IFeedAnnouncementFactory,
                    IFeedAnnouncementFactory_Vtbl,
                    0x22074243_46d8_5af2_8715_1c76d1cb774c
                );
                impl windows_core::RuntimeType for IFeedAnnouncementFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementFactory",
                        );
                }
                impl windows_core::RuntimeName for IFeedAnnouncementFactory {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementFactory";
                }
                #[repr(C)]
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
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementInvokedArgs",
                        );
                }
                impl windows_core::RuntimeName for IFeedAnnouncementInvokedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementInvokedArgs";
                }
                pub trait IFeedAnnouncementInvokedArgs_Impl: windows_core::IUnknownImpl {
                    fn FeedProviderDefinitionId(
                        &self,
                    ) -> windows_core::Result<windows_core::HSTRING>;
                    fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn AnnouncementId(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn ActionKind(&self) -> windows_core::Result<AnnouncementActionKind>;
                }
                impl IFeedAnnouncementInvokedArgs_Vtbl {
                    pub const fn new<
                        Identity: IFeedAnnouncementInvokedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn FeedProviderDefinitionId<
                            Identity: IFeedAnnouncementInvokedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                match IFeedAnnouncementInvokedArgs_Impl::FeedProviderDefinitionId(
                                    this,
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
                        unsafe extern "system" fn FeedDefinitionId<
                            Identity: IFeedAnnouncementInvokedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IFeedAnnouncementInvokedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub FeedProviderDefinitionId:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    pub FeedDefinitionId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub AnnouncementId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub ActionKind: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut AnnouncementActionKind,
                    )
                        -> windows_core::HRESULT,
                }
            }
            pub mod Providers {
                windows_core::imp::define_interface!(
                    IWidgetActionInvokedArgs,
                    IWidgetActionInvokedArgs_Vtbl,
                    0xc593cc57_04b9_52ca_88ad_46fea21ea340
                );
                impl windows_core::RuntimeType for IWidgetActionInvokedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetActionInvokedArgs",
                        );
                }
                impl windows_core::RuntimeName for IWidgetActionInvokedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetActionInvokedArgs";
                }
                pub trait IWidgetActionInvokedArgs_Impl: windows_core::IUnknownImpl {
                    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
                    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn Data(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
                }
                impl IWidgetActionInvokedArgs_Vtbl {
                    pub const fn new<
                        Identity: IWidgetActionInvokedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetActionInvokedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetActionInvokedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Verb: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetAnalyticsInfoReportedArgs,
                    IWidgetAnalyticsInfoReportedArgs_Vtbl,
                    0x1d9e5fb5_2bce_5350_87b1_d63199526639
                );
                impl windows_core::RuntimeType for IWidgetAnalyticsInfoReportedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetAnalyticsInfoReportedArgs",
                        );
                }
                impl windows_core::RuntimeName for IWidgetAnalyticsInfoReportedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetAnalyticsInfoReportedArgs";
                }
                pub trait IWidgetAnalyticsInfoReportedArgs_Impl:
                    windows_core::IUnknownImpl
                {
                    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
                    fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING>;
                }
                impl IWidgetAnalyticsInfoReportedArgs_Vtbl {
                    pub const fn new<
                        Identity: IWidgetAnalyticsInfoReportedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetAnalyticsInfoReportedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetAnalyticsInfoReportedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub AnalyticsJson: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetContext,
                    IWidgetContext_Vtbl,
                    0x903c518b_40bc_5bc6_88f7_af9d81c0cdc1
                );
                impl windows_core::RuntimeType for IWidgetContext {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                        unsafe extern "system" fn Id<
                            Identity: IWidgetContext_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                        unsafe extern "system" fn Size<
                            Identity: IWidgetContext_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut super::WidgetSize,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                match IWidgetContext_Impl::Size(this) {
                                    Ok(ok__) => {
                                        result__.write(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsActive<
                            Identity: IWidgetContext_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetContext,
                                OFFSET,
                            >(),
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
                pub struct IWidgetContext_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Id: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub DefinitionId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Size: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::WidgetSize,
                    )
                        -> windows_core::HRESULT,
                    pub IsActive: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut bool,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetContextChangedArgs,
                    IWidgetContextChangedArgs_Vtbl,
                    0x2c226d54_2252_576b_a197_370b28d25c2f
                );
                impl windows_core::RuntimeType for IWidgetContextChangedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetContextChangedArgs",
                        );
                }
                impl windows_core::RuntimeName for IWidgetContextChangedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetContextChangedArgs";
                }
                pub trait IWidgetContextChangedArgs_Impl: windows_core::IUnknownImpl {
                    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
                }
                impl IWidgetContextChangedArgs_Vtbl {
                    pub const fn new<
                        Identity: IWidgetContextChangedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetContextChangedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetContextChangedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetCustomizationRequestedArgs,
                    IWidgetCustomizationRequestedArgs_Vtbl,
                    0x41dea311_dd9b_5b8b_b493_3a30552116b8
                );
                impl windows_core::RuntimeType for IWidgetCustomizationRequestedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Providers.IWidgetCustomizationRequestedArgs") ;
                }
                impl windows_core::RuntimeName for IWidgetCustomizationRequestedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetCustomizationRequestedArgs";
                }
                pub trait IWidgetCustomizationRequestedArgs_Impl:
                    windows_core::IUnknownImpl
                {
                    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
                    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
                }
                impl IWidgetCustomizationRequestedArgs_Vtbl {
                    pub const fn new<
                        Identity: IWidgetCustomizationRequestedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetCustomizationRequestedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetCustomizationRequestedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetErrorInfoReportedArgs,
                    IWidgetErrorInfoReportedArgs_Vtbl,
                    0x30efa627_b21f_55d5_b91a_b23b4aa13645
                );
                impl windows_core::RuntimeType for IWidgetErrorInfoReportedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetErrorInfoReportedArgs",
                        );
                }
                impl windows_core::RuntimeName for IWidgetErrorInfoReportedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetErrorInfoReportedArgs";
                }
                pub trait IWidgetErrorInfoReportedArgs_Impl: windows_core::IUnknownImpl {
                    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
                    fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING>;
                }
                impl IWidgetErrorInfoReportedArgs_Vtbl {
                    pub const fn new<
                        Identity: IWidgetErrorInfoReportedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetErrorInfoReportedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetErrorInfoReportedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub ErrorJson: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetInfo,
                    IWidgetInfo_Vtbl,
                    0xcea11f42_a020_5db5_89e2_b7dece4ae5cb
                );
                impl windows_core::RuntimeType for IWidgetInfo {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetInfo_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                        unsafe extern "system" fn Template<
                            Identity: IWidgetInfo_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                        unsafe extern "system" fn Data<
                            Identity: IWidgetInfo_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                        unsafe extern "system" fn CustomState<
                            Identity: IWidgetInfo_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetInfo,
                                OFFSET,
                            >(),
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
                pub struct IWidgetInfo_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Template: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub LastUpdateTime: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut windows_time::DateTime,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetInfo2,
                    IWidgetInfo2_Vtbl,
                    0x081b0a6f_d784_5408_bb29_252fef2926d4
                );
                impl windows_core::RuntimeType for IWidgetInfo2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetInfo2,
                                OFFSET,
                            >(),
                            IsPlaceholderContent: IsPlaceholderContent::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetInfo2 as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetInfo2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsPlaceholderContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut bool,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetManager,
                    IWidgetManager_Vtbl,
                    0x71cb10c0_671e_48e3_b995_207940397123
                );
                impl windows_core::RuntimeType for IWidgetManager {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetManager",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetManager,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetManager {
                    pub fn UpdateWidget<P0>(
                        &self,
                        widgetupdaterequestoptions: P0,
                    ) -> windows_core::Result<()>
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
                    pub fn GetWidgetIds(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>
                    {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetIds)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<windows_core::HSTRING>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
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
                    pub fn GetWidgetInfos(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<WidgetInfo>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    pub fn DeleteWidget(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    fn GetWidgetIds(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>;
                    fn GetWidgetInfo(
                        &self,
                        widgetId: &windows_core::HSTRING,
                    ) -> windows_core::Result<WidgetInfo>;
                    fn GetWidgetInfos(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<WidgetInfo>>;
                    fn DeleteWidget(
                        &self,
                        widgetId: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                match IWidgetManager_Impl::GetWidgetInfo(
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
                        unsafe extern "system" fn GetWidgetInfos<
                            Identity: IWidgetManager_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result_size__: *mut u32,
                            result__: *mut *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetManager_Impl::DeleteWidget(
                                    this,
                                    core::mem::transmute(&widgetid),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetManager,
                                OFFSET,
                            >(),
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
                pub struct IWidgetManager_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub UpdateWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub GetWidgetIds: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut u32,
                        *mut *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub GetWidgetInfo: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub GetWidgetInfos: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut u32,
                        *mut *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub DeleteWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetManager2,
                    IWidgetManager2_Vtbl,
                    0x55c65a27_8845_406c_9ee1_1e79f0556bef
                );
                impl windows_core::RuntimeType for IWidgetManager2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetManager2";
                }
                pub trait IWidgetManager2_Impl: windows_core::IUnknownImpl {
                    fn SendMessageToContent(
                        &self,
                        widgetId: &windows_core::HSTRING,
                        message: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetManager2_Vtbl {
                    pub const fn new<Identity: IWidgetManager2_Impl, const OFFSET: isize>() -> Self
                    {
                        unsafe extern "system" fn SendMessageToContent<
                            Identity: IWidgetManager2_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetid: *mut core::ffi::c_void,
                            message: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetManager2_Impl::SendMessageToContent(
                                    this,
                                    core::mem::transmute(&widgetid),
                                    core::mem::transmute(&message),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetManager2,
                                OFFSET,
                            >(),
                            SendMessageToContent: SendMessageToContent::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetManager2 as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetManager2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SendMessageToContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetManagerStatics,
                    IWidgetManagerStatics_Vtbl,
                    0x7f233b06_28e5_5e2b_8c04_a4fa747c28c7
                );
                impl windows_core::RuntimeType for IWidgetManagerStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetManagerStatics",
                        );
                }
                impl windows_core::RuntimeName for IWidgetManagerStatics {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetManagerStatics";
                }
                pub trait IWidgetManagerStatics_Impl: windows_core::IUnknownImpl {
                    fn GetDefault(&self) -> windows_core::Result<WidgetManager>;
                }
                impl IWidgetManagerStatics_Vtbl {
                    pub const fn new<Identity: IWidgetManagerStatics_Impl, const OFFSET: isize>()
                    -> Self {
                        unsafe extern "system" fn GetDefault<
                            Identity: IWidgetManagerStatics_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetManagerStatics,
                                OFFSET,
                            >(),
                            GetDefault: GetDefault::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetManagerStatics as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetManagerStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub GetDefault: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetMessageReceivedArgs,
                    IWidgetMessageReceivedArgs_Vtbl,
                    0x2261cb2b_c741_5f96_9adb_fb3a7667bcb6
                );
                impl windows_core::RuntimeType for IWidgetMessageReceivedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetMessageReceivedArgs",
                        );
                }
                impl windows_core::RuntimeName for IWidgetMessageReceivedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetMessageReceivedArgs";
                }
                pub trait IWidgetMessageReceivedArgs_Impl: windows_core::IUnknownImpl {
                    fn WidgetContext(&self) -> windows_core::Result<WidgetContext>;
                    fn Message(&self) -> windows_core::Result<windows_core::HSTRING>;
                }
                impl IWidgetMessageReceivedArgs_Vtbl {
                    pub const fn new<
                        Identity: IWidgetMessageReceivedArgs_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetContext<
                            Identity: IWidgetMessageReceivedArgs_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetMessageReceivedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Message: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProvider,
                    IWidgetProvider_Vtbl,
                    0x5c5774cc_72a0_452d_b9ed_075c0dd25eed
                );
                impl windows_core::RuntimeType for IWidgetProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    pub fn OnActionInvoked<P0>(
                        &self,
                        actioninvokedargs: P0,
                    ) -> windows_core::Result<()>
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
                    pub fn OnWidgetContextChanged<P0>(
                        &self,
                        contextchangedargs: P0,
                    ) -> windows_core::Result<()>
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
                    pub fn Deactivate(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProvider";
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
                    fn Activate(
                        &self,
                        widgetContext: windows_core::Ref<WidgetContext>,
                    ) -> windows_core::Result<()>;
                    fn Deactivate(
                        &self,
                        widgetId: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProvider_Vtbl {
                    pub const fn new<Identity: IWidgetProvider_Impl, const OFFSET: isize>() -> Self
                    {
                        unsafe extern "system" fn CreateWidget<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetcontext: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::CreateWidget(
                                    this,
                                    core::mem::transmute_copy(&widgetcontext),
                                )
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::OnWidgetContextChanged(
                                    this,
                                    core::mem::transmute_copy(&contextchangedargs),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn Activate<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetcontext: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::Activate(
                                    this,
                                    core::mem::transmute_copy(&widgetcontext),
                                )
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::Deactivate(
                                    this,
                                    core::mem::transmute(&widgetid),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetProvider,
                                OFFSET,
                            >(),
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
                pub struct IWidgetProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub DeleteWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub OnActionInvoked: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub OnWidgetContextChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Activate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Deactivate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProvider2,
                    IWidgetProvider2_Vtbl,
                    0x38c3a963_dd93_479d_9276_04bf84ee1816
                );
                impl windows_core::RuntimeType for IWidgetProvider2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProvider2";
                }
                pub trait IWidgetProvider2_Impl: windows_core::IUnknownImpl {
                    fn OnCustomizationRequested(
                        &self,
                        customizationRequestedArgs: windows_core::Ref<
                            WidgetCustomizationRequestedArgs,
                        >,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProvider2_Vtbl {
                    pub const fn new<Identity: IWidgetProvider2_Impl, const OFFSET: isize>() -> Self
                    {
                        unsafe extern "system" fn OnCustomizationRequested<
                            Identity: IWidgetProvider2_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            customizationrequestedargs: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider2_Impl::OnCustomizationRequested(
                                    this,
                                    core::mem::transmute_copy(&customizationrequestedargs),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetProvider2,
                                OFFSET,
                            >(),
                            OnCustomizationRequested: OnCustomizationRequested::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetProvider2 as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetProvider2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnCustomizationRequested:
                        unsafe extern "system" fn(
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
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProviderAnalytics";
                }
                pub trait IWidgetProviderAnalytics_Impl: windows_core::IUnknownImpl {
                    fn OnAnalyticsInfoReported(
                        &self,
                        args: windows_core::Ref<WidgetAnalyticsInfoReportedArgs>,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProviderAnalytics_Vtbl {
                    pub const fn new<
                        Identity: IWidgetProviderAnalytics_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn OnAnalyticsInfoReported<
                            Identity: IWidgetProviderAnalytics_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            args: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetProviderAnalytics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProviderErrors,
                    IWidgetProviderErrors_Vtbl,
                    0x90c1b5f0_0d3a_4ac6_abb7_c97b367b8fcc
                );
                impl windows_core::RuntimeType for IWidgetProviderErrors {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProviderErrors";
                }
                pub trait IWidgetProviderErrors_Impl: windows_core::IUnknownImpl {
                    fn OnErrorInfoReported(
                        &self,
                        args: windows_core::Ref<WidgetErrorInfoReportedArgs>,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProviderErrors_Vtbl {
                    pub const fn new<Identity: IWidgetProviderErrors_Impl, const OFFSET: isize>()
                    -> Self {
                        unsafe extern "system" fn OnErrorInfoReported<
                            Identity: IWidgetProviderErrors_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            args: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProviderErrors_Impl::OnErrorInfoReported(
                                    this,
                                    core::mem::transmute_copy(&args),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetProviderErrors,
                                OFFSET,
                            >(),
                            OnErrorInfoReported: OnErrorInfoReported::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetProviderErrors as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetProviderErrors_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnErrorInfoReported: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProviderMessage,
                    IWidgetProviderMessage_Vtbl,
                    0xea4dc186_9e24_4b35_a5ef_a9f5df72d6ac
                );
                impl windows_core::RuntimeType for IWidgetProviderMessage {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProviderMessage";
                }
                pub trait IWidgetProviderMessage_Impl: windows_core::IUnknownImpl {
                    fn OnMessageReceived(
                        &self,
                        args: windows_core::Ref<WidgetMessageReceivedArgs>,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProviderMessage_Vtbl {
                    pub const fn new<Identity: IWidgetProviderMessage_Impl, const OFFSET: isize>()
                    -> Self {
                        unsafe extern "system" fn OnMessageReceived<
                            Identity: IWidgetProviderMessage_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            args: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProviderMessage_Impl::OnMessageReceived(
                                    this,
                                    core::mem::transmute_copy(&args),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetProviderMessage,
                                OFFSET,
                            >(),
                            OnMessageReceived: OnMessageReceived::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetProviderMessage as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetProviderMessage_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnMessageReceived: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceProvider,
                    IWidgetResourceProvider_Vtbl,
                    0xdcf328c0_012c_40f5_bb28_3a1c714d027d
                );
                impl windows_core::RuntimeType for IWidgetResourceProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
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
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetResourceProvider";
                }
                pub trait IWidgetResourceProvider_Impl: windows_core::IUnknownImpl {
                    fn OnResourceRequested(
                        &self,
                        args: windows_core::Ref<WidgetResourceRequestedArgs>,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetResourceProvider_Vtbl {
                    pub const fn new<
                        Identity: IWidgetResourceProvider_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn OnResourceRequested<
                            Identity: IWidgetResourceProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            args: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetResourceProvider_Impl::OnResourceRequested(
                                    this,
                                    core::mem::transmute_copy(&args),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetResourceProvider,
                                OFFSET,
                            >(),
                            OnResourceRequested: OnResourceRequested::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetResourceProvider as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetResourceProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnResourceRequested: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceRequest,
                    IWidgetResourceRequest_Vtbl,
                    0x113d249f_82d9_57cb_8cea_9a5291f2fe22
                );
                impl windows_core::RuntimeType for IWidgetResourceRequest {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequest",
                        );
                }
                impl windows_core::RuntimeName for IWidgetResourceRequest {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetResourceRequest";
                }
                #[repr(C)]
                pub struct IWidgetResourceRequest_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Uri: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Method: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetMethod: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    Content: usize,
                    SetContent: usize,
                    pub Headers: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceRequestedArgs,
                    IWidgetResourceRequestedArgs_Vtbl,
                    0x2bb30f4d_0166_58e3_aaf6_31b2ae970bcd
                );
                impl windows_core::RuntimeType for IWidgetResourceRequestedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequestedArgs",
                        );
                }
                impl windows_core::RuntimeName for IWidgetResourceRequestedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetResourceRequestedArgs";
                }
                #[repr(C)]
                pub struct IWidgetResourceRequestedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Request: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Response: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetResponse: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceResponse,
                    IWidgetResourceResponse_Vtbl,
                    0x03a2d32c_2e9e_54a3_b084_1479d5060f80
                );
                impl windows_core::RuntimeType for IWidgetResourceResponse {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponse",
                        );
                }
                impl windows_core::RuntimeName for IWidgetResourceResponse {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetResourceResponse";
                }
                #[repr(C)]
                pub struct IWidgetResourceResponse_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    Content: usize,
                    pub Headers: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub ReasonPhrase: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub StatusCode: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut i32,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceResponseFactory,
                    IWidgetResourceResponseFactory_Vtbl,
                    0x08881ef1_a78a_5804_b070_9153a8657f85
                );
                impl windows_core::RuntimeType for IWidgetResourceResponseFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponseFactory",
                        );
                }
                impl windows_core::RuntimeName for IWidgetResourceResponseFactory {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetResourceResponseFactory";
                }
                #[repr(C)]
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
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions",
                        );
                }
                impl windows_core::RuntimeName for IWidgetUpdateRequestOptions {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions";
                }
                pub trait IWidgetUpdateRequestOptions_Impl: windows_core::IUnknownImpl {
                    fn WidgetId(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn Template(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn SetTemplate(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
                    fn Data(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
                    fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn SetCustomState(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetUpdateRequestOptions_Vtbl {
                    pub const fn new<
                        Identity: IWidgetUpdateRequestOptions_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn WidgetId<
                            Identity: IWidgetUpdateRequestOptions_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetUpdateRequestOptions_Impl::SetTemplate(
                                    this,
                                    core::mem::transmute(&value),
                                )
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetUpdateRequestOptions_Impl::SetData(
                                    this,
                                    core::mem::transmute(&value),
                                )
                                .into()
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetUpdateRequestOptions_Impl::SetCustomState(
                                    this,
                                    core::mem::transmute(&value),
                                )
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
                pub struct IWidgetUpdateRequestOptions_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Template: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetTemplate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetCustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptions2,
                    IWidgetUpdateRequestOptions2_Vtbl,
                    0x77c4efc4_38f3_57a5_aba1_f83f257b899e
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptions2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions2",
                        );
                }
                impl windows_core::RuntimeName for IWidgetUpdateRequestOptions2 {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions2";
                }
                pub trait IWidgetUpdateRequestOptions2_Impl: windows_core::IUnknownImpl {
                    fn IsPlaceholderContent(
                        &self,
                    ) -> windows_core::Result<windows_reference::IReference<bool>>;
                    fn SetIsPlaceholderContent(
                        &self,
                        value: windows_core::Ref<windows_reference::IReference<bool>>,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetUpdateRequestOptions2_Vtbl {
                    pub const fn new<
                        Identity: IWidgetUpdateRequestOptions2_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn IsPlaceholderContent<
                            Identity: IWidgetUpdateRequestOptions2_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                match IWidgetUpdateRequestOptions2_Impl::IsPlaceholderContent(this)
                                {
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
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetUpdateRequestOptions2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsPlaceholderContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetIsPlaceholderContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptionsFactory,
                    IWidgetUpdateRequestOptionsFactory_Vtbl,
                    0xe0e00af8_1d10_57a8_9419_3f568e854daa
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsFactory") ;
                }
                impl windows_core::RuntimeName for IWidgetUpdateRequestOptionsFactory {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsFactory";
                }
                pub trait IWidgetUpdateRequestOptionsFactory_Impl:
                    windows_core::IUnknownImpl
                {
                    fn CreateInstance(
                        &self,
                        widgetId: &windows_core::HSTRING,
                    ) -> windows_core::Result<WidgetUpdateRequestOptions>;
                }
                impl IWidgetUpdateRequestOptionsFactory_Vtbl {
                    pub const fn new<
                        Identity: IWidgetUpdateRequestOptionsFactory_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn CreateInstance<
                            Identity: IWidgetUpdateRequestOptionsFactory_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetid: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetUpdateRequestOptionsFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptionsStatics,
                    IWidgetUpdateRequestOptionsStatics_Vtbl,
                    0x4645b5e3_d332_5d11_82f0_3607e5df6018
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsStatics") ;
                }
                impl windows_core::RuntimeName for IWidgetUpdateRequestOptionsStatics {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsStatics";
                }
                pub trait IWidgetUpdateRequestOptionsStatics_Impl:
                    windows_core::IUnknownImpl
                {
                    fn UnsetValue(&self) -> windows_core::Result<windows_core::HSTRING>;
                }
                impl IWidgetUpdateRequestOptionsStatics_Vtbl {
                    pub const fn new<
                        Identity: IWidgetUpdateRequestOptionsStatics_Impl,
                        const OFFSET: isize,
                    >() -> Self {
                        unsafe extern "system" fn UnsetValue<
                            Identity: IWidgetUpdateRequestOptionsStatics_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
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
                pub struct IWidgetUpdateRequestOptionsStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub UnsetValue: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
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
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetActionInvokedArgs>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetActionInvokedArgs {
                    type Vtable = <IWidgetActionInvokedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetActionInvokedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetActionInvokedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs";
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
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetAnalyticsInfoReportedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetAnalyticsInfoReportedArgs {
                    type Vtable =
                        <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
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
                    const IID: windows_core::GUID =
                        <IWidgetContext as windows_core::Interface>::IID;
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
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContextChangedArgs>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetContextChangedArgs {
                    type Vtable = <IWidgetContextChangedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetContextChangedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetContextChangedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs";
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
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetCustomizationRequestedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetCustomizationRequestedArgs {
                    type Vtable =
                        <IWidgetCustomizationRequestedArgs as windows_core::Interface>::Vtable;
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
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetErrorInfoReportedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetErrorInfoReportedArgs {
                    type Vtable = <IWidgetErrorInfoReportedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetErrorInfoReportedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetErrorInfoReportedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetErrorInfoReportedArgs";
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
                    pub fn UpdateWidget<P0>(
                        &self,
                        widgetupdaterequestoptions: P0,
                    ) -> windows_core::Result<()>
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
                    pub fn GetWidgetIds(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>
                    {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetIds)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<windows_core::HSTRING>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
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
                    pub fn GetWidgetInfos(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<WidgetInfo>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    pub fn DeleteWidget(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    fn IWidgetManagerStatics<
                        R,
                        F: FnOnce(&IWidgetManagerStatics) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            WidgetManager,
                            IWidgetManagerStatics,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for WidgetManager {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetManager>();
                }
                unsafe impl windows_core::Interface for WidgetManager {
                    type Vtable = <IWidgetManager as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetManager as windows_core::Interface>::IID;
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
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetMessageReceivedArgs>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetMessageReceivedArgs {
                    type Vtable = <IWidgetMessageReceivedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetMessageReceivedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetMessageReceivedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetMessageReceivedArgs";
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
                    pub fn SetMethod(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    ) -> windows_core::Result<
                        windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>,
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
                }
                impl windows_core::RuntimeType for WidgetResourceRequest {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceRequest>();
                }
                unsafe impl windows_core::Interface for WidgetResourceRequest {
                    type Vtable = <IWidgetResourceRequest as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetResourceRequest as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetResourceRequest {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetResourceRequest";
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
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetResourceRequestedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetResourceRequestedArgs {
                    type Vtable = <IWidgetResourceRequestedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetResourceRequestedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetResourceRequestedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetResourceRequestedArgs";
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
                    ) -> windows_core::Result<
                        windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>,
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
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceResponse>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetResourceResponse {
                    type Vtable = <IWidgetResourceResponse as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetResourceResponse as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetResourceResponse {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetResourceResponse";
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
                    pub fn SetTemplate(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetData(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
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
                    pub fn SetCustomState(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetCustomState)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
                        let this =
                            &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
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
                    pub fn SetIsPlaceholderContent(
                        &self,
                        value: Option<bool>,
                    ) -> windows_core::Result<()> {
                        let this =
                            &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
                        let value__ =
                            value.map(<windows_reference::IReference<bool> as From<_>>::from);
                        unsafe {
                            (windows_core::Interface::vtable(this).SetIsPlaceholderContent)(
                                windows_core::Interface::as_raw(this),
                                windows_core::Param::param(value__.as_ref()).abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CreateInstance(
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<Self> {
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
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetUpdateRequestOptions,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetUpdateRequestOptions {
                    type Vtable = <IWidgetUpdateRequestOptions as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetUpdateRequestOptions as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetUpdateRequestOptions {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions";
                }
                unsafe impl Send for WidgetUpdateRequestOptions {}
                unsafe impl Sync for WidgetUpdateRequestOptions {}
            }
        }
    }
}
