use eframe::egui::*;
use image::DynamicImage;

#[derive(Clone)]
pub struct ImageViewer {
    img_path: Option<String>,
    texture: Option<TextureHandle>,
    error: Option<String>,
    //
    scale: f32,
    offset: Vec2,
    initialized: bool,
    min_scale: f32,
    max_scale: f32,
    mouse_pos: Pos2,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            img_path: None,
            texture: None,
            error: None,
            scale: 0.0,
            offset: Default::default(),
            initialized: false,
            min_scale: 0.5,
            max_scale: 2.0,
            mouse_pos: Default::default(),
        }
    }
}

impl ImageViewer {
    fn is_new_img(&self, img_path: &String) -> bool {
        if let Some(inner_path) = self.img_path.as_ref()
            && inner_path == img_path
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
            self.texture = Some(ctx.load_texture("preview", color_image, TextureOptions::LINEAR));
            self.img_path = Some(img_path.clone());
            self.error = None;
        } else {
            self.error = Some("Error loading image, try another.".to_string());
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        if let Some(error) = &self.error {
            ui.label(error);
        } else if let Some(tex) = &self.texture {
            let img_desc = self.img_path.clone().unwrap_or("img".to_string());
            self.inner_ui(tex.clone(), ui)
                .on_hover_text_at_pointer(img_desc);
        } else {
            ui.label("No image.");
        }
    }

    fn inner_ui(&mut self, tex: TextureHandle, ui: &mut Ui) -> Response {
        let available_rect = ui.available_size();
        let tex_size = tex.size_vec2();

        // 第一次进入时，自动 fit
        if !self.initialized {
            self.scale = fit_scale(tex_size, available_rect);
            self.offset = Vec2::ZERO;
            self.initialized = true;
        }

        // 分配一个可交互区域（整个区域）
        let (rect, response) = ui.allocate_exact_size(available_rect, Sense::click_and_drag());
        // 拖拽平移
        if response.dragged() {
            self.offset += response.drag_delta();
        }
        // 滚轮缩放（以鼠标为中心）
        self.handle_zoom(&response, rect, tex_size, ui);
        // 双击复位
        if response.double_clicked() {
            println!("Clicked on image viewer");
            self.scale = fit_scale(tex_size, available_rect);
            self.offset = Vec2::ZERO;
        }

        let img_size = tex_size * self.scale;
        let center = rect.center() - img_size / 2.0;
        let img_pos = center + self.offset;
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

    fn handle_zoom(&mut self, response: &Response, rect: Rect, tex_size: Vec2, ui: &Ui) {
        let scroll_delta_y = ui.input(|i| i.smooth_scroll_delta.y);

        if scroll_delta_y != 1.0 && response.hovered() {
            let zoom_factor = (scroll_delta_y * 0.001).exp();

            let old_scale = self.scale;
            let new_scale = (self.scale * zoom_factor).clamp(self.min_scale, self.max_scale);

            let pointer = ui.input(|i| i.pointer.hover_pos()).unwrap_or(rect.center());

            // 图片左上角（缩放前）
            let img_size = tex_size * old_scale;
            let img_pos = rect.center() - img_size / 2.0 + self.offset;

            // 鼠标在图片中的相对位置（0~1）
            let rel = (pointer - img_pos) / img_size;

            // 缩放后，重新计算 offset，使该点仍在鼠标下
            let new_img_size = tex_size * new_scale;
            let new_img_pos = pointer - rel * new_img_size;

            self.offset += new_img_pos - img_pos;
            self.scale = new_scale;
        }
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

fn fit_scale(tex_size: Vec2, avail: Vec2) -> f32 {
    let sx = avail.x / tex_size.x;
    let sy = avail.y / tex_size.y;
    sx.min(sy)
}
