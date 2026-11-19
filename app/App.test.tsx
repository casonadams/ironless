import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";
import { DRAWINGS } from "./art";
import { HOW_TO } from "./art/howto";

/** Wednesday, November 18, 2026, at noon local time. */
const NOV_18 = { clock: { time: new Date(2026, 10, 18, 12) } };

/** Lifts the first push exercise's first set one dumbbell and one rep up, marks it done, and finishes. */
async function finishPushWithOneSet(app: App): Promise<void> {
  await app.press("id:weight-0-0-up");
  await app.press("id:reps-0-0-up");
  await app.press("id:done-0-0");
  await app.press("id:finish");
}

test("a new tracker opens on push day with the starting plan and first-time hints", async () => {
  const app = await open("./App.tsx", NOV_18);
  for (const text of ["Push day", "Floor press", "Standing overhead press", "Overhead triceps extension"]) {
    expect(app.texts()).toContain(text);
  }
  expect(app.texts()).toContain("First time: pick a weight you can lift 8-12 times");
  expect(app.texts()).toContain("0 of 5 workouts this week");
  expect(app.shows("id:exercise-curl")).toBe(false);
});

test("finish waits for a done set", async () => {
  const app = await open("./App.tsx", NOV_18);
  expect(app.enabled("id:finish")).toBe(false);
  await app.press("id:done-0-0");
  expect(app.enabled("id:finish")).toBe(true);
  await app.press("id:done-0-0");
  expect(app.enabled("id:finish")).toBe(false);
});

test("finishing saves the done sets, counts the week and moves the rotation on to pull", async () => {
  const app = await open("./App.tsx", NOV_18);
  await finishPushWithOneSet(app);
  expect(app.texts()).toContain("Push workout saved. Next up: Pull.");
  expect(app.texts()).toContain("Pull day");
  expect(app.texts()).toContain("1 of 5 workouts this week");
  expect(app.shows("id:exercise-curl")).toBe(true);
  await app.press("id:tab-history");
  for (const text of ["Wed, Nov 18", "Floor press", "10 lb x 9"]) expect(app.texts()).toContain(text);
  expect(app.texts()).not.toContain("Floor fly");
});

test("the next push day prefills last time's weight and says what to beat", async () => {
  const app = await open("./App.tsx", NOV_18);
  await finishPushWithOneSet(app);
  await app.press("id:day-push");
  expect(app.texts()).toContain("Beat last time: 10 lb x 9");
  expect(app.texts()).toContain("Push day");
  expect(app.shows("text:Push (next)")).toBe(false);
  expect(app.shows("text:Pull (next)")).toBe(true);
  expect(app.count("text:10 lb")).toBe(3);
});

test("another day can be chosen in place of the suggested one", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:day-legs");
  for (const text of ["Legs day", "Goblet squat", "Floor hip thrust", "Standing calf raise"]) {
    expect(app.texts()).toContain(text);
  }
  expect(app.shows("id:exercise-floor-press")).toBe(false);
});

test("another day cannot replace a workout with done sets", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:done-0-0");
  expect(app.enabled("id:day-legs")).toBe(false);
  expect(app.texts()).toContain("Finish, or undo the done sets, to switch day.");
  await app.press("id:done-0-0");
  await app.press("id:day-legs");
  expect(app.texts()).toContain("Legs day");
});

test("a saved tracker this version cannot read is set aside, not overwritten", async () => {
  const app = await open("./App.tsx", { ...NOV_18, storage: { "ironless.tracker": "{not a backup" } });
  await app.waitFor("id:storage-notice");
  expect(app.texts()).toContain("Push day");
  await app.press("id:done-0-0");
  expect(app.storage()["ironless.tracker.unreadable"]).toBe("{not a backup");
  expect(app.storage()["ironless.tracker"]).not.toBe("{not a backup");
  await app.press("id:dismiss-notice");
  expect(app.shows("id:storage-notice")).toBe(false);
});

