use std::{
    io::{stdout, Write},
    iter,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon_progress::ProgressAdaptor;
use switch_manips::SWITCH_OVERFLOWS;
use undertale_ruins_analysis::rng::{PrecomputedRNG, RNG};

mod switch_manips;
// its
fn main() {
    // let mut me = RNG::new(70654, false, false, false);

    // for _ in 0..10 {
    //     println!("{}", me.next_f64(100.))
    // }

    // return;
    let seeds = RNG::calculate_unique_seeds(false, false);

    let it = ProgressAdaptor::new(seeds);

    let progress = it.items_processed();
    let total = it.len();

    let done = Arc::new(AtomicBool::new(false));

    {
        let done = done.clone();

        rayon::spawn(move || {
            let all_manips = it
                .map_init(
                    || {
                        vec![
                            step_tb(NAMING_SCREEN),
                            Box::new(step_fun),
                            step_tb(FLOWEY_PREDIALOGUE),
                            step_tb(FLOWEY_DIALOGUE_BEFORE_WINKSTAR),
                            Box::new(step_flowey_until_pellets),
                            Box::new(|mut rng, _| {
                                let left_pellet_seed = rng.next_f64(0.5);

                                if left_pellet_seed > 0.48128 {
                                    Box::new(iter::once((rng, 0, None)))
                                } else {
                                    Box::new(iter::empty())
                                }
                            }),
                            Box::new(|mut rng, _| {
                                for _ in 0..11460 {
                                    rng.next_u32();
                                }
                                rng.action_move("000100000");

                                Box::new(iter::once((rng, 0, None)))
                            }),
                            step_tb(TORIEL_TERRIBLE_CREATURE_DIALOGUE),
                            step_regularlist(&SWITCH_OVERFLOWS),
                            step_regularlist(&FEARLESSES),
                            step_tb(TORIEL_RUINS3_DIALOGUE),
                            step_tb(TORIEL_RUINS4_DIALOGUE),
                            step_regularlist(&TAWS),
                            step_tb(TORIEL_RUINS6_BEGINNING_DIALOGUE),
                            step_tb(TORIEL_RUINS6_ENDING_DIALOGUE),
                            step_tb(TORIEL_RUINS7_DIALOGUE),
                            Box::new(|mut rng, _| {
                                rng.next_u32();
                                // first ruins7 stepcount
                                Box::new(iter::once((rng, 0, None)))
                            }),
                            step_tb(CANDY_TAKE),
                            Box::new(step_encounterskip(40., 16, "room_ruins7")),
                            Box::new(step_encounterskip(40., 6, "room_ruins8")),
                            Box::new(step_ruins9_encounter),
                            step_tb(ONEROCK_FIRSTCALL),
                            step_regularlist(&ONEROCK_SECONDCALL),
                            step_tb(ONEROCK_SIGN),
                            Box::new(|mut rng, _| {
                                rng.next_u32();
                                // first ruins10 stepcount
                                Box::new(iter::once((rng, 0, None)))
                            }),
                            Box::new(step_ruins11_stepcount),
                            step_tb(GRUMPYROCK),
                            Box::new(step_ruins11_rest),
                            // Box::new(step_phonecall),
                            // Box::new(|mut rng, _| {
                            //     for _ in 0..1200 {
                            //         rng.next_u32();
                            //     }
                            //     // napstablook
                            //     if rng.next_f64(100.) <= 50. {
                            //         Box::new(iter::once((rng, 0, None)))
                            //     } else {
                            //         Box::new(iter::empty())
                            //     }
                            // }),
                        ]
                    },
                    |steps, seed| {
                        let mut res = apply_steps(
                            &steps,
                            0,
                            PrecomputedRNG::new(seed, false, false, false, 50_000),
                        );

                        for i in res.iter_mut() {
                            i.1.push(format!("seed {seed}"));
                        }

                        res
                    },
                )
                .flatten()
                .collect::<Vec<_>>();

            for manip in all_manips.iter() {
                println!("{:?}", manip)
            }

            println!(
                "{:?} 0f manips",
                all_manips.iter().filter(|manip| manip.0 == 0).count()
            );

            println!(
                "{:?} 1f manips",
                all_manips.iter().filter(|manip| manip.0 == 1).count()
            );

            println!(
                "{:?} 2f manips",
                all_manips.iter().filter(|manip| manip.0 == 2).count()
            );

            println!(
                "{:?} 3f manips",
                all_manips.iter().filter(|manip| manip.0 == 3).count()
            );

            stdout().flush().unwrap();

            done.store(true, std::sync::atomic::Ordering::Relaxed);
        })
    }

    while !done.load(Ordering::Relaxed) {
        eprintln!("Processing... {}/{total}", progress.get());

        sleep(Duration::from_secs(1));
    }
}

const NAMING_SCREEN: Textbox<2> = Textbox {
    base: 751,
    tbs: &[
        (3, "delay on \"is this name correct\""),
        (104, "delay on naming screen"),
    ],
};

const FLOWEY_PREDIALOGUE: Textbox<7> = Textbox::<7> {
    base: 514,
    tbs: &[
        (84, "howdy im flowey flowey the flower"),
        (18, "hmm..."),
        (88, "youre new to the underground, arentcha"),
        (68, "golly you must be so confused"),
        (
            118,
            "someone ought to teach you how things work around here",
        ),
        (82, "I guess little old me will have to do"),
        (42, "ready here we go"),
    ],
};

const FLOWEY_DIALOGUE_BEFORE_WINKSTAR: Textbox<4> = Textbox {
    base: 492,
    tbs: &[
        (
            628,
            "see that heart thats your soul the very essence of your being",
        ),
        (574, "whats lv stand for? why love of course"),
        (550, "you want some love dont you"),
        (564, "dont worry ill share some  with you"),
    ],
};

const TORIEL_TERRIBLE_CREATURE_DIALOGUE: Textbox<7> = Textbox {
    base: 602,
    tbs: &[
        (
            104,
            "what a terrible creature torturing such a poor innocent youth",
        ),
        (60, "do not be afraid my child"),
        (68, "i am toriel caretaker of the ruins"),
        (
            110,
            "i pass through this place every day to see if someone has fallen down",
        ),
        (100, "you are the first human to come here in a long time"),
        (127, "come i will guide you through the catocombs"),
        (22, "This way"),
    ],
};

const TORIEL_RUINS2_DIALOGUE: Textbox<6> = Textbox {
    base: 578,
    tbs: &[
        (84, "welcome to your new home innocent one"),
        (116, "allow me to educate you in the operation of the ruins"),
        (67, "the ruins are full of puzzles"),
        (105, "ancient fusions between diversions and doorkeys"),
        (101, "one must solve them to move from room to room"),
        (99, "please adjust yourself to the sight of them"),
    ],
};

const FEARLESSES: [(usize, i32, Option<&str>); 7] = [
    (0, 0, Some("fearless none")),
    (0, 182, Some("fearless xz")),
    (0, 364, Some("fearless x.z")),
    (0, 546, Some("fearless x..z")),
    (0, 184, Some("fearless .xz")),
    (0, 366, Some("fearless .x.z")),
    (0, 188, Some("fearless ..xz")),
];

const TORIEL_RUINS3_DIALOGUE: Textbox<0> = Textbox {
    base: 864,
    tbs: &[],
};

const TORIEL_RUINS4_DIALOGUE: Textbox<15> = Textbox {
    base: 1621,
    tbs: &[
        (
            133,
            ("as a human living in the underground, monsters may attack you"),
        ),
        (103, "you will need to be prepared for this"),
        (91, "worry not the process is simple"),
        (116, ("when you encounter a monster you will enter a fight")),
        (
            128,
            ("while youre in a fight, strike up a friendly conversation"),
        ),
        (112, ("stall for time i will settle the conflict")),
        (66, ("practice talking to  the dummy (first time)")),
        (113, ("well i often start with a simple how do you do")),
        (93, "jokes can be useful for breaking the ice"),
        (47, "listen to this one"),
        (89, "what did the skeleton tile his roof with"),
        (33, "shin-gles"),
        (11, "... (ruins4)"),
        (68, "well i thought it was amusing"),
        (
            120,
            ("you can say anything you want, the dummy will not be bothered"),
        ),
    ],
};

const TAWS: [(usize, i32, Option<&str>); 7] = [
    (0, 188, Some("TAW xzxz")),
    (0, 308, Some("TAW xzx.z")),
    (0, 428, Some("TAW xzx..z")),
    (0, 190, Some("TAW xz.xz")),
    (0, 310, Some("TAW xz.x.z")),
    (0, 192, Some("TAW xz..xz")),
    (0, 256, Some("TAW .x.zxz")),
];

const TORIEL_RUINS6_BEGINNING_DIALOGUE: Textbox<5> = Textbox {
    base: 401,
    tbs: &[
        (97, "you have done excellently thus far my child"),
        (111, "however i have a very difficult request"),
        (9, "... (ruins6 beginning)"),
        (
            128,
            "i would like you to walk to the end of this room by yourself",
        ),
        (44, "forgive me for this"),
    ],
};

const TORIEL_RUINS6_ENDING_DIALOGUE: Textbox<9> = Textbox {
    base: 981,
    tbs: &[
        (117, "greetings my child do not worry i did not leave you"),
        (99, "i was merely behind this pillar the whole time"),
        (57, "thank you for trusting me"),
        (
            122,
            "however there was an important reason for this exercise",
        ),
        (67, "to test your independence"),
        (
            145,
            "i must attend to some business and you must stay alone for a while",
        ),
        (
            125,
            "please remain here its dangerous to explore by yourself",
        ),
        (92, "if you have a need for antrhing just call "),
        (38, "be good alright (ruins6)"),
    ],
};

const TORIEL_RUINS7_DIALOGUE: Textbox<6> = Textbox {
    base: 485,
    tbs: &[
        (20, "Ring (ruins7)"),
        (49, "Hello this is toriel (ruins7)"),
        (79, "you have not left the room have you"),
        (121, "there are a few puzzles ahead i have yet to explain"),
        (112, "it would be dangerous to try and solve them yourself"),
        (38, "be good alright (ruins7)"),
    ],
};

const CANDY_TAKE: Textbox<1> = Textbox {
    base: 820,
    tbs: &[(118, "you took a piece of candy press c to open the menu")],
};

const ONEROCK_FIRSTCALL: Textbox<4> = Textbox {
    base: 817,
    tbs: &[
        (20, "Ring (onerock)"),
        (50, "Hello this is toriel (onerock)"),
        (11, "For no reason in particular which do you prefer"),
        (68, "Oh I see thank you very much"),
    ],
};

// needs to stay as a regularlist because of the choose thing for now. this does mean we miss some 2fs but idc
const ONEROCK_SECONDCALL: [(usize, i32, Option<&str>); 12] = [
    (0, 573, Some("choose cinnamon")),
    (
        1,
        658,
        Some("choose cinnamon; you do not dislike butterscotch do you"),
    ),
    (
        1,
        654,
        Some("choose cinnamon; i know what your preference is but"),
    ),
    (
        1,
        697,
        Some("choose cinnamon; would you turn up your nose at it if you found it on your plate"),
    ),
    (1, 633, Some("choose cinnamon; right right i understand")),
    (
        1,
        659,
        Some("choose cinnamon; Thank you for being patient by the way"),
    ),
    (0, 565, Some("choose butterscotch")),
    (
        1,
        642,
        Some("choose butterscotch; you do not dislike cinnamon do you"),
    ),
    (
        1,
        646,
        Some("choose butterscotch; i know what your preference is but"),
    ),
    (
        1,
        689,
        Some(
            "choose butterscotch; would you turn up your nose at it if you found it on your plate",
        ),
    ),
    (
        1,
        625,
        Some("choose butterscotch; right right i understand"),
    ),
    (
        1,
        651,
        Some("choose butterscotch; Thank you for being patient by the way"),
    ),
];

const ONEROCK_SIGN: Textbox<1> = Textbox {
    base: 112,
    tbs: &[(112, "Three out of four gray rocks recommend you push them")],
};

const GRUMPYROCK: Textbox<5> = Textbox {
    base: 394,
    tbs: &[
        (120, "whoa there pardner who said you could push me around"),
        (84, "hmm so youre askin me to move over"),
        (62, "okay just for you pumpkin"),
        (78, "hmm you want me to move some more? "),
        (46, "alrighty hows this"),
    ],
};

struct Textbox<const N: usize> {
    base: u32,
    tbs: &'static [(usize, &'static str); N],
}

