pub struct LaTeXTable {
    pub lines: Vec<String>,
    pub begin_table_idx: usize,
    pub end_table_idx: usize,
    pub begin_tabular_idx: usize,
    pub end_tabular_idx: usize,
    pub caption_idx: Option<usize>,
    pub data_start_idx: usize,
    pub data_end_idx: usize,
}

impl LaTeXTable {
    pub fn from_table(table: &crate::table::Table, config: &crate::config::TableConfig) -> Self {
        let mut lines = Vec::new();

        lines.push("\\begin{table}[htbp]".to_string());
        let begin_table_idx = lines.len() - 1;

        if let Some(centering) = config.centering {
            if centering {
                lines.push("\\centering".to_string());
            }
        }

        let mut col_fmt = String::new();
        for (i, align) in config.alignment.iter().enumerate() {
            if let Some(vertical_borders) = &config.vertical_borders {
                let count = vertical_borders.iter().filter(|&&v| v == i).count();
                col_fmt.push_str(&"|".repeat(count));
            }
            col_fmt.push_str(align);
        }
        if let Some(vertical_borders) = &config.vertical_borders {
            let count = vertical_borders.iter().filter(|&&v| v == config.alignment.len()).count();
            col_fmt.push_str(&"|".repeat(count));
        }
        if config.alignment.contains(&"X".to_string()) {
            if let Some(width) = &config.width {
                lines.push(format!("\\begin{{tabularx}}{{{}}}{{{}}}", width, col_fmt))
            } else {
                lines.push(format!("\\begin{{tabularx}}{{\\textwidth}}{{{}}}", col_fmt))
            }
        } else { 
            lines.push(format!("\\begin{{tabular}}{{{}}}", col_fmt)); 
        }
        let begin_tabular_idx = lines.len() - 1;

        if let Some(horizontal_borders) = &config.horizontal_borders {
            let count = horizontal_borders.iter().filter(|&&v| v == 0).count();
            for _ in 0..count {
                lines.push("\\hline".to_string());
            }
        }

        use std::collections::HashMap;

        let data_start_idx = lines.len();
        for (i, row) in table.rows.iter().enumerate() {
            let mut span_map: HashMap<usize, &crate::config::Span> = HashMap::new();
            if let Some(multicolumns) = &config.multicolumns {
                for s in multicolumns.iter().filter(|s| s.row == i) {
                    span_map.insert(s.col, s);
                }
            }

            let mut j = 0;
            let mut rendered_row = String::new();

            while j < row.len() {
                if let Some(span) = span_map.get(&j) {
                    let alignment = if let Some(custom) = &span.alignment {
                        let mut custom_align = custom.clone();
                        if let Some(vertical_borders) = &config.vertical_borders {
                            let count = vertical_borders.iter().filter(|&&v| v == j + span.span).count();
                            if count > 0 {
                                custom_align = format!("{}{}", custom_align, "|".repeat(count));
                            }
                        }
                        custom_align
                    } else {
                        let mut base = config.alignment[j].clone();
                        if let Some(vertical_borders) = &config.vertical_borders {
                            let count = vertical_borders.iter().filter(|&&v| v == j + span.span).count();
                            if count > 0 {
                                base = format!("{}{}", base, "|".repeat(count));
                            }
                        }
                        base
                    };
                    let content = &row[j];
                    rendered_row.push_str(&format!("\\multicolumn{{{}}}{{{}}}{{{}}}", span.span, alignment, content));
                    j += span.span;
                } else {
                    rendered_row.push_str(&row[j]);
                    j += 1;
                }

                if j < row.len() {
                    rendered_row.push_str(" & ");
                }
            }

            lines.push(rendered_row + " \\\\");
            if let Some(horizontal_borders) = &config.horizontal_borders {
                let count = horizontal_borders.iter().filter(|&&v| v == i + 1).count();
                for _ in 0..count {
                    lines.push("\\hline".to_string());
                }
            }
        }
        let data_end_idx = lines.len() - 1;


        if config.alignment.contains(&"X".to_string()) {
            lines.push("\\end{tabularx}".to_string());
        } else {
            lines.push("\\end{tabular}".to_string());
        }
        
        let end_tabular_idx = lines.len() - 1;

        let caption_idx = if let Some(caption) = &config.caption {
            lines.push(format!("\\caption{{{}}}", caption));
            Some(lines.len() - 1)
        } else {
            None
        };

        lines.push("\\end{table}".to_string());
        let end_table_idx = lines.len() - 1;

        Self {
            lines,
            begin_table_idx,
            end_table_idx,
            begin_tabular_idx,
            end_tabular_idx,
            caption_idx,
            data_start_idx,
            data_end_idx,
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}