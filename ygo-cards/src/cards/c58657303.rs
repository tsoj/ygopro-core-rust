//! Gravekeeper's Spiritualist
use super::prelude::*;

pub const ID: u32 = 58657303;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NECROVALLEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion summon
    // local params = {aux.FilterBoolFunction(Card.IsRace,RACE_SPELLCASTER),nil,nil,nil,Fusion.ForcedHandler}
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    let p = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_SPELLCASTER))),
        gc: Some(fusion::forced_handler()),
        ..Default::default()
    };
    // e1:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, p.clone());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, p);
    effect::set_operation(d, e1, Some(op));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_NECROVALLEY)
    duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
