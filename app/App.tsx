import { mount, Scroll, signal, Text, View } from "nsh";
import { Action } from "./components/controls";
import { HistoryScreen } from "./screens/HistoryScreen";
import { PlanScreen } from "./screens/PlanScreen";
import { ProgressScreen } from "./screens/ProgressScreen";
import { SettingsScreen } from "./screens/SettingsScreen";
import { TodayScreen } from "./screens/TodayScreen";
import { dismissNotice, ready, restoreTracker, storageNotice } from "./store";

const TABS = [
  { id: "today", name: "Today", screen: TodayScreen },
  { id: "history", name: "History", screen: HistoryScreen },
  { id: "progress", name: "Progress", screen: ProgressScreen },
  { id: "plan", name: "Plan", screen: PlanScreen },
  { id: "settings", name: "Settings", screen: SettingsScreen },
] as const;

type Tab = (typeof TABS)[number];

function App() {
  const tab = signal<Tab>(TABS[0]);
  void restoreTracker();
  return (
    <View
      class="bg-background"
      style={{
        flex: 1,
        paddingTop: "env(safe-area-inset-top)",
        paddingBottom: "env(safe-area-inset-bottom)",
      }}
    >
      <Scroll class="flex-1">
        <View class="w-full gap-4 self-center p-4 pb-8" style={{ maxWidth: 640 }}>
          <If when={storageNotice()}>
            {(message) => (
              <View testID="storage-notice" class="gap-3 rounded-(--radius) border border-destructive bg-card p-4">
                <Text class="text-sm text-card-foreground">{message()}</Text>
                <Action label="Dismiss" testID="dismiss-notice" onPress={dismissNotice} />
              </View>
            )}
          </If>
          <If when={ready()} else={<Text class="text-muted-foreground">Loading your workouts</Text>}>
            <Each of={[tab()]} key={(shown) => shown.id}>
              {(shown) => {
                const Screen = shown().screen;
                return <Screen />;
              }}
            </Each>
          </If>
        </View>
      </Scroll>
      <View role="tablist" class="flex-row border-t border-border bg-card">
        <Each of={TABS} key={(entry) => entry.id}>
          {(entry) => (
            <View
              role="tab"
              testID={`tab-${entry().id}`}
              accessibilityState={{ selected: entry().id === tab().id }}
              onPress={() => tab.set(entry())}
              class="flex-1 items-center py-3 active:opacity-60"
            >
              <Text class={`${entry().id === tab().id ? "text-primary" : "text-muted-foreground"} text-sm font-semibold`}>
                {entry().name}
              </Text>
            </View>
          )}
        </Each>
      </View>
    </View>
  );
}

mount(App);
