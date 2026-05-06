import { invoke } from "@tauri-apps/api/core";

export type AppSettings = {
  workDurationMinutes: number;
  breakDurationMinutes: number;
  timerNotificationsEnabled: boolean;
};

export function loadAppSettings(): Promise<AppSettings> {
  return invoke("load_app_settings");
}

export function saveAppSettings(settings: AppSettings): Promise<AppSettings> {
  return invoke("save_app_settings", { settings });
}