fn step_tb<const N: usize>(tb: Textbox<N>) -> Step {
    Box::new(move |mut rng: PrecomputedRNG, max_frames_lost: usize| {
        for _ in 0..tb.base {
            rng.next_u32();
        }
        Box::new((0..=max_frames_lost).flat_map(move |frames_lost| {
            let rng = rng.clone();
            tb.tbs
                .iter()
                .combinations_with_replacement(frames_lost)
                .map(move |tbs| {
                    let mut rng = rng.clone();

                    let mut manips = vec![];

                    for (calls, tb) in tbs {
                        manips.push(*tb);
                        for _ in 0..*calls {
                            rng.next_u32();
                        }
                    }

                    (rng, frames_lost, Some(manips.join(", ")))
                })
        }))
    })
}

fn step_regularlist(list: &'static [(usize, i32, Option<&'static str>)]) -> Step {
    Box::new(|rng: PrecomputedRNG, max_frames_lost: usize| {
        Box::new(
            list.iter()
                .filter(move |(frames_lost, _, _)| *frames_lost <= max_frames_lost)
                .map(move |(frames_lost, calls, which_manip)| {
                    let mut rng = rng.clone();

                    for _ in 0..*calls {
                        rng.next_u32();
                    }

                    (rng, *frames_lost, which_manip.map(str::to_owned))
                }),
        )
    })
}

