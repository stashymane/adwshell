use crate::settings::app_list::AppList;
use crate::settings::clock::ClockSettings;
use crate::settings::launcher::LauncherSettings;
use crate::settings::panel::Panel;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Settings {
    pub panel: Panel,
    pub launcher: LauncherSettings,
    pub app_list: AppList,
    pub clock: ClockSettings,
}
