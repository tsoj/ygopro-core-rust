//! Spiral Fusion
use super::prelude::*;

pub const ID: u32 = 38590361;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GAIA_CHAMPION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:RegisterEffect(Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON),stage2=s.stage2}))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_race(d, x, RACE_DRAGON as u64))),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e1, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    if chk == 1 && card::is_code(d, tc.nil(), &[CARD_GAIA_CHAMPION]) {
        let tc = tc.nil();
        let h = effect::get_handler(d, e).nil();
        // atk gain
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e1, 2600);
        card::register_effect(d, tc, e1, false);
        // attack twice
        let e2 = effect::create_effect(d, h);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_description(d, e2, aux::stringid(ID, 1));
        effect::set_value(d, e2, 1);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
    }
}
