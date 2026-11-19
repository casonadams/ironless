import { signal, Text, View } from "nsh";
import { DAYS, dayName } from "../days";
import { change, read, today } from "../store";
import { Action, Chip } from "../components/controls";
import { ExerciseCard } from "../components/ExerciseCard";

/** The workout in progress: the day, this week's count, each exercise's sets, and Finish. */
export function TodayScreen() {
  const workout = () => read((t) => t.workout());
  const week = () => read((t) => t.week(today()));
  const finished = signal<string | null>(null);

  const finish = () => {
    const day = dayName(workout().day);
    change((t) => t.finish(today()));
    finished.set(`${day} workout saved. Next up: ${dayName(workout().day)}.`);
  };

  return (
    <View class="gap-4">
      <View class="gap-1">
        <Text role="header" class="text-3xl font-bold text-foreground">
          {dayName(workout().day)} day
        </Text>
        <Text testID="week-count" class="text-sm text-muted-foreground">
          {`${week().workouts} of ${week().goal} workouts this week`}
        </Text>
      </View>
      <View class="flex-row gap-2">
        <Each of={DAYS} key={(entry) => entry.day}>
          {(entry) => (
            <Chip
              testID={`day-${entry().day}`}
              label={entry().day === workout().suggested ? `${entry().name} (next)` : entry().name}
              selected={entry().day === workout().day}
              onPress={() => {
                finished.set(null);
                if (entry().day !== workout().day) change((t) => t.chooseDay(entry().day));
              }}
            />
          )}
        </Each>
      </View>
      <If when={finished()}>
        {(message) => (
          <Text testID="finished" class="rounded-(--radius) bg-done/15 p-3 text-done">
            {message()}
          </Text>
        )}
      </If>
      <Each of={workout().exercises.map((exercise) => exercise.id)}>
        {(_id, index) => <ExerciseCard exercise={() => workout().exercises[index()]} index={index} />}
      </Each>
      <If when={workout().exercises.length === 0}>
        <Text class="text-muted-foreground">This day has no exercises. Add some on the Plan tab.</Text>
      </If>
      <Action
        label="Finish workout"
        testID="finish"
        tone="primary"
        disabled={!workout().canFinish}
        onPress={finish}
      />
    </View>
  );
}
