import { signal, Text, View } from "nsh";
import { read, today } from "../store";
import { Chip, Section } from "../components/controls";

/** This week's sets per muscle against the target, and each logged exercise's history and best set. */
export function ProgressScreen() {
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
      <Text role="header" class="text-3xl font-bold text-foreground">
        Progress
      </Text>
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