fn step_fun(mut rng: PrecomputedRNG, _fl: usize) -> StepResult {
    let fun = (f64::floor(rng.next_f64(100.)) + 1.) as u32;

    if fun >= 40 && fun <= 50 {
        return Box::new(iter::empty());
    }

    if fun == 66 {
        return Box::new(iter::once((rng, 0, Some(format!("Fun value {fun}")))));
    }
    return Box::new(iter::empty());
}

fn step_flowey_until_pellets(mut rng: PrecomputedRNG, _fl: usize) -> StepResult {
    // winkstar
    rng.action_move("000000001");

    for _ in 0..820 {
        rng.next_u32();
    }
    // friendly pellets

    for _ in 0..5 {
        rng.action_move("000010000");
    }

    for _ in 0..4004 {
        rng.next_u32();
    }

    Box::new(iter::once((rng, 0, None)))
}

// todo add deliberately buffering extra and losing a frame
fn step_encounterskip(range: f64, min_good: usize, room: &'static str) -> Step {
    Box::new(move |mut rng, fl| {
        let step_count = f64::round(rng.next_f64(range)) as usize;

        if step_count >= min_good {
            // todo taking extra steps to get a blcon
            Box::new(iter::once((rng, 0, None)))
        } else {
            let minimum_buffer = {
                let mut rng = rng.clone();
                let frames_lost = (min_good - 1) - step_count;
                for _ in 0..3 {
                    rng.next_u32();
                }

                (
                    rng,
                    frames_lost,
                    Some(format!("buffer {frames_lost} times in {room}, get blcon")),
                )
            };

            let one_extra = {
                let frames_lost = min_good - step_count;
                (
                    rng,
                    frames_lost,
                    Some(format!(
                        "buffer {frames_lost} times in {room}, don't get blcon"
                    )),
                )
            };

            Box::new([minimum_buffer, one_extra].into_iter())
        }
    })
}

