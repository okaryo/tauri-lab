import { getCurrentWindow } from "@tauri-apps/api/window";

export async function showCurrentWindow() {
  const appWindow = getCurrentWindow();

  await appWindow.unminimize();
  await appWindow.show();
  await appWindow.setFocus();
}
