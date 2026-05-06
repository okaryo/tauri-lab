import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

export type NotificationPermissionStatus = "Granted" | "Denied";

async function ensureNotificationPermission(): Promise<NotificationPermissionStatus> {
  let permissionGranted = await isPermissionGranted();

  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }

  if (!permissionGranted) {
    return "Denied";
  }

  return "Granted";
}

export async function sendAppNotification(
  title: string,
  body: string,
): Promise<NotificationPermissionStatus> {
  const permissionStatus = await ensureNotificationPermission();

  if (permissionStatus === "Denied") {
    return permissionStatus;
  }

  sendNotification({
    title,
    body,
  });

  return permissionStatus;
}

export function sendTestNotification(): Promise<NotificationPermissionStatus> {
  return sendAppNotification("tauri-lab", "Notification plugin is ready.");
}
