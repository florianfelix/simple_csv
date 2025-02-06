pub trait TableExt {
    fn get_ref(&self, row: usize, column: usize) -> Option<&str>;
    fn column_widths(&self, width: usize) -> Vec<u16>;
}

impl TableExt for Vec<Vec<String>> {
    fn get_ref(&self, row: usize, column: usize) -> Option<&str> {
        if let Some(r) = self.get(row) {
            if let Some(c) = r.get(column) {
                Some(c.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }
    fn column_widths(&self, width: usize) -> Vec<u16> {
        let mut widths: Vec<u16> = vec![0; width];

        for r in self.iter() {
            for (column_index, content) in r.iter().enumerate() {
                // let current = widths.get_mut(column_index).unwrap();
                if let Some(current) = widths.get_mut(column_index) {
                    if content.len() as u16 > *current {
                        *current = content.len() as u16;
                    }
                }
            }
        }
        widths
    }
}
