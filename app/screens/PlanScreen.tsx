import { signal, Text, View } from "nsh";
import { Input } from "nsh/base";
import type { Muscle } from "ironless/native";
import { DAYS } from "../days";
import { change, read, reason } from "../store";
import { Action, Chip, Section } from "../components/controls";

/** Each day's exercises to include or leave out, and a form to add the user's own. */
export function PlanScreen() {
  const day = signal(DAYS[0]);
  const rows = () => read((t) => t.plan(day().day));
  const muscles = () => read((t) => t.muscles(day().day));
  const name = signal("");
  const picked = signal<Muscle | null>(null);
  const muscle = (): Muscle => picked() ?? muscles()[0].muscle;
  const error = signal<string | null>(null);

  const chooseDay = (entry: (typeof DAYS)[number]) => {
    day.set(entry);
    picked.set(null);
    error.set(null);
  };

  const add = () => {
    try {
      change((t) => t.addExercise(name(), muscle()));
      name.set("");
      error.set(null);
    } catch (problem) {
      error.set(reason(problem));
    }
  };

  return (
    <View class="gap-4">
      <Text role="header" class="text-3xl font-bold text-foreground">
        Plan
      </Text>
      <View class="flex-row gap-2">
        <Each of={DAYS} key={(entry) => entry.day}>
          {(entry) => (
            <Chip
              testID={`plan-day-${entry().day}`}
              label={entry().name}
              selected={entry().day === day().day}
              onPress={() => chooseDay(entry())}
            />
          )}
        </Each>
      </View>
      <Section title={`${day().name} exercises`}>
        <Each of={rows()} key={(row) => row.id}>
          {(row) => (
            <View class="flex-row items-center justify-between gap-3">
              <View class="flex-1">
                <Text class="text-base text-card-foreground">{row().name}</Text>
                <Text class="text-sm text-muted-foreground">{row().reps}</Text>
              </View>
              <Chip
                testID={`plan-${row().id}`}
                label={row().planned ? "In plan" : "Add"}
                selected={row().planned}
                onPress={() => change((t) => t.togglePlanned(row().id))}
              />
            </View>
          )}
        </Each>
      </Section>
      <Section title="Add your own">
        <Input
          testID="new-exercise-name"
          value={name()}
          onChange={({ value }) => name.set(value)}
          placeholder="Exercise name"
          autoCapitalize="sentences"
          class="rounded-(--radius) border border-border bg-muted px-3 py-2 text-base text-foreground placeholder:text-muted-foreground"
        />
        <View class="flex-row flex-wrap gap-2">
          <Each of={muscles()} key={(entry) => entry.muscle}>
            {(entry) => (
              <Chip
                testID={`muscle-choice-${entry().muscle}`}
                label={entry().name}
                selected={entry().muscle === muscle()}
                onPress={() => picked.set(entry().muscle)}
              />
            )}
          </Each>
        </View>
        <If when={error()}>{(message) => <Text class="text-sm text-destructive">{message()}</Text>}</If>
        <Action label="Add exercise" testID="add-exercise" onPress={add} />
      </Section>
    </View>
  );
}
