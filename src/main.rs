use std::{cmp, iter};

use itertools::Itertools;
use undertale_ruins_analysis::{
    layer::{Layer, LayerIter, LayerResult},
    node_heap::{NodeHandle, NodeHeap},
    rng::RNG,
    switch_manips::SWITCH_MANIPS,
    visitor::run_visitor,
};

fn main() {
    let layers: [Box<Layer>; _] = [
        Box::new(layer_seeds),
        Box::new(layer_naming_screen),
        layer_filter_on_rng("fun_filter", |rng| {
            let fun = (f64::floor(rng.next_f64(100.)) + 1.) as u32;

            if fun >= 40 && fun <= 50 {
                return None;
            };

            if fun == 66 {
                return None;
            }

            return Some(format!("fun value {fun}"));
        }),
        layer_tbs("flowey_predialog", &FLOWEY_PREDIALOGUE),
        layer_straight_modify("flowey_until_pellets", |rng| {
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
        }),
        layer_filter_on_rng("pellet_speed", |rng| {
            let pellet_speed = rng.next_f64(0.5);

            if pellet_speed > 0.48128 {
                Some(format!("pellet speed {pellet_speed}"))
            } else {
                None
            }
        }),
        layer_straight_modify("post flowey tbs", step_n::<11460>),
        layer_straight_modify("pretori action_move", |rng| rng.action_move("000100000")),
        layer_straight_modify(
            "fixed calls in pre-ruins7",
            step_n::<FIXED_CALLS_PRE_RUINS_7>,
        ),
        layer_zero_frames("early ruins zeros", &SWITCH_MANIPS),
        layer_encounterskip("room_ruins7", 40., 16),
        layer_encounterskip("room_ruins8", 40., 6),
        layer_encounterskip("room_ruins9", 40., 41),
        layer_tbs("onerock_firstcall", &ONEROCK_FIRSTCALL),
        // delaying on these might help but its lowk a pain
        layer_zero_frames(
            "bscotch v cinna",
            &[(573, "choose cinnamon"), (565, "choose butterscotch")],
        ),
        layer_tbs("onerock_sign", &ONEROCK_SIGN),
        // ruins10. encounterskip will try to get us to get a blcon here which will take decades
        layer_filter_on_rng("ruins10", |rng| {
            Some(format!("ruins10 has step count {}", rng.next_f64(140.)))
        }),
        Box::new(ruins11sc),
        // todo delay on these
        layer_tbs("grumpyrock", &GRUMPYROCK),
        layer_filter_on_rng("ruins11 secondsc", |rng| {
            Some(format!(
                "ruins11 has second step count {}",
                rng.next_f64(100.)
            ))
        }),
        layer_filter_on_rng("ruins11 blcon and encounter", |rng| {
            let battlegroup = rng.next_f64(20.);

            let blcon = rng.next_f64(5.);

            if blcon > 0.5 {
                return None;
            }
            let (encounter_calls, tag) = match battlegroup {
                // frog whim; cancel
                0.0..=5. => return None,
                // single mold
                5.0..=10. => (230, "3rr single mold"),
                // triple mold
                10.0..=15. => (338, "3rr triple mold"),
                // Double Frog
                15.0..=18. => return None,
                // double mold
                18.0..=20. => (286, "3rr double mold"),
                _ => unreachable!(),
            };

            for _ in 0..encounter_calls {
                rng.next_u32();
            }

            Some(tag.to_owned())
        }),
        layer_straight_modify("goofyrock text2", step_n::<122>),
        layer_straight_modify("goofyrock corrupt text", step_n::<156>),
        Box::new(layer_approx_zero_frames),
        layer_straight_modify("napstablook_prefight", step_n::<816>),
        layer_straight_modify("napstablook fightstart", step_n::<384>),
        layer_filter_on_rng("napstablook_commandattack", |rng| {
            let blookcommand = rng.next_f64(100.).round() as usize;

            if blookcommand <= 50 {
                Some("napstablook has good attack".to_owned())
            } else {
                None
            }
        }),
        layer_straight_modify("napstablook postfight text", step_n::<1490>),
        // this is actually a straightmodify, but we use a filter to attach it
        layer_filter_on_rng("napstablook last attack", |rng| {
            let blookcommand = rng.next_f64(100.).round() as usize;

            if blookcommand <= 50 {
                Some(format!(
                    "napstablook fake attack is crygen1 ({})",
                    rng.num_calls - 1 - rng.calls_the_tas_mod_cant_see
                ))
            } else {
                rng.next_u32();
                rng.next_u32();
                Some(format!(
                    "napstablook fake attack is crygen2 ({})",
                    rng.num_calls - 3 - rng.calls_the_tas_mod_cant_see
                ))
            }
        }),
        layer_tbs("post napstablook fight tbs", &BLOOKY_POSTFIGHT),
        layer_encounterskip("room_ruins13", 140., 36),
        // TODO duplicate on these - PITA because of blinking
        layer_straight_modify("ruins13 phone call", step_n::<809>),
        layer_encounterskip("room_ruins14", 120., 52),
        layer_filter_on_rng("ruins15A", |rng| {
            Some(format!("ruins15A has step count {}", rng.next_f64(140.)))
        }),
        layer_encounterskip("room_ruins15B", 100., 3),
        layer_tbs("ruins15B switch", &SWITCHES),
        layer_encounterskip("room_ruins15C switch", 100., 17),
        layer_tbs("ruins15B switch", &SWITCHES),
        layer_encounterskip("room_ruins15D switch", 100., 57),
        layer_tbs("ruins15B switch", &SWITCHES),
        // switching any of these to TB calls would require us to do blink compensation
        // which is a pain in the ass. I will consider juicing the squeeze if i cant
        // find any 1fs but otherwise its a big pain for a small benefit.
        layer_zero_frames("toriel heal", &[(4607, "heal"), (4631, "dont heal")]),
        layer_filter_on_rng("toriel snacts (snail facts)", |rng| {
            let r = rng.num_calls - rng.calls_the_tas_mod_cant_see;
            let snacti = rng.next_f64(3.).round() as u64;

            let (name, calls) = match snacti {
                0 => ("radula snact", 2540),
                1 => ("digest snact", 2560),
                2 => ("shoelaces snact", 2498),
                3 => ("quiet snact", 2560),
                _ => unreachable!(),
            };

            for _ in 0..calls {
                rng.next_u32();
            }

            return Some(format!("{name}, {r}"));
        }),
        layer_filter_on_rng("expected calls entering tori", |rng| {
            let r = rng.num_calls - rng.calls_the_tas_mod_cant_see;
            return Some(format!("expect {r} calls entering toriel"));
        }),
        Box::new(layer_toriel1),
        Box::new(layer_toriel2),
    ];
    run_visitor(&layers);
    const FIXED_CALLS_PRE_RUINS_7: usize = 602 + 578 + 276 + 767 + 856 + 401 + 981 + 485 + 818 + 1;
}

