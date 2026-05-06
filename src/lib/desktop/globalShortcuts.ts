import { isRegistered, register, unregister } from "@tauri-apps/plugin-global-shortcut";

export const workLogShortcut = "CommandOrControl+Shift+L";

export async function registerWorkLogShortcut(onPressed: () => void) {
  if (await isRegistered(workLogShortcut)) {
    return;
  }

  await register(workLogShortcut, (event) => {
    if (event.state === "Pressed") {
      onPressed();
    }
  });
}

export function unregisterWorkLogShortcut() {
  return unregister(workLogShortcut);
}
