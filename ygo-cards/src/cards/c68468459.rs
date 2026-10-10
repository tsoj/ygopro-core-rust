//! Fallen of Albaz
use super::prelude::*;

pub const ID: u32 = 68468459;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local params={nil,Fusion.CheckWithHandler(aux.FALSE),s.fextra,nil,Fusion.ForcedHandler}
    let params = || fusion::SummonEff {
        fusfilter: None,
        matfilter: Some(fusion::check_with_handler(std::sync::Arc::new(|_d, _c, _e, _tp, _chk| false))),
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: None,
        gc: Some(fusion::forced_handler()),
        ..Default::default()
    };
    // Fusion Summon 1 Fusion Monster from your Extra Deck, using monsters on either field as Fusion Material, including this card, but you cannot use other monsters you control as Fusion Material
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_condition(d, e1a, Some(condition(|d, _a| !duel::is_damage_step(d, None))));
    effect::set_cost(d, e1a, Some(cost::discard(None, false, None, None, None)));
    let tg = fusion::summon_eff_tg(d, params());
    effect::set_target(d, e1a, Some(tg));
    let op = fusion::summon_eff_op(d, params());
    effect::set_operation(d, e1a, Some(op));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
}
/// `s.fcheck(c)`: `function(tp,sg,fc) return not sg:IsExists(aux.AND(Card.IsControler,Card.IsOnField),1,c,tp) end`
fn fcheck(c: CardId) -> LuaFn {
    lua_fn(move |d, a: Args| {
        let tp = a.p(0);
        let sg = a.g(1);
        !group::is_exists(d, &sg, &|d, x| card::is_controler(d, x, tp) && card::is_on_field(d, x), 1, c.into())
    })
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    let h = effect::get_handler(d, e).nil();
    (Some(g), Some(fcheck(h)))
}
