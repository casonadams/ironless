import type { Day } from "ironless/native";

/** The rotation's days in order, with the names the app shows. */
export const DAYS: readonly { day: Day; name: string }[] = [
  { day: "push", name: "Push" },
  { day: "pull", name: "Pull" },
  { day: "legs", name: "Legs" },
];

/** The name the app shows for `day`. */
export const dayName = (day: Day): string => DAYS.find((entry) => entry.day === day)?.name ?? day;