test("sets are added and removed, keeping one", async () => {
  const app = await open("./App.tsx", NOV_18);
  expect(app.shows("id:done-0-3")).toBe(false);
  await app.press("id:add-set-0");
  expect(app.shows("id:done-0-3")).toBe(true);
  for (let i = 0; i < 3; i++) await app.press("id:remove-set-0");
  expect(app.shows("id:done-0-1")).toBe(false);
  expect(app.enabled("id:remove-set-0")).toBe(false);
});

test("the plan swaps a seeded exercise in, and bodyweight moves have no weight steps", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-plan");
  await app.press("id:plan-floor-fly");
  await app.press("id:plan-push-up");
  await app.press("id:tab-today");
  expect(app.shows("id:exercise-floor-fly")).toBe(false);
  expect(app.shows("id:exercise-push-up")).toBe(true);
  const pushUp = app.texts().indexOf("Push-up");
  expect(app.texts().slice(pushUp)).toContain("Bodyweight");
});

test("an exercise of the user's own joins its muscle's day, and a name is required", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-plan");
  await app.press("id:plan-day-pull");
  await app.press("id:add-exercise");
  expect(app.texts()).toContain("an exercise needs a name");
  await app.type("id:new-exercise-name", "Zottman curl");
  await app.press("id:muscle-choice-biceps");
  await app.press("id:add-exercise");
  expect(app.texts()).toContain("Zottman curl");
  expect(app.texts()).not.toContain("an exercise needs a name");
  await app.press("id:tab-today");
  await app.press("id:day-pull");
  expect(app.texts()).toContain("Zottman curl");
});

test("weights step over the dumbbells in settings", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-settings");
  await app.type("id:new-weight", "7.5");
  await app.press("id:add-weight");
  expect(app.texts()).toContain("7.5 lb");
  await app.type("id:new-weight", "0");
  await app.press("id:add-weight");
  expect(app.texts()).toContain("a dumbbell must weigh more than 0");
  await app.press("id:unit-kg");
  expect(app.texts()).toContain("7.5 kg");
  await app.press("id:tab-today");
  await app.press("id:weight-0-0-up");
  expect(app.texts()).toContain("7.5 kg");
});

test("progress shows the week's sets per muscle and each exercise's best set", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-progress");
  expect(app.texts()).toContain("Finish a workout to see each exercise's progress.");
  await app.press("id:tab-today");
  await finishPushWithOneSet(app);
  await app.press("id:tab-progress");
  expect(app.texts()).toContain("Week of Mon, Nov 16");
  expect(app.shows("id:muscle-chest >> text:1 / 10 sets")).toBe(true);
  expect(app.shows("id:muscle-back >> text:0 / 10 sets")).toBe(true);
  expect(app.texts()).toContain("Best: 10 lb x 9");
});

test("every seeded exercise has a drawing and every drawing an exercise", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-plan");
  const ids = Object.keys(DRAWINGS);
  let exercises = 0;
  let drawn = 0;
  for (const day of ["push", "pull", "legs"]) {
    await app.press(`id:plan-day-${day}`);
    exercises += app.count("text:In plan") + app.count("text:Add");
    drawn += ids.filter((id) => app.shows(`id:plan-${id}`)).length;
  }
  expect(drawn).toBe(ids.length);
  expect(exercises).toBe(ids.length);
  expect(Object.keys(HOW_TO).sort()).toEqual([...ids].sort());
});

test("the info button explains a lift only when asked", async () => {
  const app = await open("./App.tsx", NOV_18);
  expect(app.shows("id:howto-sheet")).toBe(false);
  await app.press("id:howto-floor-press");
  await app.waitFor("id:howto-sheet");
  for (const text of [
    "ON THE FLOOR",
    "1. Lie on your back, knees bent, a dumbbell in each hand over your chest.",
    "Cue: Keep elbows about 45 degrees from your body, not flared out.",
  ]) {
    expect(app.texts()).toContain(text);
  }
  await app.press("id:howto-close");
  expect(app.shows("id:howto-sheet")).toBe(false);
});

