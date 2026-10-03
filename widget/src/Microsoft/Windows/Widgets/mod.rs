#[cfg(feature = "Windows_Widgets_Feeds")]
pub mod Feeds;
#[cfg(feature = "Windows_Widgets_Notifications")]
pub mod Notifications;
#[cfg(feature = "Windows_Widgets_Providers")]
pub mod Providers;
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
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.Widgets.WidgetSize;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Microsoft.Windows.Widgets.WidgetSize");
}
