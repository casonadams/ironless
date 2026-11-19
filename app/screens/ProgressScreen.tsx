import { signal, Text, View } from "nsh";
import { read, today } from "../store";
import { Chip, Section } from "../components/controls";
import { WhyView } from "../components/WhyView";

const VIEWS = [
  { id: "numbers", name: "Your numbers" },
  { id: "why", name: "Why" },
] as const;

/** Progress: your numbers, or the reasons behind the rules that produce them. */
export function ProgressScreen() {
  const view = signal<(typeof VIEWS)[number]["id"]>("numbers");
  return (
    <View class="gap-4">
      <Text role="header" class="text-3xl font-bold text-foreground">
        Progress
      </Text>
      <View class="flex-row gap-2">
        <Each of={VIEWS} key={(entry) => entry.id}>
          {(entry) => (
            <Chip
              testID={`progress-view-${entry().id}`}
              label={entry().name}
              selected={entry().id === view()}
              onPress={() => view.set(entry().id)}
            />
          )}
        </Each>
      </View>
      <If when={view() === "why"} else={<ProgressNumbers />}>
        <WhyView />
      </If>
    </View>
  );
}

/** This week's sets per muscle against the target, and each logged exercise's history and best set. */
function ProgressNumbers() {
  const week = () => read((t) => t.week(today()));
  const logged = () => read((t) => t.logged());
  const chosen = signal<string | null>(null);
  const shown = () => chosen() ?? logged()[0]?.id ?? null;
  const progress = () => {
    const id = shown();
    return id === null ? null : read((t) => t.progress(id));
  };

  return (
    <View class="gap-4">
      <Section title={week().title}>
        <Each of={week().muscles} key={(muscle) => muscle.muscle}>
          {(muscle) => (
            <View testID={`muscle-${muscle().muscle}`} class="gap-1">
              <View class="flex-row justify-between">
                <Text class="text-sm text-card-foreground">{muscle().name}</Text>
                <Text class="text-sm text-muted-foreground">{`${muscle().sets} / ${muscle().target} sets`}</Text>
              </View>
              <View class="h-2 overflow-hidden rounded-full bg-muted">
                <View
                  class={`${muscle().sets >= muscle().target ? "bg-done" : "bg-primary"} h-2 rounded-full`}
                  style={{ width: `${Math.min(100, (100 * muscle().sets) / muscle().target)}%` }}
                />
              </View>
            </View>
          )}
        </Each>
      </Section>
      <If
        when={progress()}
        else={<Text class="text-muted-foreground">Finish a workout to see each exercise's progress.</Text>}
      >
        {(view) => (
          <Section title="Exercises">
            <View class="flex-row flex-wrap gap-2">
              <Each of={logged()} key={(exercise) => exercise.id}>
                {(exercise) => (
                  <Chip
                    testID={`progress-${exercise().id}`}
                    label={exercise().name}
                    selected={exercise().id === view().id}
                    onPress={() => chosen.set(exercise().id)}
                  />
                )}
              </Each>
            </View>
            <Text testID="best-set" class="text-base font-semibold text-primary">
              {`Best: ${view().best ?? "none yet"}`}
            </Text>
            <Each of={view().rows}>
              {(row) => (
                <View class="flex-row justify-between gap-3">
                  <Text class="text-sm text-card-foreground">{row().date}</Text>
                  <Text class="flex-1 text-right text-sm text-muted-foreground">{row().sets}</Text>
                </View>
              )}
            </Each>
          </Section>
        )}
      </If>
    </View>
  );
}