test("the plan explains each lift, and the user's own say they have no steps", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-plan");
  await app.press("id:plan-day-pull");
  expect(app.texts()).toContain("Bent over - 8-12 reps");
  await app.press("id:howto-single-arm-row");
  await app.waitFor("id:howto-sheet");
  expect(app.texts()).toContain("Cue: Pull with your elbow, not your hand, and keep your shoulders level.");
  await app.press("id:howto-close");
  await app.type("id:new-exercise-name", "Zottman curl");
  await app.press("id:add-exercise");
  await app.press("id:howto-custom-1");
  await app.waitFor("id:howto-sheet");
  expect(app.texts()).toContain("Your own exercise: no steps are written for it.");
});

test("the why view explains each rule and opens its sources", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-progress");
  await app.press("id:progress-view-why");
  for (const text of ["Twice a week beats once", "More sets, more muscle", "A missed day is not a broken habit"]) {
    expect(app.texts()).toContain(text);
  }
  expect(app.shows("id:muscle-chest")).toBe(false);
  await app.press("id:source-frequency-0");
  await app.press("id:write-up");
  expect(app.openedUrls()).toEqual([
    "https://doi.org/10.1007/s40279-016-0543-8",
    "https://github.com/casonadams/ironless/blob/main/docs/why.md",
  ]);
  await app.press("id:progress-view-numbers");
  expect(app.shows("id:muscle-chest")).toBe(true);
});

test("a source that cannot open says so", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.failOpenUrls("https://doi.org/*");
  await app.press("id:tab-progress");
  await app.press("id:progress-view-why");
  await app.press("id:source-habit-0");
  await app.waitFor("id:link-problem");
  expect(app.texts()).toContain(
    "Could not open https://doi.org/10.1002/ejsp.674: nothing on this device opens it",
  );
});

test("the tracker is kept between sessions", async () => {
  const first = await open("./App.tsx", NOV_18);
  await finishPushWithOneSet(first);
  await first.press("id:done-0-0");
  const second = await open("./App.tsx", { ...NOV_18, storage: first.storage() });
  await second.waitFor("text:Pull day");
  expect(second.texts()).toContain("1 of 5 workouts this week");
  expect(second.shows("id:done-0-0")).toBe(true);
  expect(second.enabled("id:finish")).toBe(true);
});

test("a backup downloads through the page and a picked backup file restores", async () => {
  const saved = await open("./App.tsx", NOV_18);
  await finishPushWithOneSet(saved);
  const backup = saved.storage()["ironless.tracker"];

  const app = await open("./App.tsx", { ...NOV_18, host: { handlers: { "backup.download": null } } });
  await app.press("id:tab-settings");
  await app.press("id:download-backup");
  const [post] = app.host.posts();
  expect(post.name).toBe("backup.download");
  expect((post.data as { name: string }).name).toBe("ironless-2026-11-18.json");

  app.onPick(() => [{ name: "backup.json", type: "application/json", data: new TextEncoder().encode(backup) }]);
  await app.press("id:restore-backup");
  await app.waitFor("text:Backup restored");
  await app.press("id:tab-history");
  expect(app.texts()).toContain("10 lb x 9");
});

test("a file that is not a backup is refused and changes nothing", async () => {
  const app = await open("./App.tsx", NOV_18);
  await app.press("id:tab-settings");
  app.onPick(() => [{ name: "notes.json", type: "application/json", data: new TextEncoder().encode("{}") }]);
  await app.press("id:restore-backup");
  await app.waitFor("id:backup-status");
  expect(app.texts().some((text) => text.startsWith("Could not restore: not an ironless backup"))).toBe(true);
  await app.press("id:tab-today");
  expect(app.texts()).toContain("Push day");
});
