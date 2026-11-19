import { Image, Text, token, View } from "nsh";
import type { WorkoutExercise } from "ironless/native";
import { art } from "../art";
import { change } from "../store";
import { Action, Stepper } from "./controls";

type ExerciseCardProps = {
  exercise: () => WorkoutExercise;
  index: () => number;
};

/** An exercise of the workout: its drawing, the hint from last time, and its sets to adjust and mark done. */
export function ExerciseCard(props: ExerciseCardProps) {
  const picture = () => art(props.exercise().id);
  return (
    <View testID={`exercise-${props.exercise().id}`} class="gap-3 rounded-(--radius) border border-border bg-card p-4">
      <View class="flex-row items-center gap-3">
        <If when={picture()}>
          {(source) => (
            <Image
              src={source()}
              accessibilityLabel={`${props.exercise().name}: start and end positions`}
              style={{ width: 120, height: 60, tintColor: token("foreground") }}
            />
          )}
        </If>
        <View class="flex-1 gap-1">
          <Text class="text-lg font-bold text-card-foreground">{props.exercise().name}</Text>
          <Text class="text-sm text-muted-foreground">{props.exercise().hint}</Text>
        </View>
      </View>
      <Each of={props.exercise().sets.map((_set, index) => index)}>
        {(setIndex) => {
          const set = () => props.exercise().sets[setIndex()];
          const at = () => ({ exercise: props.index(), set: setIndex() });
          const id = () => `${props.index()}-${setIndex()}`;
          return (
            <View class="flex-row items-center justify-between gap-2">
              <Stepper
                testID={`weight-${id()}`}
                value={set().weight}
                valueWidth="w-20"
                disabled={!props.exercise().loaded}
                downLabel="Lighter"
                upLabel="Heavier"
                onDown={() => change((t) => t.stepWeight(at(), false))}
                onUp={() => change((t) => t.stepWeight(at(), true))}
              />
              <Stepper
                testID={`reps-${id()}`}
                value={`${set().reps}`}
                valueWidth="w-8"
                downLabel="Fewer reps"
                upLabel="More reps"
                onDown={() => change((t) => t.stepReps(at(), false))}
                onUp={() => change((t) => t.stepReps(at(), true))}
              />
              <View
                role="checkbox"
                testID={`done-${id()}`}
                accessibilityLabel={`Set ${setIndex() + 1} done`}
                accessibilityState={{ checked: set().done }}
                onPress={() => change((t) => t.toggleDone(at()))}
                class={`${set().done ? "bg-done" : "bg-secondary"} h-9 w-9 items-center justify-center rounded-full active:opacity-70`}
              >
                <Text class={`${set().done ? "text-done-foreground" : "text-muted-foreground"} text-sm font-bold`}>
                  {`${setIndex() + 1}`}
                </Text>
              </View>
            </View>
          );
        }}
      </Each>
      <View class="flex-row gap-2">
        <Action
          label="Add set"
          testID={`add-set-${props.index()}`}
          onPress={() => change((t) => t.addSet(props.index()))}
        />
        <Action
          label="Remove set"
          testID={`remove-set-${props.index()}`}
          disabled={props.exercise().sets.length <= 1}
          onPress={() => change((t) => t.removeSet(props.index()))}
        />
      </View>
    </View>
  );
}
