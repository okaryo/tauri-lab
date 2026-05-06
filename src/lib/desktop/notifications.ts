import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

export type NotificationPermissionStatus = "Granted" | "Denied";

export async function sendTestNotification(): Promise<NotificationPermissionStatus> {
  let permissionGranted = await isPermissionGranted();

  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }

  if (!permissionGranted) {
    return "Denied";
  }

  sendNotification({
    title: "tauri-lab",
    body: "Notification plugin is ready.",
  });

  return "Granted";
}
