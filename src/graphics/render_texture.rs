use crate::{
    ffi::graphics as ffi,
    graphics::{
        CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType, RcSprite,
        RcText, RectangleShape, RenderStates, RenderTarget, Sprite, Text, Texture, Vertex,
        VertexBuffer, View,
    },
    system::{Vector2f, Vector2i, Vector2u},
    window::ContextSettings,
};

/// Target for off-screen 2D rendering into a texture
#[derive(Debug)]
pub struct RenderTexture {
    render_texture: *mut ffi::sfRenderTexture,
}

impl RenderTexture {
    /// Construct a new render texture
    ///
    /// # Arguments
    /// * width - Width of the render texture
    /// * height - Height of the render texture
    ///
    /// Returns `None` if creation fails.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Option<RenderTexture> {
        Self::with_settings(width, height, &ContextSettings::default())
    }

    /// Create a `RenderTexture` with the given `ContextSettings`.
    ///
    /// Useful if you want to enable multi-sampling or use the render-texture for
    /// OpenGL rendering that requires a depth or stencil buffer.
    /// Otherwise it is unnecessary, and you should call [`RenderTexture::new`].
    ///
    /// # Parameters
    /// * width - Width of the render-texture
    /// * height - Height of the render-texture
    /// * settings - Additional settings for the underlying OpenGL texture and context
    ///
    /// Returns `None` if creation fails.
    #[must_use]
    pub fn with_settings(width: u32, height: u32, settings: &ContextSettings) -> Option<Self> {
        let tex = unsafe { ffi::sfRenderTexture_createWithSettings(width, height, settings) };
        if tex.is_null() {
            None
        } else {
            Some(Self {
                render_texture: tex,
            })
        }
    }

    /// Update the contents of the target texture
    pub fn display(&self) {
        unsafe { ffi::sfRenderTexture_display(self.render_texture) }
    }

    /// Activate or deactivate a render texture as the current target for rendering
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    pub fn set_active(&mut self, active: bool) -> bool {
        unsafe { ffi::sfRenderTexture_setActive(self.render_texture, active) }
    }

    /// Get the target texture of a render texture
    ///
    /// Return the target texture
    #[must_use]
    pub fn texture(&self) -> &Texture {
        unsafe {
            ffi::sfRenderTexture_getTexture(self.render_texture)
                .as_ref()
                .expect("sfRenderTexture_getTexture failed")
        }
    }

    /// Enable or disable the smooth filter on a render texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfRenderTexture_setSmooth(self.render_texture, smooth) }
    }

    /// Tell whether the smooth filter is enabled or not for a render texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isSmooth(self.render_texture) }
    }
    /// Enable or disable texture repeating.
    ///
    /// This function is similar to `Texture::setRepeated`. This parameter is disabled by default.
    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe { ffi::sfRenderTexture_setRepeated(self.render_texture, repeated) }
    }
    /// Tell whether the texture is repeated or not.
    #[must_use]
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isRepeated(self.render_texture) }
    }
    /// Generate a mipmap using the current texture data.
    ///
    /// This function is similar to [`Texture::generate_mipmap`] and
    /// operates on the texture used as the target for drawing.
    ///
    /// # Safety
    ///
    /// Be aware that any draw operation may modify the base level image data.
    /// For this reason, calling this function only makes sense after all drawing is
    /// completed and display has been called. Not calling display after subsequent drawing
    /// will lead to __undefined behavior__ if a mipmap had been previously generated.
    pub unsafe fn generate_mipmap(&mut self) -> bool {
        ffi::sfRenderTexture_generateMipmap(self.render_texture)
    }

    /// Get the maximum anti-aliasing level supported by the system.
    #[must_use]
    pub fn maximum_antialiasing_level() -> u32 {
        unsafe { ffi::sfRenderTexture_getMaximumAntialiasingLevel() }
    }
}

impl RenderTarget for RenderTexture {
    fn size(&self) -> Vector2u {
        unsafe { ffi::sfRenderTexture_getSize(self.render_texture) }
    }
    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderTexture_clear(self.render_texture, color) }
    }
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderTexture_setView(self.render_texture, view) }
    }
    fn view(&self) -> &View {
        unsafe { &*(ffi::sfRenderTexture_getView(self.render_texture)) }
    }
    fn default_view(&self) -> &View {
        unsafe { &*(ffi::sfRenderTexture_getDefaultView(self.render_texture)) }
    }
    fn viewport(&self, view: &View) -> IntRect {
        unsafe { ffi::sfRenderTexture_getViewport(self.render_texture, view) }
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe { ffi::sfRenderTexture_mapPixelToCoords_View(self.render_texture, point, view) }
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        unsafe { ffi::sfRenderTexture_mapPixelToCoords(self.render_texture, point) }
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe { ffi::sfRenderTexture_mapCoordsToPixel_View(self.render_texture, point, view) }
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        unsafe { ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture, point) }
    }
    fn draw(&mut self, object: &dyn Drawable) {
        object.draw(self, &RenderStates::DEFAULT);
    }
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: &RenderStates) {
        object.draw(self, render_states);
    }
    fn draw_text(&self, text: &Text, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawText(self.render_texture, text.raw(), rs) }
    }
    fn draw_rc_text(&self, text: &RcText, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawText(self.render_texture, text.raw(), rs) }
    }
    fn draw_shape(&self, shape: &CustomShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawShape(self.render_texture, shape.raw(), rs) }
    }
    fn draw_sprite(&self, sprite: &Sprite, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawSprite(self.render_texture, sprite.raw(), rs) }
    }
    fn draw_rc_sprite(&self, sprite: &RcSprite, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawSprite(self.render_texture, sprite.raw(), rs) }
    }
    fn draw_circle_shape(&self, circle_shape: &CircleShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawCircleShape(self.render_texture, circle_shape.raw(), rs) }
    }
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.render_texture, rectangle_shape.raw(), rs)
        }
    }
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawConvexShape(self.render_texture, convex_shape.raw(), rs) }
    }
    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawVertexBuffer(self.render_texture, vertex_buffer.raw(), rs)
        }
    }
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        let len = vertices.len();
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(
                self.render_texture,
                vertices.as_ptr() as *const _,
                len,
                ty.0,
                rs,
            );
        }
    }
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_pushGLStates(self.render_texture) }
    }
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_popGLStates(self.render_texture) }
    }
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_resetGLStates(self.render_texture) }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::sfRenderTexture_destroy(self.render_texture) }
    }
}