fn step_ruins9_encounter(mut rng: PrecomputedRNG, fl: usize) -> StepResult {
    let ruins9_sc = f64::round(rng.next_f64(40.)) as usize;

    let minimum_buffer = {
        let mut rng = rng.clone();
        for _ in 0..3 {
            rng.next_u32();
        }

        (
            rng,
            40 - ruins9_sc,
            Some(format!("{ruins9_sc} step count ruins 9")),
        )
    };
    let one_extra_buffer = {
        (
            rng,
            40 - ruins9_sc + 1,
            Some(format!(
                "{ruins9_sc} step count ruins 9, and buffer an extra time"
            )),
        )
    };

    Box::new([minimum_buffer, one_extra_buffer].into_iter())
}

fn step_ruins11_stepcount(mut rng: PrecomputedRNG, _fl: usize) -> StepResult {
    let ruins11_sc = rng.next_f64(60.);

    // 11.5 - 12.5 is 1f early

    if ruins11_sc < 11.5 {
        // way early
        Box::new(iter::empty())
    } else if ruins11_sc < 12.5 {
        // 11.5 - 12.5 1f early
        Box::new(iter::once((rng, 1, Some("buffer ruins11".to_owned()))))
    } else if ruins11_sc < 13.5 {
        // 12.5-13.5 on it
        Box::new(iter::once((rng, 0, None)))
    } else if ruins11_sc < 14.5 {
        // 13.5 - 14.5 1f late
        Box::new(iter::once((rng, 1, Some("extra step ruins11".to_owned()))))
    } else {
        Box::new(iter::empty())
    }
}

