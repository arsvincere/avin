/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

pub struct Timer {
    instant: std::time::Instant,
}
impl Timer {
    pub fn new() -> Self {
        Self {
            instant: std::time::Instant::now(),
        }
    }
    pub fn start(&mut self) {
        self.instant = std::time::Instant::now();
    }
    pub fn stop(&self, msg: &str) {
        let duration = self.instant.elapsed();
        println!("Timer {msg} {duration:?}");
    }
}
impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
