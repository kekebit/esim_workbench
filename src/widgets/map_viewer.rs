use eframe::egui::*;
use image::DynamicImage;

#[derive(Clone)]
struct ImageInfo {
    img_path: String,
    texture: TextureHandle,
}

#[derive(Clone, Debug, Default)]
struct ViewState {
    pub scale: f32,
    pub offset: Vec2,
    pub initialized: bool,
    pub min_scale: f32,
    pub max_scale: f32,
}

#[derive(Clone)]
pub struct MapViewer {
    /// image info ctx
    image_info: Option<ImageInfo>,
    /// image handle error
    error: Option<String>,
    /// view state
    view_state: ViewState,
}

impl Default for MapViewer {
    fn default() -> Self {
        Self {
            image_info: None,
            error: None,
            view_state: ViewState {
                min_scale: 0.3,
                max_scale: 2.0,
                ..ViewState::default()
            },
        }
    }
}

impl MapViewer {
    fn is_new_img(&self, check_path: &String) -> bool {
        if let Some(ImageInfo { img_path, .. }) = &self.image_info
            && check_path == img_path
        {
            false
        } else {
            true
        }
    }

    pub fn load(&mut self, ctx: &Context, img_path: &String) {
        if !self.is_new_img(img_path) {
            return;
        }
        if let Ok(dyn_img) = image::open(&img_path) {
            let color_image = dynamic_image_to_color_image(&dyn_img);
            self.image_info = Some(ImageInfo {
                img_path: img_path.clone(),
                texture: ctx.load_texture("preview", color_image, TextureOptions::LINEAR),
            });
        } else {
            self.error = Some("Error loading image, try another.".to_string());
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        if let Some(error) = &self.error {
            ui.label(error);
        } else if let Some(ImageInfo { texture, .. }) = &self.image_info {
            self.map_canvas_ui(texture.clone(), ui);
        } else {
            ui.label("No image.");
        }
    }

    fn map_canvas_ui(&mut self, tex: TextureHandle, ui: &mut Ui) -> Response {
        let canvas_size = ui.available_size();
        let tex_size = tex.size_vec2();

        let mut view_state = self.view_state.clone();
        // 第一次进入时，自动 fit
        if !view_state.initialized {
            view_state.scale = fit_scale(&tex_size, &canvas_size);
            view_state.offset = Vec2::ZERO;
            view_state.initialized = true;
        }
        // 分配一个可交互区域（整个区域）
        let (rect, response) = ui.allocate_exact_size(canvas_size, Sense::click_and_drag());
        self.view_state = self.handle_view(view_state, (&response, rect), &tex_size, ui);

        let img_size = tex_size * self.view_state.scale;
        let center = rect.center() - img_size / 2.0;
        let img_pos = center + self.view_state.offset;
        let image_rect = Rect::from_min_size(img_pos, img_size);

        let painter = ui.painter();

        painter.rect_stroke(
            image_rect,
            8.0, // 圆角
            Stroke::new(2.0, Color32::BLACK),
            StrokeKind::Outside,
        );
        painter.image(
            tex.id(),
            image_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );

        // 最后绘制表面上的鼠标旁的位置
        if let Some(mouse_pos) = response.hover_pos() {
            if image_rect.contains(mouse_pos) {
                // 计算鼠标在图片rect中对应图片本身的位置（考虑缩放和偏移）
                let uv = (mouse_pos - image_rect.min) / image_rect.size(); // 0~1
                let mouse_pos_in_img = uv * tex_size;
                // 绘制在实际鼠标右上角附近，以带有背景的label绘制

                // === 要显示的文字 ===
                let text = format!("x: {:.0}, y: {:.0}", mouse_pos_in_img.x, mouse_pos_in_img.y);

                let font = FontId::monospace(12.0);
                let text_color = Color32::WHITE;

                // === 计算文字布局（不换行）===
                let galley =
                    ui.fonts_mut(|f| f.layout_no_wrap(text.clone(), font.clone(), text_color));

                // === 背景框大小 ===
                let padding = vec2(6.0, 4.0);
                let bg_rect = Rect::from_min_size(
                    mouse_pos + vec2(12.0, -galley.size().y - 8.0), // 右上角
                    galley.size() + padding * 2.0,
                );
                // === 背景 ===
                painter.rect_filled(bg_rect, 4.0, Color32::from_black_alpha(180));
                // === 文字 ===
                painter.galley(bg_rect.min + padding, galley, Color32::BLACK);
            }
        }

        response
    }

    fn handle_view(
        &self,
        mut view_state: ViewState,
        canvas_interaction_area: (&Response, Rect),
        tex_size: &Vec2,
        ui: &Ui,
    ) -> ViewState {
        let (canvas_resp, canvas_rect) = canvas_interaction_area;
        // 拖拽平移
        if canvas_resp.dragged() {
            view_state.offset += canvas_resp.drag_delta();
        }
        // 处理缩放
        let scroll_delta_y = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta_y != 1.0 && canvas_resp.hovered() {
            let zoom_factor = (scroll_delta_y * 0.001).exp();

            let old_scale = view_state.scale;
            let new_scale =
                (view_state.scale * zoom_factor).clamp(view_state.min_scale, view_state.max_scale);

            let pointer = ui
                .input(|i| i.pointer.hover_pos())
                .unwrap_or(canvas_rect.center());

            // 图片左上角（缩放前）
            let img_size = *tex_size * old_scale;
            let img_pos = canvas_rect.center() - img_size / 2.0 + view_state.offset;

            // 鼠标在图片中的相对位置（0~1）
            let rel = (pointer - img_pos) / img_size;

            // 缩放后，重新计算 offset，使该点仍在鼠标下
            let new_img_size = *tex_size * new_scale;
            let new_img_pos = pointer - rel * new_img_size;

            view_state.offset += new_img_pos - img_pos;
            view_state.scale = new_scale;
        }

        // 处理双击复位
        // 双击复位
        if canvas_resp.double_clicked() {
            println!("Clicked on image viewer");
            view_state.scale = fit_scale(tex_size, &canvas_rect.size());
            view_state.offset = Vec2::ZERO;
        }

        view_state
    }
}

fn dynamic_image_to_color_image(img: &DynamicImage) -> ColorImage {
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba
        .pixels()
        .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    ColorImage {
        size,
        pixels,
        source_size: Vec2::new(size[0] as f32, size[1] as f32),
    }
}

fn fit_scale(texture_size: &Vec2, canvas_size: &Vec2) -> f32 {
    let sx = canvas_size.x / texture_size.x;
    let sy = canvas_size.y / texture_size.y;
    sx.min(sy)
}
