//! Dark Fusion
use super::prelude::*;

pub const ID: u32 = 94820406;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon 1 fiend fusion monster
    // Using monsters from hand or field as material
    // c:RegisterEffect(Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_FIEND),stage2=s.stage2}))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_FIEND))),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e1, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    if chk == 1 {
        // Cannot be targeted by opponent's card effects
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3061);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value_fn(d, e1, aux::tgoval());
        card::register_effect(d, tc.nil(), e1, false);
    }
}
