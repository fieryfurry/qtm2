// SPDX-License-Identifier: BSD-2-Clause-Patent

use eframe::egui::{vec2, Align, Id, Layout, Rect, Response, ScrollArea, Sense, Ui};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum ColumnWidthType {
    Fixed(f32),
    // fixed column width
    #[default]
    Remainder, // divide all remaining widths between remainders
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Column {
    width_type: ColumnWidthType,
    layout: Option<Layout>,
}

impl Column {
    pub fn fixed(width: f32) -> Self {
        Self {
            width_type: ColumnWidthType::Fixed(width),
            layout: None,
        }
    }

    pub fn remainder() -> Self {
        Self {
            width_type: ColumnWidthType::Remainder,
            layout: None,
        }
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = Some(layout);
        self
    }
}

pub struct TableBuilder {
    ui: Ui,
    columns: Vec<Column>,
    layout: Layout,
    striped: bool,
    vscroll: bool,
    id: Id,
}

impl TableBuilder {
    pub fn new(ui: &mut Ui) -> Self {
        let layout = Layout::top_down(Align::Min);
        let child_ui = ui.child_ui(ui.available_rect_before_wrap(), layout);
        Self {
            ui: child_ui,
            columns: Vec::new(),
            layout,
            striped: false,
            vscroll: false,
            id: ui.id(),
        }
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn vscroll(mut self, vscroll: bool) -> Self {
        self.vscroll = vscroll;
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    pub fn columns(mut self, column: Column, count: usize) -> Self {
        (0..count).for_each(|_| self.columns.push(column));
        self
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    fn scroll_width(&self) -> f32 {
        if self.vscroll {
            self.ui.spacing().scroll_bar_inner_margin
                + self.ui.spacing().scroll_bar_width
                + self.ui.spacing().scroll_bar_outer_margin
        } else {
            0.
        }
    }

    fn avaialble_width(&self) -> f32 {
        self.ui.available_rect_before_wrap().width() - self.scroll_width()
    }

    /// Return the widths of columns and the total width of the table
    fn widths(&self) -> (Vec<f32>, f32) {
        let available_width = self.avaialble_width();
        let mut remainder_count = 0;
        let mut remaining_width =
            available_width - self.ui.spacing().item_spacing.x * (self.columns.len() - 1) as f32;

        self.columns
            .iter()
            .for_each(|Column { width_type, .. }| match width_type {
                ColumnWidthType::Fixed(width) => remaining_width -= width,
                ColumnWidthType::Remainder => remainder_count += 1,
            });

        (
            self.columns
                .iter()
                .map(|Column { width_type, .. }| match width_type {
                    ColumnWidthType::Fixed(width) => *width,
                    ColumnWidthType::Remainder => {
                        if remaining_width > 0. {
                            remaining_width / remainder_count as f32
                        } else {
                            0.
                        }
                    }
                })
                .collect(),
            self.ui.available_rect_before_wrap().width()
                - if remainder_count == 0 {
                    remaining_width
                } else {
                    0.
                },
        )
    }

    pub fn build(self) -> Table {
        let widths = self.widths();

        Table {
            ui: self.ui,
            total_width: widths.1,
            column_widths: widths.0,
            column_layouts: self
                .columns
                .iter()
                .map(|Column { layout, .. }| layout.unwrap_or(self.layout))
                .collect(),
            striped: self.striped,
            vscroll: self.vscroll,
            id: self.id,
        }
    }
}

pub struct Table {
    ui: Ui,
    total_width: f32,
    column_widths: Vec<f32>,
    column_layouts: Vec<Layout>,
    striped: bool,
    vscroll: bool,
    id: Id,
}

pub(crate) const SENSE_NONE: Sense = Sense {
    click: false,
    drag: false,
    focusable: false,
};

impl Table {
    pub fn ui_mut(&mut self) -> &mut Ui {
        &mut self.ui
    }

    /// Support multi-row header
    pub fn header(mut self, height: f32, add_header_row: impl FnOnce(TableRow)) -> Self {
        add_header_row(TableRow {
            ui: &mut self.ui.child_ui(
                self.ui.available_rect_before_wrap(),
                Layout::left_to_right(Align::Min),
            ),
            widths: &self.column_widths,
            layout: &self.column_layouts,
            height,
            column_no: 0,
        });
        self.ui
            .allocate_exact_size(vec2(self.total_width, height), SENSE_NONE);
        self.ui.separator();

        Self {
            ui: self.ui,
            total_width: self.total_width,
            column_widths: self.column_widths,
            column_layouts: self.column_layouts,
            striped: self.striped,
            vscroll: self.vscroll,
            id: self.id,
        }
    }

    /// Return the index of the row being selected (if any)
    /// determined by `add_body_contents`
    pub fn body<F>(mut self, add_body_contents: F) -> Option<usize>
    where
        F: for<'b> FnOnce(TableBody<'b>) -> Option<usize>,
    {
        let selected_row = self.ui.data_mut(|d| *d.get_persisted_mut_or(self.id, None));
        let mut new_selected_row = selected_row;
        ScrollArea::neither()
            .vscroll(self.vscroll)
            .auto_shrink([false; 2])
            .show(&mut self.ui, |ui| {
                if let new_selected_row_inner @ Some(_) = add_body_contents(TableBody {
                    ui,
                    total_width: self.total_width,
                    widths: &self.column_widths,
                    layout: &self.column_layouts,
                    striped: self.striped,
                    row_striped: true,
                    row_no: 0,
                    selected_row,
                }) {
                    new_selected_row = new_selected_row_inner;
                };
            });
        if selected_row != new_selected_row {
            self.ui
                .data_mut(|d| d.insert_persisted(self.id, new_selected_row));
        }
        new_selected_row
    }
}

pub struct TableBody<'a> {
    ui: &'a mut Ui,
    total_width: f32,
    widths: &'a [f32],
    layout: &'a [Layout],
    striped: bool,
    row_striped: bool,
    row_no: usize,
    selected_row: Option<usize>,
}

impl<'a> TableBody<'a> {
    pub fn ui_mut(&mut self) -> &mut Ui {
        self.ui
    }

    pub fn row(&mut self, height: f32, add_row_content: impl FnOnce(TableRow)) -> Response {
        let clicked = self.ui.interact(
            Rect::from_min_size(
                self.ui.next_widget_position(),
                vec2(self.total_width, height),
            ),
            self.ui.id().with(self.row_no),
            Sense::click(),
        );
        self.ui.allocate_ui_with_layout(
            vec2(self.total_width, height),
            Layout::left_to_right(Align::Center),
            |ui| {
                if self.selected_row.is_some() && self.selected_row.unwrap() == self.row_no {
                    ui.painter().rect_filled(
                        ui.available_rect_before_wrap(),
                        0.,
                        ui.visuals().extreme_bg_color,
                    );
                } else if self.striped && self.row_striped {
                    ui.painter().rect_filled(
                        ui.available_rect_before_wrap(),
                        0.,
                        ui.visuals().faint_bg_color,
                    );
                }

                add_row_content(TableRow {
                    ui,
                    widths: self.widths,
                    layout: self.layout,
                    height,
                    column_no: 0,
                });

                self.row_striped = !self.row_striped;
                self.row_no += 1;
            },
        );
        clicked
    }
}

pub struct TableRow<'a> {
    ui: &'a mut Ui,
    widths: &'a [f32],
    layout: &'a [Layout],
    height: f32,
    column_no: usize,
}

impl<'a> TableRow<'a> {
    pub fn col(&mut self, add_cell_contents: impl FnOnce(&mut Ui)) {
        if self.column_no >= self.widths.len() {
            panic!("adding more column than specified in table");
        }

        // self.ui.allocate_ui_with_layout(
        //     vec2(self.widths[self.column_no], self.height),
        //     self.layout[self.column_no],
        //     |ui| {
        //         ui.set_min_size(vec2(self.widths[self.column_no], self.height));
        //         add_cell_contents(ui);
        //     },
        // );
        let (rect, _) = self
            .ui
            .allocate_exact_size(vec2(self.widths[self.column_no], self.height), SENSE_NONE);
        let mut ui = self.ui.child_ui(rect, self.layout[self.column_no]);
        let mut old_rect = ui.clip_rect();
        old_rect.max.x = rect.max.x;
        ui.set_clip_rect(old_rect);
        add_cell_contents(&mut ui);

        self.column_no += 1;
    }
}
