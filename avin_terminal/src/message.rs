/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#[derive(Debug, Clone, Copy)]
pub enum Message {
    // AssetChanged(Asset),
    Connect(bool),
}
