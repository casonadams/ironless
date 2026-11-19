import { type JSX, Text, View } from "nsh";

type ActionProps = {
  label: string;
  onPress: () => void;
  testID?: string;
  accessibilityLabel?: string;
  disabled?: boolean;
  tone?: "primary" | "secondary";
};

/** A pressable button with a text label. */
export function Action(props: ActionProps) {
  const primary = props.tone === "primary";
  return (
    <View
      role="button"
      testID={props.testID}
      accessibilityLabel={props.accessibilityLabel}
      disabled={props.disabled}
      onPress={props.onPress}
      class={`${primary ? "bg-primary active:bg-primary/80" : "bg-secondary active:bg-secondary/70"} items-center justify-center rounded-(--radius) px-4 py-3 disabled:opacity-40`}
    >
      <Text class={`${primary ? "text-primary-foreground" : "text-secondary-foreground"} text-base font-semibold`}>
        {props.label}
      </Text>
    </View>
  );
}

type ChipProps = {
  label: string;
  selected: boolean;
  onPress: () => void;
  testID?: string;
  disabled?: boolean;
};

/** One choice of a row of choices, highlighted while selected. */
export function Chip(props: ChipProps) {
  return (
    <View
      role="button"
      testID={props.testID}
      accessibilityState={{ selected: props.selected }}
      disabled={props.disabled}
      onPress={props.onPress}
      class={`${props.selected ? "bg-primary" : "bg-secondary"} rounded-full px-4 py-2 active:opacity-70 disabled:opacity-40`}
    >
      <Text class={`${props.selected ? "text-primary-foreground" : "text-secondary-foreground"} text-sm font-semibold`}>
        {props.label}
      </Text>
    </View>
  );
}

type StepperProps = {
  value: string;
  onDown: () => void;
  onUp: () => void;
  downLabel: string;
  upLabel: string;
  testID: string;
  /** The value's width class, wide enough for its longest text. */
  valueWidth: string;
  disabled?: boolean;
};

/** A value between a minus and a plus button. */
export function Stepper(props: StepperProps) {
  const button = (sign: string, label: string, id: string, onPress: () => void) => (
    <View
      role="button"
      testID={id}
      accessibilityLabel={label}
      disabled={props.disabled}
      onPress={onPress}
      class="h-8 w-8 items-center justify-center rounded-full bg-secondary active:bg-secondary/60 disabled:opacity-0"
    >
      <Text class="text-lg font-bold text-secondary-foreground">{sign}</Text>
    </View>
  );
  return (
    <View class="flex-row items-center gap-1">
      {button("-", props.downLabel, `${props.testID}-down`, props.onDown)}
      <Text testID={props.testID} class={`${props.valueWidth} text-center text-sm font-semibold text-foreground`}>
        {props.value}
      </Text>
      {button("+", props.upLabel, `${props.testID}-up`, props.onUp)}
    </View>
  );
}

/** A titled group of content on a card. */
export function Section(props: { title: string; children?: JSX.Element }) {
  return (
    <View class="gap-3 rounded-(--radius) border border-border bg-card p-4">
      <Text role="header" class="text-lg font-bold text-card-foreground">
        {props.title}
      </Text>
      {props.children}
    </View>
  );
}
