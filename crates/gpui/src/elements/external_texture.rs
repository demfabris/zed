use crate::{
    App, Bounds, DevicePixels, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, ObjectFit, Pixels, Size, Style, StyleRefinement, Styled, Window, size,
};
use refineable::Refineable;

/// An element that paints a texture created on GPUI's wgpu device.
pub struct ExternalTexture {
    view: wgpu::TextureView,
    size: Size<DevicePixels>,
    object_fit: ObjectFit,
    style: StyleRefinement,
}

/// Creates an element that paints a texture created on GPUI's wgpu device.
pub fn external_texture(texture: wgpu::Texture) -> ExternalTexture {
    let extent = texture.size();
    ExternalTexture {
        view: texture.create_view(&wgpu::TextureViewDescriptor::default()),
        size: size(extent.width.into(), extent.height.into()),
        object_fit: ObjectFit::Contain,
        style: Default::default(),
    }
}

impl ExternalTexture {
    /// Sets how the texture fits within the element's bounds.
    pub fn object_fit(mut self, object_fit: ObjectFit) -> Self {
        self.object_fit = object_fit;
        self
    }
}

impl Element for ExternalTexture {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        _: &mut App,
    ) {
        let bounds = self.object_fit.get_bounds(bounds, self.size);
        window.paint_external_texture(bounds, self.view.clone());
    }
}

impl IntoElement for ExternalTexture {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for ExternalTexture {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
