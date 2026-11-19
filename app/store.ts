import { host, Picker, signal, Storage } from "nsh";
import { Tracker } from "ironless/native";

/** The storage key the tracker is kept under between sessions. */
const TRACKER_KEY = "ironless.tracker";

const tracker = new Tracker();
const revision = signal(0);

/** Reads from the tracker, tracked so the reader runs again after every change. */
export function read<T>(view: (tracker: Tracker) => T): T {
  revision();
  return view(tracker);
}

/** Changes the tracker and keeps it for the next session; what `action` throws is rethrown after nothing changed. */
export function change(action: (tracker: Tracker) => void): void {
  action(tracker);
  revision.set(revision() + 1);
  void Storage.setItem(TRACKER_KEY, tracker.save());
}

/** Brings back the tracker the last session kept; a kept one this version cannot read is replaced. */
export async function restoreTracker(): Promise<void> {
  const kept = await Storage.getItem(TRACKER_KEY);
  if (kept === null) return;
  try {
    tracker.restore(kept);
    revision.set(revision() + 1);
  } catch (error) {
    console.warn(`starting a new tracker: ${error}`);
  }
}

const pad = (value: number): string => String(value).padStart(2, "0");

/** What a refused change says, for the user. */
export const reason = (error: unknown): string => (error instanceof Error ? error.message : String(error));

/** The device's local day as `YYYY-MM-DD`. */
export function today(): string {
  const now = new Date();
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
}

/** Whether the page around the app can download a backup file. */
export const canDownload = (): boolean => host.available();

/** Hands the backup to the page, which downloads it as `ironless-<day>.json`. */
export function downloadBackup(): void {
  host.post("backup.download", { name: `ironless-${today()}.json`, text: tracker.save() });
}

/** Replaces the tracker with a backup file the user picks; resolves with what happened, for the user. */
export async function restoreBackup(): Promise<string | null> {
  const [picked] = await Picker.pickFiles({ filters: [{ name: "ironless backup", extensions: ["json"] }] });
  if (!picked) return null;
  if (!picked.file) return `${picked.name} is too large to be a backup`;
  const text = await picked.file.text();
  try {
    change((t) => t.restore(text));
    return "Backup restored";
  } catch (error) {
    return `Could not restore: ${reason(error)}`;
  }
}
