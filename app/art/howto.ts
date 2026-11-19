/** How to do a seeded exercise: a few short steps and the one thing people most often get wrong. */
export type HowTo = {
  steps: readonly string[];
  cue: string;
};

/** Each seeded exercise's instructions, by exercise id. */
export const HOW_TO: Readonly<Record<string, HowTo>> = {
  "floor-press": {
    steps: [
      "Lie on your back, knees bent, a dumbbell in each hand over your chest.",
      "Lower until your upper arms touch the floor.",
      "Pause, then press straight back up.",
    ],
    cue: "Keep elbows about 45 degrees from your body, not flared out.",
  },
  "floor-fly": {
    steps: [
      "Lie on your back, knees bent, dumbbells over your chest, palms facing.",
      "With a slight bend in the elbows, open your arms out to the sides.",
      "Stop when your upper arms touch the floor, then hug them back up.",
    ],
    cue: "Keep the elbow bend fixed: the movement is at the shoulders.",
  },
  "overhead-press": {
    steps: [
      "Stand tall, dumbbells at your shoulders, palms forward.",
      "Press them straight up until your arms are straight.",
      "Lower back to your shoulders under control.",
    ],
    cue: "Squeeze your glutes and brace: do not lean back.",
  },
  "lateral-raise": {
    steps: [
      "Stand tall, dumbbells at your sides.",
      "Raise your arms out to the sides, elbows slightly bent.",
      "Stop at shoulder height, then lower slowly.",
    ],
    cue: "Lead with the elbows and keep the shoulders down, not shrugged.",
  },
  "lying-extension": {
    steps: [
      "Lie on your back, knees bent, dumbbells over your shoulders, palms facing.",
      "Bend only at the elbows to lower the dumbbells beside your head.",
      "Straighten your arms back up.",
    ],
    cue: "Keep your upper arms still and pointing at the ceiling.",
  },
  "overhead-extension": {
    steps: [
      "Stand tall, holding one dumbbell with both hands above your head.",
      "Bend at the elbows to lower it behind your head.",
      "Straighten your arms back up.",
    ],
    cue: "Keep elbows pointing forward and close to your head.",
  },
  "push-up": {
    steps: [
      "Hands under your shoulders, body straight from head to heels.",
      "Lower until your chest nearly touches the floor.",
      "Push back up. On your knees is fine until full ones feel solid.",
    ],
    cue: "Brace your stomach so your hips do not sag.",
  },
  kickback: {
    steps: [
      "Hinge forward with a flat back, a dumbbell in each hand.",
      "Lift your upper arms level with your back, elbows bent.",
      "Straighten your arms behind you, then bend back.",
    ],
    cue: "Only the forearm moves: the upper arm stays pinned.",
  },
  "bent-over-row": {
    steps: [
      "Hinge forward with a flat back, knees soft, dumbbells hanging.",
      "Pull the dumbbells toward your hips.",
      "Lower until your arms are straight.",
    ],
    cue: "Keep your back flat and still: do not swing up.",
  },
  "single-arm-row": {
    steps: [
      "Step one foot forward and rest that hand on your thigh, back flat.",
      "Pull the dumbbell toward your hip with the other arm.",
      "Lower to a full stretch, finish the reps, then switch sides.",
    ],
    cue: "Pull with your elbow, not your hand, and keep your shoulders level.",
  },
  "floor-pullover": {
    steps: [
      "Lie on your back, knees bent, holding one dumbbell over your chest with both hands.",
      "With arms nearly straight, lower it back over your head toward the floor.",
      "Pull it back over your chest.",
    ],
    cue: "Keep your lower back on the floor and the elbows slightly bent.",
  },
  "rear-delt-fly": {
    steps: [
      "Hinge forward with a flat back, dumbbells hanging, palms facing.",
      "Raise your arms out to the sides, elbows slightly bent.",
      "Squeeze at the top, then lower slowly.",
    ],
    cue: "Go light: move the arms, not your back.",
  },
  curl: {
    steps: [
      "Stand tall, dumbbells at your sides, palms forward.",
      "Bend your elbows to curl the dumbbells up.",
      "Lower slowly until your arms are straight.",
    ],
    cue: "Elbows stay at your sides: no swinging.",
  },
  "hammer-curl": {
    steps: [
      "Stand tall, dumbbells at your sides, palms facing in.",
      "Curl them up, keeping your palms facing each other.",
      "Lower slowly until your arms are straight.",
    ],
    cue: "Keep your wrists straight and elbows at your sides.",
  },
  "cross-body-curl": {
    steps: [
      "Stand tall, dumbbells at your sides, palms facing in.",
      "Curl one dumbbell up and across toward the other shoulder.",
      "Lower it, then curl the other arm.",
    ],
    cue: "Keep your elbow by your side as the hand crosses.",
  },
  "goblet-squat": {
    steps: [
      "Hold one dumbbell upright against your chest, feet a little wider than your hips.",
      "Sit down between your heels as low as you can with a flat back.",
      "Stand back up through your whole foot.",
    ],
    cue: "Knees track over your toes; chest stays up.",
  },
  "split-squat": {
    steps: [
      "Take a long step, back heel up, dumbbells at your sides.",
      "Lower straight down until your back knee nearly touches the floor.",
      "Push back up, finish the reps, then switch legs.",
    ],
    cue: "Stay upright and push through the front heel.",
  },
  "romanian-deadlift": {
    steps: [
      "Stand with dumbbells in front of your thighs, knees soft.",
      "Push your hips back and slide the dumbbells down your legs.",
      "Stop when you feel a stretch behind your thighs, then stand up.",
    ],
    cue: "The back stays flat: the hips move, not the spine.",
  },
  "hip-thrust": {
    steps: [
      "Lie on your back, knees bent, feet flat, a dumbbell resting on your hips.",
      "Drive through your heels to lift your hips.",
      "Squeeze at the top, then lower.",
    ],
    cue: "Stop when your body makes a straight line: do not arch your back.",
  },
  "calf-raise": {
    steps: [
      "Stand tall, dumbbells at your sides, a wall nearby for balance.",
      "Rise up onto the balls of your feet as high as you can.",
      "Pause, then lower your heels all the way.",
    ],
    cue: "Go slow at the bottom: that stretch does the work.",
  },
  "reverse-lunge": {
    steps: [
      "Stand tall, dumbbells at your sides.",
      "Step one foot back and lower until the back knee nearly touches the floor.",
      "Push through the front foot to step back up, then switch legs.",
    ],
    cue: "Keep your weight on the front foot.",
  },
  "single-leg-rdl": {
    steps: [
      "Stand on one leg, a dumbbell in the opposite hand, knee soft.",
      "Hinge forward as the free leg lifts behind you.",
      "Stop at a stretch behind your thigh, stand up, then switch sides.",
    ],
    cue: "Keep your hips level and your back flat.",
  },
};
