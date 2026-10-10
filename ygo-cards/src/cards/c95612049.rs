//! Turning of the World
use super::prelude::*;

pub const ID: u32 = 95612049;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[46427957, 72426662], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon
    // local e1=Ritual.CreateProc(c,RITPROC_GREATER,aux.FilterBoolFunction(Card.IsCode,46427957,72426662),nil,nil,nil,nil,s.mfilter,nil,LOCATION_HAND|LOCATION_DECK)
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_code(d, x, &[46427957, 72426662]))),
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, mc: CardId, _e: EffectId, _tp: u8| mfilter(d, mc)))),
            location: Some(LOCATION_HAND | LOCATION_DECK),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_HAND) and c:IsType(TYPE_RITUAL)
    card::is_location(d, c, LOCATION_HAND) && card::is_type(d, c, TYPE_RITUAL)
}
