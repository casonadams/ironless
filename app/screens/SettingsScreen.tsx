import { signal, Text, View } from "nsh";
import { Input } from "nsh/base";
import { canDownload, change, downloadBackup, read, reason, restoreBackup } from "../store";
import { Action, Chip, Section, Stepper } from "../components/controls";

/** The unit, the weekly goal, the dumbbells owned, and backups. */
export function SettingsScreen() {
  const settings = () => read((t) => t.settings());
  const newWeight = signal("");
  const weightError = signal<string | null>(null);
  const backupStatus = signal<string | null>(null);

  const addWeight = () => {
    try {
      change((t) => t.addWeight(newWeight()));
      newWeight.set("");
      weightError.set(null);
    } catch (problem) {
      weightError.set(reason(problem));
    }
  };

  const restore = async () => {
    backupStatus.set(await restoreBackup());
  };

  return (
    <View class="gap-4">
      <Text role="header" class="text-3xl font-bold text-foreground">
        Settings
      </Text>
      <Section title="Weekly goal">
        <View class="flex-row items-center justify-between">
          <Text class="text-base text-card-foreground">Workouts a week</Text>
          <Stepper
            testID="weekly-goal"
            valueWidth="w-8"
            value={`${settings().weeklyGoal}`}
            downLabel="Fewer workouts"
            upLabel="More workouts"
            onDown={() => change((t) => t.setWeeklyGoal(settings().weeklyGoal - 1))}
            onUp={() => change((t) => t.setWeeklyGoal(settings().weeklyGoal + 1))}
          />
        </View>
      </Section>
      <Section title="Unit">
        <View class="flex-row gap-2">
          <Chip testID="unit-lb" label="lb" selected={settings().unit === "lb"} onPress={() => change((t) => t.setUnit("lb"))} />
          <Chip testID="unit-kg" label="kg" selected={settings().unit === "kg"} onPress={() => change((t) => t.setUnit("kg"))} />
        </View>
        <Text class="text-sm text-muted-foreground">Labels every weight; numbers are not converted.</Text>
      </Section>
      <Section title="Your dumbbells">
        <Text class="text-sm text-muted-foreground">Weights step between these. Tap one to remove it.</Text>
        <View class="flex-row flex-wrap gap-2">
          <Each of={settings().weights} key={(weight) => weight.value}>
            {(weight) => (
              <Chip
                testID={`rack-${weight().value}`}
                label={weight().label}
                selected={false}
                onPress={() => change((t) => t.removeWeight(weight().value))}
              />
            )}
          </Each>
        </View>
        <View class="flex-row items-center gap-2">
          <Input
            testID="new-weight"
            value={newWeight()}
            onChange={({ value }) => newWeight.set(value)}
            placeholder="12.5"
            keyboardType="decimal-pad"
            class="flex-1 rounded-(--radius) border border-border bg-muted px-3 py-2 text-base text-foreground placeholder:text-muted-foreground"
          />
          <Action label="Add weight" testID="add-weight" onPress={addWeight} />
        </View>
        <If when={weightError()}>{(message) => <Text class="text-sm text-destructive">{message()}</Text>}</If>
      </Section>
      <Section title="Backup">
        <Text class="text-sm text-muted-foreground">
          Workouts live in this browser only. Download a backup now and then, and restore it on another device.
        </Text>
        <View class="flex-row flex-wrap gap-2">
          <If when={canDownload()}>
            <Action label="Download backup" testID="download-backup" onPress={downloadBackup} />
          </If>
          <Action label="Restore from file" testID="restore-backup" onPress={() => void restore()} />
        </View>
        <If when={backupStatus()}>
          {(message) => (
            <Text testID="backup-status" class="text-sm text-card-foreground">
              {message()}
            </Text>
          )}
        </If>
      </Section>
    </View>
  );
}
