/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#[derive(Debug, Clone, Copy)]
pub enum NoticePriority {
    Low,
    Normal,
    Critical,
}

#[derive(Debug)]
pub struct Notice {
    pub title: String,
    pub body: String,
    pub priority: NoticePriority,
}
impl Notice {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        priority: NoticePriority,
    ) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            priority,
        }
    }
}

pub struct Informer {}
impl Informer {
    pub fn notify(notice: Notice) {
        let mut command = std::process::Command::new("/bin/notify-send");

        // priority
        command.arg("-u");
        match notice.priority {
            NoticePriority::Low => command.arg("low"),
            NoticePriority::Normal => command.arg("normal"),
            NoticePriority::Critical => command.arg("critical"),
        };

        // title
        command.arg(notice.title);

        // body
        if !notice.body.is_empty() {
            command.arg(notice.body);
        }

        // execute
        command.spawn().unwrap().wait().unwrap();
    }
}
