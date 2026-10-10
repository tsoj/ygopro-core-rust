//! Magistus Invocation
use super::prelude::*;

pub const ID: u32 = 34933456;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff({handler=c,matfilter=s.mfilter,extrafil=s.fextra})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::mat_filter_fn(|d, x, _e, _tp, _chk| mfilter(d, x))),
            extrafil: Some(std::sync::Arc::new(fextra)),
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
    // return (c:IsLocation(LOCATION_HAND|LOCATION_MZONE) and c:IsAbleToGrave())
    //     or (c:IsOriginalType(TYPE_MONSTER) and c:IsLocation(LOCATION_SZONE) and c:IsType(TYPE_EQUIP) and c:GetEquipTarget():IsSetCard(SET_MAGISTUS))
    (card::is_location(d, c, LOCATION_HAND | LOCATION_MZONE) && card::is_able_to_grave(d, c, None))
        || (card::is_original_type(d, c, TYPE_MONSTER)
            && card::is_location(d, c, LOCATION_SZONE)
            && card::is_type(d, c, TYPE_EQUIP)
            && {
                let et = card::get_equip_target(d, c).nil();
                card::is_set_card(d, et, &[SET_MAGISTUS as u16])
            })
}
/// `s.checkmat(tp,sg,fc)`
fn checkmat(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    let fc = a.c(2);
    // return (fc:IsSetCard(SET_MAGISTUS) or not sg:IsExists(Card.IsLocation,1,nil,LOCATION_SZONE)) and sg:IsExists(Card.IsRace,1,nil,RACE_SPELLCASTER)
    (card::is_set_card(d, fc, &[SET_MAGISTUS as u16]) || !group::is_exists(d, &sg, &|d, x| card::is_location(d, x, LOCATION_SZONE), 1, NO_EX))
        && group::is_exists(d, &sg, &|d, x| card::is_race(d, x, RACE_SPELLCASTER), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil),s.checkmat
    let g = duel::get_matching_group(d, Some(&|d, c| mfilter(d, c)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, NO_EX);
    (Some(g), Some(lua_fn(checkmat)))
}
