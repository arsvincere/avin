/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Low,
    Normal,
    Critical,
}

#[derive(Debug)]
pub struct Notice {
    pub title: String,
    pub body: String,
    pub priority: Priority,
}
impl Notice {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        priority: Priority,
    ) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            priority,
        }
    }
}