fn step_n<const N: usize>(r: &mut RNG) {
    for _ in 0..N {
        r.next_u32();
    }
}

fn ruins11sc(nh: &'static NodeHeap, node: NodeHandle, _budget: u32) -> LayerIter {
    let mut rng = nh.get_rng(node).unwrap();

    let step_count = f64::round(rng.next_f64(60.)) as usize;

    let frames_lost = usize::abs_diff(step_count, 13);

    Box::new(iter::once(LayerResult {
        child: nh.get_or_construct_node("ruins11 step count", Some(rng)),
        layer_cost: frames_lost as u32,
        layer_path: Some(format!(
            "ruins 11 has step count {step_count} losing {frames_lost}"
        )),
    }))
}

fn layer_approx_zero_frames(nh: &'static NodeHeap, node: NodeHandle, _budget: u32) -> LayerIter {
    let rng = nh.get_rng(node).unwrap();
    Box::new((460..=3590).map(move |offset| {
        let mut rng = rng.clone();

        for _ in 0..offset {
            rng.next_u32();
        }
        LayerResult {
            child: nh.get_or_construct_node("ruins11 zero frames", Some(rng)),
            layer_cost: 0,
            layer_path: Some(format!("call rng {offset} times in the cell phone room")),
        }
    }))
}
fn layer_encounterskip(layer_id: &'static str, range: f64, min_good: usize) -> Box<Layer> {
    Box::new(
        move |nh: &'static NodeHeap, node: NodeHandle, _budget: u32| {
            let mut rng = nh.get_rng(node).unwrap();
            let step_count = f64::round(rng.next_f64(range)) as usize;

            let rng_prebl = rng.clone();

            let frames_lost = if step_count >= min_good {
                0
            } else {
                rng.next_u32();
                rng.next_u32();
                rng.next_u32();

                min_good - 1 - step_count
            };

            let delayer = if step_count >= min_good {
                let mut rng = rng.clone();
                rng.next_u32();
                rng.next_u32();
                rng.next_u32();
                let child = nh.get_or_construct_node(layer_id, Some(rng));

                let frames_to_delay = step_count - (min_good - 1);

                if frames_to_delay == 0 {
                    panic!("wtf, {step_count} and {min_good} give {frames_to_delay}")
                }

                Some(LayerResult {
                    child,
                    layer_cost: frames_to_delay as u32,
                    layer_path: Some(format!(
                        "{layer_id} step count {step_count}, take extra steps to get blcon"
                    )),
                })
            } else {
                let child = nh.get_or_construct_node(layer_id, Some(rng_prebl));

                Some(LayerResult {
                    child,
                    layer_cost: frames_lost as u32 + 1,
                    layer_path: Some(format!(
                        "{layer_id} has {step_count}, buffer an extra time to skip the blcon"
                    )),
                })
            };

            let repoff = rng.num_calls - rng.calls_the_tas_mod_cant_see;
            let child = nh.get_or_construct_node(layer_id, Some(rng));

            Box::new(
                iter::once(LayerResult {
                    child,
                    layer_cost: frames_lost as u32,
                    layer_path: Some(format!(
                        "{layer_id} has {step_count} losing {frames_lost} ({repoff})"
                    )),
                })
                .chain(delayer.into_iter()),
            )
        },
    )
}

