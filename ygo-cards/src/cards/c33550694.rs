//! Fusion Gate
use super::prelude::*;

pub const ID: u32 = 33550694;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // fusion
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_property(d, e2, EFFECT_FLAG_BOTH_SIDE, 0);
    // Fusion.SummonEffTG(nil,Card.IsAbleToRemove,nil,Fusion.BanishMaterial)
    // Fusion.SummonEffOP(nil,Card.IsAbleToRemove,nil,Fusion.BanishMaterial)
    let p = fusion::SummonEff {
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32)))),
        extraop: Some(fusion::banish_material()),
        ..Default::default()
    };
    let tg = fusion::summon_eff_tg(d, p.clone());
    effect::set_target(d, e2, Some(tg));
    let op = fusion::summon_eff_op(d, p);
    effect::set_operation(d, e2, Some(op));
    card::register_effect(d, c, e2, false);
}