fn step_ruins11_rest(mut rng: PrecomputedRNG, _fl: usize) -> StepResult {
    let _sc = rng.next_u32();

    let battlegroup = rng.next_f64(20.);

    // triple mold: 338
    // single mold: 230
    // double: 286

    let encounter_calls = match battlegroup {
        // frog whim; cancel
        0.0..=5. => return Box::new(iter::empty()),
        // single mold
        5.0..=10. => 230,
        // triple mold
        10.0..=15. => 338,
        // Double Frog
        15.0..=18. => return Box::new(iter::empty()),
        // double mold
        18.0..=20. => 286,
        _ => unreachable!(),
    };

    let blcon = rng.next_f64(5.);

    if blcon < 0.5 {
        // grumpy rock late text
        for _ in 0..122 {
            rng.next_u32();
        }
        for _ in 0..encounter_calls {
            rng.next_u32();
        }
        // "YOU WON TEXT"
        for _ in 0..156 {
            rng.next_u32();
        }
        Box::new(iter::once((rng, 0, None)))

    // we skip 16 and 17f blcons because they are confusing. only cuts 1 manip.
    // } else if blcon < 2.5 {
    //     Box::new(iter::once((rng, 1, Some("blcon is too long".into()))))
    } else {
        Box::new(iter::empty())
    }
}

fn step_phonecall(mut rng: PrecomputedRNG, _fl: usize) -> StepResult {
    for _ in 0..999 {
        rng.next_u32();
    }
    Box::new(iter::once((rng, 0, None)))
}

fn apply_steps(
    steps: &[Step],
    total_max_frames_lost: usize,
    rng: PrecomputedRNG,
) -> Vec<(usize, Vec<String>)> {
    let mut manips = vec![];

    recursively_apply_steps(
        &steps,
        0,
        total_max_frames_lost,
        rng,
        &mut manips,
        vec![],
        &mut vec![],
        0,
    );

    manips = manips
        .into_iter()
        .map(|manip| {
            (
                manip.0,
                manip.1.into_iter().filter(|x| x.len() > 0).collect(),
            )
        })
        .collect();

    return manips;
}
fn recursively_apply_steps(
    steps: &[Step],
    frames_lost: usize,
    total_max_frames_lost: usize,
    rng: PrecomputedRNG,
    manips: &mut Vec<(usize, Vec<String>)>,
    curr_manip: Vec<String>,
    already_checked_queue: &mut Vec<Vec<(usize, PrecomputedRNG)>>,
    depth: usize,
) {
    if steps.is_empty() {
        manips.push((frames_lost, curr_manip));
        return;
    }

    if already_checked_queue.len() < depth + 1 {
        already_checked_queue.push(vec![]);
    }

    let my_depth_already_checked_queue = &mut already_checked_queue[depth];

    for (new_frames_lost, new_rng) in my_depth_already_checked_queue.iter() {
        if *new_frames_lost <= frames_lost && *new_rng == rng {
            return;
        }
    }

    my_depth_already_checked_queue.push((frames_lost, rng.clone()));

    let my_step = &steps[0];

    for (new_rng, new_frames_lost, which_manip) in
        my_step(rng.clone(), total_max_frames_lost - frames_lost)
    {
        if frames_lost + new_frames_lost > total_max_frames_lost {
            continue;
        }
        let mut curr_manip = curr_manip.clone();

        if let Some(which_manip) = which_manip {
            curr_manip.push(which_manip);
        }
        recursively_apply_steps(
            &steps[1..],
            frames_lost + new_frames_lost,
            total_max_frames_lost,
            new_rng,
            manips,
            curr_manip.clone(),
            already_checked_queue,
            depth + 1,
        );
    }
}

// input is RNG, max frames lost, output is RNG, actual frames lost
type StepResult = Box<dyn Iterator<Item = (PrecomputedRNG, usize, Option<String>)>>;
type Step = Box<dyn Fn(PrecomputedRNG, usize) -> StepResult>;