fn layer_zero_frames(
    layer_id: &'static str,
    zeros: &'static [(usize, &'static str)],
) -> Box<Layer> {
    Box::new(
        move |nh: &'static NodeHeap, node: NodeHandle, _budget: u32| {
            let rng = nh.get_rng(node);

            Box::new(zeros.iter().map(move |(calls, name)| {
                let mut rng = rng.clone().unwrap();

                for _ in 0..*calls {
                    rng.next_u32();
                }
                LayerResult {
                    child: nh.get_or_construct_node(layer_id, Some(rng)),
                    layer_cost: 0,
                    layer_path: Some(name.to_owned().to_owned()),
                }
            }))
        },
    )
}
fn layer_straight_modify<T: Fn(&mut RNG) + 'static>(
    layer_id: &'static str,
    modifier: T,
) -> Box<Layer> {
    Box::new(
        move |nh: &'static NodeHeap, node: NodeHandle, _budget: u32| {
            let mut rng = nh.get_rng(node).unwrap();

            modifier(&mut rng);

            Box::new(iter::once(LayerResult {
                child: nh.get_or_construct_node(layer_id, Some(rng)),
                layer_cost: 0,
                layer_path: None,
            }))
        },
    )
}

fn layer_filter_on_rng<T: Fn(&mut RNG) -> Option<String> + 'static>(
    layer_id: &'static str,
    modifier: T,
) -> Box<Layer> {
    Box::new(
        move |nh: &'static NodeHeap, node: NodeHandle, _budget: u32| {
            let mut rng = nh.get_rng(node).unwrap();

            match modifier(&mut rng) {
                Some(path) => Box::new(iter::once(LayerResult {
                    child: nh.get_or_construct_node(layer_id, Some(rng)),
                    layer_cost: 0,
                    layer_path: Some(path),
                })),
                None => Box::new(iter::empty()),
            }
        },
    )
}

fn layer_seeds(nh: &'static NodeHeap, _node: NodeHandle, _budget: u32) -> LayerIter {
    let seeds = RNG::calculate_unique_seeds(false, false);
    //let seeds = vec![1024];
    Box::new(seeds.into_iter().map(|x| LayerResult {
        child: nh.get_or_construct_node("seedsLayer", Some(RNG::new(x, false, false, false))),
        layer_cost: 0,
        layer_path: Some(format!("seed {x}")),
    }))
}

fn layer_naming_screen(nh: &'static NodeHeap, node: NodeHandle, budget: u32) -> LayerIter {
    let mut rng = nh.get_rng(node).unwrap();
    for _ in 0..751 {
        rng.next_u32();
    }

    Box::new((0..=budget).map(move |x| {
        let mut rng = rng.clone();

        for _ in 0..104 * x {
            rng.next_u32();
        }
        LayerResult {
            layer_cost: x,
            layer_path: Some(format!("delay {x} on naming screen")),
            child: nh.get_or_construct_node("naming_screen_layer", Some(rng)),
        }
    }))
}

