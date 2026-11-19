import { Text, View } from "nsh";
import { read } from "../store";

/** The finished workouts, newest first, each exercise with its sets. */
export function HistoryScreen() {
  const history = () => read((t) => t.history());
  return (
    <View class="gap-4">
      <Text role="header" class="text-3xl font-bold text-foreground">
        History
      </Text>
      <If when={history().length === 0}>
        <Text class="text-muted-foreground">Finished workouts show up here.</Text>
      </If>
      <Each of={history()} key={(entry) => entry.key}>
        {(entry) => (
          <View testID={`workout-${entry().key}`} class="gap-2 rounded-(--radius) border border-border bg-card p-4">
            <View class="flex-row items-baseline justify-between">
              <Text class="text-lg font-bold text-card-foreground">{entry().date}</Text>
              <Text class="text-sm font-semibold uppercase text-primary">{entry().day}</Text>
            </View>
            <Each of={entry().lines}>
              {(line) => (
                <View class="gap-0.5">
                  <Text class="text-base text-card-foreground">{line().name}</Text>
                  <Text class="text-sm text-muted-foreground">{line().sets}</Text>
                </View>
              )}
            </Each>
          </View>
        )}
      </Each>
    </View>
  );
}
