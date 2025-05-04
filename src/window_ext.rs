use crate::data::Position;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub trait WindowExt {
    fn apply_layer_shell(&self, position: Position);
}

impl WindowExt for gtk::Window {
    fn apply_layer_shell(&self, position: Position) {
        if !self.is_layer_window() {
            return; //init_layer_shell warns user if compositor not supported, so just return
        }

        self.set_layer(Layer::Overlay);
        self.auto_exclusive_zone_enable();

        let edges = [
            match position {
                Position::Top => (Edge::Bottom, false),
                Position::Bottom => (Edge::Top, false),
            },
            match position {
                Position::Top => (Edge::Top, true),
                Position::Bottom => (Edge::Bottom, true),
            },
            (Edge::Left, true),
            (Edge::Right, true),
        ];

        for (edge, anchor) in edges {
            self.set_anchor(edge, anchor);
        }
    }
}