const FLOWEY_PREDIALOGUE: Textbox<11> = Textbox::<11> {
    base: 1006,
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
        (
            628,
            "see that heart thats your soul the very essence of your being",
        ),
        (574, "whats lv stand for? why love of course"),
        (550, "you want some love dont you"),
        (564, "dont worry ill share some  with you"),
    ],
};

const BLOOKY_POSTFIGHT: Textbox<5> = Textbox {
    base: 316,
    tbs: &[
        (
            128,
            "i usually come to the ruins because theres no one around",
        ),
        (70, "but today i met someone nice"),
        (10, "... (napstablook)"),
        (48, "oh im rambling again"),
        (52, "ill get out of your way"),
    ],
};

struct Textbox<const N: usize> {
    base: u32,
    tbs: &'static [(usize, &'static str); N],
}

fn layer_tbs<const N: usize>(layer_id: &'static str, tb: &'static Textbox<N>) -> Box<Layer> {
    Box::new(
        move |nh: &'static NodeHeap, node: NodeHandle, _budget: u32| {
            let mut rng = nh.get_rng(node).unwrap();

            for _ in 0..tb.base {
                rng.next_u32();
            }

            Box::new((0..).flat_map(move |x| {
                let rng = rng.clone();

                tb.tbs
                    .iter()
                    .combinations_with_replacement(x)
                    .map(move |tbs| {
                        let mut rng = rng.clone();

                        let mut manips = vec![];

                        for (calls, tb) in tbs {
                            manips.push(*tb);
                            for _ in 0..*calls {
                                rng.next_u32();
                            }
                        }

                        let node = nh.get_or_construct_node(layer_id, Some(rng));
                        let path = if manips.is_empty() {
                            None
                        } else {
                            Some(format!("delay at {layer_id}: {}", manips.join(",")))
                        };

                        LayerResult {
                            child: node,
                            layer_cost: x as u32,
                            layer_path: path,
                        }
                    })
            }))
        },
    )
}
const ONEROCK_FIRSTCALL: Textbox<4> = Textbox {
    base: 817,
    tbs: &[
        (20, "Ring (onerock)"),
        (50, "Hello this is toriel (onerock)"),
        (11, "For no reason in particular which do you prefer"),
        (68, "Oh I see thank you very much"),
    ],
};
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

const SWITCHES: Textbox<2> = Textbox {
    base: 576,
    tbs: &[
        (102, "its a switch press it"),
        (56, "you hear a mysterious clicking sound"),
    ],
};

// this is a separate subtree because it requires a bunch of information sharing between layers and I didn't want to implement it properly.

fn layer_toriel1(nh: &'static NodeHeap, node: NodeHandle, budget: u32) -> LayerIter {
    let budget = cmp::min(budget, 30);

    // Plan:
    // generator that has all possible lists of actions (so like [zz,zz,zz...], [zz,zz,zxz...])
    // these are in regular order of cost

    // state = (Vec<action> actions, int numFirstHalfBottoms, )
    // manips = actions
    // . flat_map()
    // for each firstAction {

    // loop over all possible lists with 12 normal actions and 1 healing  {
    //    skip while first attack isn't a hand
    //    skip while second attack isn't a hand
    //    skip while third attack isn't a hand
    //    skip while fourth attack isnt a hand
    //    skip while the number of bottoms in the first 4 hands is 0.
    //    skip while the fifth attack isn't a hand (1->5, 2->6, 3-> 7)
    //    ^^ sixth (4, 5, 6)
    //    ^^ seventh (3, 4, 5)
    //    if one bottom {
    //        skip while eith attack isnt fire
    //
    //        for healing action in healing actions.filterOn(gives hand) {
    //            loop over all possible lists of 5 actions
    //            skip while first
    //        }
    //    }
    //
    //}
    Box::new(iter::once(LayerResult {
        child: nh.get_or_construct_node_with_custom_data("toriel thing", nh.get_rng(node), 69420),
        layer_cost: 0,
        layer_path: Some(format!("Toriel budget {budget}")),
    }))
}

fn layer_toriel2(nh: &'static NodeHeap, node: NodeHandle, budget: u32) -> LayerIter {
    let custom_data = nh.get_custom_data::<i32>(node);

    Box::new(iter::once(LayerResult {
        child: nh.get_or_construct_node_with_custom_data("toriel thing", nh.get_rng(node), 69420),
        layer_cost: 0,
        layer_path: Some(format!(
            "Toriel budget {budget}, stored custom data {custom_data:?}"
        )),
    }))
}
