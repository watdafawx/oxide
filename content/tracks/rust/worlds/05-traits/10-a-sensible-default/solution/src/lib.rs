#[derive(Debug, PartialEq, Default)]
pub struct Margins {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

/// A new page: no margins at all.
pub fn fresh_margins() -> Margins {
    Margins::default()
}

#[derive(Debug, PartialEq)]
pub struct Pen {
    pub width: u32,
    pub color: String,
}

impl Default for Pen {
    fn default() -> Self {
        Pen {
            width: 1,
            color: "black".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn margins_default_to_zero() {
        assert_eq!(fresh_margins(), Margins { top: 0, right: 0, bottom: 0, left: 0 });
    }

    #[test]
    fn pen_defaults_to_thin_black() {
        let pen = Pen::default();
        assert_eq!(pen.width, 1);
        assert_eq!(pen.color, "black");
        let saved: Vec<Pen> = Vec::new();
        let first_saved: Option<Pen> = saved.into_iter().next(); // None: nothing saved yet
        assert_eq!(first_saved.unwrap_or_default(), pen, "generic code uses your Default too");
    }
}
