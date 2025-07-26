/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;

use avin_connect::Tinkoff;
use avin_core::Action;

use crate::asset::AssetWidget;
use crate::chart::ChartWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Terminal {
    #[serde(skip)]
    asset_widget: AssetWidget,
    chart_widget: ChartWidget,

    #[serde(skip)]
    is_active_mode: bool,
    #[serde(skip)]
    action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    #[serde(skip)]
    tokio_runtime: tokio::runtime::Runtime,
}
impl Default for Terminal {
    fn default() -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let broker = Tinkoff::new(event_tx.clone());
        let action_tx = broker.get_sender();

        // create tokio runtime
        let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        // start tokio main loop, broker in there
        tokio_runtime.block_on(async {
            start_broker(broker).await;
        });

        Self {
            asset_widget: AssetWidget::new(event_rx, action_tx.clone()),
            chart_widget: ChartWidget::default(),

            is_active_mode: false,
            action_tx,
            tokio_runtime,
        }
    }
}
impl Terminal {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Called once before the first frame.
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY)
                .unwrap_or_default();
        }

        Terminal::default()
    }
}
impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        ui_top(self, ctx);

        ui_left(self, ctx);
        ui_right(self, ctx);

        if self.is_active_mode {
            ctx.request_repaint();
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called by the frame work to save state before shutdown.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn ui_top(app: &mut Terminal, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(16.0);

            egui::widgets::global_theme_preference_buttons(ui);

            ui.separator();
            ui.label("Active:");
            ui.add(toggle(&mut app.is_active_mode));
        });
    });
}
fn ui_left(app: &mut Terminal, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        app.asset_widget.ui(ctx, ui);
    });
}
fn ui_right(app: &mut Terminal, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let asset = app.asset_widget.current_asset();
        app.chart_widget.ui(ui, asset);
    });
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) =
        ui.allocate_exact_size(desired_size, egui::Sense::click());

    // 3. Interact: Time to check for clicks!
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            *on,
            "",
        )
    });

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        // Let's ask for a simple animation from egui.
        // egui keeps track of changes in the boolean associated with the id and
        // returns an animated value in the 0-1 range for how much "on" we are.
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response, *on);
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp(
            (rect.left() + radius)..=(rect.right() - radius),
            how_on,
        );
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter().circle(
            center,
            0.75 * radius,
            visuals.bg_fill,
            visuals.fg_stroke,
        );
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}
// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
// iOS-style toggle switch.
//
// ## Example:
// ``` ignore
// ui.add(toggle(&mut my_bool));
// ```
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}

async fn start_broker(mut broker: Tinkoff) {
    broker.connect().await.unwrap();
    log::debug!(":: Broker connected!");

    broker.create_marketdata_stream().await.unwrap();
    log::debug!(":: Data stream started!");

    broker.create_transactions_stream().await.unwrap();
    log::debug!(":: Transaction stream started!");

    tokio::spawn(async move {
        broker.start().await;
    });
    log::debug!(":: Broker started!");
}
