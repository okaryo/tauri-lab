import { invoke } from "@tauri-apps/api/core";

export type WorkLog = {
  id: number;
  body: string;
  createdAtMs: number;
};

export function listWorkLogs() {
  return invoke<WorkLog[]>("list_work_logs");
}

export function createWorkLog(body: string) {
  return invoke<WorkLog>("create_work_log", { body });
}
