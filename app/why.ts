/** A study a reason rests on, opened by its DOI. */
export type Source = {
  label: string;
  url: string;
};

/** One idea behind the app: what the evidence says, what ironless does about it, and where it comes from. */
export type Reason = {
  id: string;
  title: string;
  body: string;
  inApp: string;
  sources: readonly Source[];
};

/** The full write-up, with what each study did and its limits. */
export const WRITE_UP_URL = "https://github.com/casonadams/ironless/blob/main/docs/why.md";

/** The reasons, in the order a new user meets them. Keep in step with docs/why.md. */
export const REASONS: readonly Reason[] = [
  {
    id: "frequency",
    title: "Twice a week beats once",
    body: "Training a muscle twice a week grew more muscle than once. When the weekly sets were the same, how they were spread out mattered little.",
    inApp: "Push, pull and legs over five days trains each muscle close to twice a week.",
    sources: [
      { label: "Schoenfeld et al. 2016, Sports Medicine", url: "https://doi.org/10.1007/s40279-016-0543-8" },
      { label: "Schoenfeld et al. 2019, J Sports Sciences", url: "https://doi.org/10.1080/02640414.2018.1555906" },
    ],
  },
  {
    id: "volume",
    title: "More sets, more muscle",
    body: "Each extra hard set a week added a little more growth. Ten or more sets per muscle was the top group studied; trained lifters may do best with 12 to 20.",
    inApp: "Your numbers count sets per muscle each week against 10, a starting point rather than a ceiling.",
    sources: [
      { label: "Schoenfeld et al. 2017, J Sports Sciences", url: "https://doi.org/10.1080/02640414.2016.1210197" },
      { label: "Baz-Valle et al. 2022, J Human Kinetics", url: "https://doi.org/10.2478/hukin-2022-0017" },
    ],
  },
  {
    id: "load",
    title: "Light or heavy, go close to failure",
    body: "Light weights for many reps and heavy weights for few built similar muscle when every set was taken to failure. Heavy weights did build more top-end strength.",
    inApp: "When you reach your heaviest dumbbell, the app keeps asking for more reps instead of stalling.",
    sources: [
      { label: "Schoenfeld et al. 2017, J Strength Cond Res", url: "https://doi.org/10.1519/JSC.0000000000002200" },
      { label: "Schoenfeld et al. 2021, Sports", url: "https://doi.org/10.3390/sports9020032" },
    ],
  },
  {
    id: "progression",
    title: "Add reps, then add weight",
    body: "Double progression is a common coaching method rather than a tested program: it puts the findings above into practice. Beat last time by a rep, and once every set reaches the top of the range, move up a dumbbell.",
    inApp: "Each set starts one rep past last time, and the hint says what to beat or when to go heavier.",
    sources: [],
  },
  {
    id: "habit",
    title: "A missed day is not a broken habit",
    body: "In a 12-week study of new daily habits, missing one chance did not materially slow the habit forming. Getting to fully automatic took from 18 to 254 days.",
    inApp: "There is a weekly goal instead of a daily streak, and a skipped day just means the rotation waits.",
    sources: [{ label: "Lally et al. 2010, Eur J Social Psychology", url: "https://doi.org/10.1002/ejsp.674" }],
  },
  {
    id: "motivation",
    title: "Train for your own reasons",
    body: "Among 2,771 users of a fitness app, those who trained because they enjoyed it or valued it stayed longer. The study shows a link, not a cause.",
    inApp: "The app shows real progress and lets you swap exercises. No badges, points or guilt.",
    sources: [{ label: "Fuente-Vidal et al. 2026, JMIR mHealth", url: "https://doi.org/10.2196/72201" }],
  },
  {
    id: "trade-offs",
    title: "What floor and standing gives up",
    body: "No bench shortens the chest press, no bar means no pulling from overhead, and legs outgrow light dumbbells quickly. These are practical limits, not study findings.",
    inApp: "Push-ups, pullovers and single-leg moves fill the gaps, and higher reps keep legs progressing.",
    sources: [],
  },
];
