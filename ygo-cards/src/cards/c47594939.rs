//! And the Band Played On
//! 召喚制限－ディスコードセクター
use super::prelude::*;

pub const ID: u32 = 47594939;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot Special Summon monsters with the same Level/Rank
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.splimit(e,c,sump,sumtype,sumpos,targetp)
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sump = a.p(2);
    // if c:HasLevel() then
    if card::has_level(d, c) {
        // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevel,c:GetLevel()),sump,LOCATION_MZONE,0,1,nil)
        let lv = card::get_level(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_level(d, x, &[lv])), sump, LOCATION_MZONE, 0, 1, NO_EX);
    // elseif c:IsRankAbove(0) then
    } else if card::is_rank_above(d, c, 0) {
        // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRank,c:GetRank()),sump,LOCATION_MZONE,0,1,nil)
        let rk = card::get_rank(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_rank(d, x, &[rk])), sump, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // return false
    false
}
