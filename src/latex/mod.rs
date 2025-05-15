use std::fmt::format;

pub struct LaTeXTable {
    pub lines: Vec<String>,
    pub begin_table_idx: usize,
    pub end_table_idx: usize,
    pub begin_tabular_idx: usize,
    pub end_tabular_idx: usize,
    pub caption_idx: Option<usize>,
    pub header_idx: usize,
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
            if config.vertical_borders.contains(&i) {
                col_fmt.push('|');
            }
            col_fmt.push_str(align);
        }
        if config.vertical_borders.contains(&config.alignment.len()) {
            col_fmt.push('|');
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

        if config.horizontal_borders.contains(&0) {
            lines.push("\\hline".to_string());
        }

        use std::collections::HashMap;

        let data_start_idx = lines.len();
        for (i, row) in table.rows.iter().enumerate() {
            let mut span_map: HashMap<usize, &crate::config::Span> = HashMap::new();
            for s in config.multicolumns.iter().filter(|s| s.row == i) {
                span_map.insert(s.col, s);
            }

            let mut j = 0;
            let mut rendered_row = String::new();

            while j < row.len() {
                if let Some(span) = span_map.get(&j) {
                    let alignment = if let Some(custom) = &span.alignment {
                        let mut custom_align = custom.clone();
                        if config.vertical_borders.contains(&(j + span.span)) {
                            custom_align = format!("{}|", custom_align);
                        }
                        custom_align
                    } else {
                        let mut base = config.alignment[j].clone();
                        if config.vertical_borders.contains(&(j + span.span)) {
                            base = format!("{}|", base);
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
            if config.horizontal_borders.contains(&(i + 1)) {
                lines.push("\\hline".to_string());
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
            header_idx: 0,
            data_start_idx,
            data_end_idx,
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}