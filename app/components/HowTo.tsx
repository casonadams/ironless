import { Image, signal, Text, token, View } from "nsh";
import { Sheet } from "nsh/base";
import { art } from "../art";
import { HOW_TO } from "../art/howto";

type HowToButtonProps = {
  id: string;
  name: string;
  position: string | null;
};

/** A small info button that opens a sheet on how to do the exercise: its position, drawing, steps and key cue. */
export function HowToButton(props: HowToButtonProps) {
  const open = signal(false);
  const howTo = () => HOW_TO[props.id];
  const picture = () => art(props.id);

  return (
    <>
      <View
        role="button"
        testID={`howto-${props.id}`}
        accessibilityLabel={`How to do ${props.name}`}
        onPress={() => open.set(true)}
        class="h-8 w-8 items-center justify-center rounded-full border border-border active:opacity-60"
      >
        <Text class="text-sm font-bold text-muted-foreground">i</Text>
      </View>
      <Sheet
        open={open()}
        onOpenChange={(event) => open.set(event.open)}
        closeOnSwipe
        accessibilityLabel={`How to do ${props.name}`}
        backdropClass="bg-background/70"
        popupClass="gap-3 rounded-t-2xl border-t border-border bg-card p-6 pb-10"
      >
        <View testID="howto-sheet" class="w-full gap-3 self-center" style={{ maxWidth: 640 }}>
          <View class="h-1 w-10 self-center rounded-full bg-muted" />
          <Text role="header" class="text-xl font-bold text-card-foreground">
            {props.name}
          </Text>
          <If when={props.position}>
            {(position) => <Text class="text-sm font-semibold uppercase text-primary">{position()}</Text>}
          </If>
          <If when={picture()}>
            {(source) => (
              <Image
                src={source()}
                accessibilityLabel={`${props.name}: start and end positions`}
                style={{ width: 240, height: 120, alignSelf: "center", tintColor: token("foreground") }}
              />
            )}
          </If>
          <If
            when={howTo()}
            else={<Text class="text-sm text-muted-foreground">Your own exercise: no steps are written for it.</Text>}
          >
            {(entry) => (
              <View class="gap-2">
                <Each of={entry().steps.map((text, index) => `${index + 1}. ${text}`)}>
                  {(step) => <Text class="text-base text-card-foreground">{step()}</Text>}
                </Each>
                <Text class="text-base text-card-foreground">
                  <Text class="font-bold text-primary">Cue: </Text>
                  {entry().cue}
                </Text>
              </View>
            )}
          </If>
          <View
            role="button"
            testID="howto-close"
            onPress={() => open.set(false)}
            class="items-center rounded-(--radius) bg-secondary py-3 active:opacity-70"
          >
            <Text class="font-semibold text-secondary-foreground">Got it</Text>
          </View>
        </View>
      </Sheet>
    </>
  );
}
