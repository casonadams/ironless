import { openUrl, signal, Text, View } from "nsh";
import { reason as message } from "../store";
import { REASONS, WRITE_UP_URL } from "../why";

/** A link that opens outside the app, reporting through `failed` when it cannot. */
function Link(props: { label: string; url: string; testID: string; failed: (url: string, why: string) => void }) {
  const open = async () => {
    try {
      const { opened } = await openUrl(props.url);
      if (!opened) props.failed(props.url, "nothing on this device opens it");
    } catch (error) {
      props.failed(props.url, message(error));
    }
  };
  return (
    <View role="link" testID={props.testID} accessibilityLabel={props.label} onPress={() => void open()} class="active:opacity-60">
      <Text class="text-sm text-primary underline">{props.label}</Text>
    </View>
  );
}

/** The ideas behind the app, each with what ironless does about it and the studies it rests on. */
export function WhyView() {
  const problem = signal<string | null>(null);
  const failed = (url: string, why: string) => problem.set(`Could not open ${url}: ${why}`);

  return (
    <View class="gap-4">
      <Text class="text-base text-muted-foreground">
        The rules ironless follows, and the research behind them. Tap a source to read it.
      </Text>
      <If when={problem()}>
        {(text) => (
          <Text testID="link-problem" class="text-sm text-destructive">
            {text()}
          </Text>
        )}
      </If>
      <Each of={REASONS} key={(entry) => entry.id}>
        {(entry) => (
          <View testID={`why-${entry().id}`} class="gap-2 rounded-(--radius) border border-border bg-card p-4">
            <Text role="header" class="text-lg font-bold text-card-foreground">
              {entry().title}
            </Text>
            <Text class="text-base text-card-foreground">{entry().body}</Text>
            <Text class="text-sm text-muted-foreground">{`In ironless: ${entry().inApp}`}</Text>
            <Each of={entry().sources} key={(source) => source.url}>
              {(source, index) => (
                <Link
                  testID={`source-${entry().id}-${index()}`}
                  label={source().label}
                  url={source().url}
                  failed={failed}
                />
              )}
            </Each>
          </View>
        )}
      </Each>
      <Link testID="write-up" label="Read the full write-up" url={WRITE_UP_URL} failed={failed} />
    </View>
  );
}
