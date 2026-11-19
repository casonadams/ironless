import calfRaise from "./calf-raise.svg";
import crossBodyCurl from "./cross-body-curl.svg";
import curl from "./curl.svg";
import bentOverRow from "./bent-over-row.svg";
import floorFly from "./floor-fly.svg";
import floorPress from "./floor-press.svg";
import floorPullover from "./floor-pullover.svg";
import gobletSquat from "./goblet-squat.svg";
import hammerCurl from "./hammer-curl.svg";
import hipThrust from "./hip-thrust.svg";
import kickback from "./kickback.svg";
import lateralRaise from "./lateral-raise.svg";
import lyingExtension from "./lying-extension.svg";
import overheadExtension from "./overhead-extension.svg";
import overheadPress from "./overhead-press.svg";
import pushUp from "./push-up.svg";
import rearDeltFly from "./rear-delt-fly.svg";
import reverseLunge from "./reverse-lunge.svg";
import romanianDeadlift from "./romanian-deadlift.svg";
import singleArmRow from "./single-arm-row.svg";
import singleLegRdl from "./single-leg-rdl.svg";
import splitSquat from "./split-squat.svg";

type Drawing = typeof curl;

/** Each seeded exercise's drawing, by exercise id: its start and end positions side by side, in one color. */
export const DRAWINGS: Readonly<Record<string, Drawing>> = {
  "bent-over-row": bentOverRow,
  "calf-raise": calfRaise,
  "cross-body-curl": crossBodyCurl,
  curl,
  "floor-fly": floorFly,
  "floor-press": floorPress,
  "floor-pullover": floorPullover,
  "goblet-squat": gobletSquat,
  "hammer-curl": hammerCurl,
  "hip-thrust": hipThrust,
  kickback,
  "lateral-raise": lateralRaise,
  "lying-extension": lyingExtension,
  "overhead-extension": overheadExtension,
  "overhead-press": overheadPress,
  "push-up": pushUp,
  "rear-delt-fly": rearDeltFly,
  "reverse-lunge": reverseLunge,
  "romanian-deadlift": romanianDeadlift,
  "single-arm-row": singleArmRow,
  "single-leg-rdl": singleLegRdl,
  "split-squat": splitSquat,
};

/** The drawing of the exercise with `id`; the user's own exercises have none. */
export const art = (id: string): Drawing | undefined => DRAWINGS[id];
