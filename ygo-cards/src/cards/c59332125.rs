//! A.I. Love Fusion
//! Scripted by ahtelel, anime version by Larry126
use super::prelude::*;

pub const ID: u32 = 59332125;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsRace,RACE_CYBERSE),nil,s.fextra)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_CYBERSE as u64))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.chkfilter(c,tp,fc)`
fn chkfilter(d: &mut Duel, c: CardId, tp: u8, fc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_IGNISTER,fc,SUMMON_TYPE_FUSION,tp) and c:IsControler(tp)
    card::is_set_card_ctx(d, c, &[SET_IGNISTER as u16], fc, SUMMON_TYPE_FUSION as u64, Some(tp)) && card::is_controler(d, c, tp)
}
/// `s.fcheck(tp,sg,fc,mg)`
fn fcheck(d: &mut Duel, tp: u8, sg: &CardSet, fc: Option<CardId>) -> bool {
    // if sg:IsExists(Card.IsControler,1,nil,1-tp) then
    if group::is_exists(d, sg, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX) {
        // return sg:IsExists(s.chkfilter,1,nil,tp,fc) end
        return group::is_exists(d, sg, &|d, c| chkfilter(d, c, tp, fc), 1, NO_EX);
    }
    // return true
    true
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if mg:IsExists(Card.IsSetCard,1,nil,SET_IGNISTER,nil,SUMMON_TYPE_FUSION,tp) then
    if group::is_exists(
        d,
        mg,
        &|d, c| card::is_set_card_ctx(d, c, &[SET_IGNISTER as u16], None, SUMMON_TYPE_FUSION as u64, Some(tp)),
        1,
        NO_EX,
    ) {
        // local g=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup,Card.IsLinkMonster,Card.IsAbleToGrave),tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c) && card::is_link_monster(d, c) && card::is_able_to_grave(d, c, None)),
            tp,
            0,
            LOCATION_MZONE,
            NO_EX,
        );
        // if g and #g>0 then
        if !g.is_empty() {
            // return g,s.fcheck
            let fc = lua_fn(|d, a: Args| fcheck(d, a.int(0) as u8, &a.g(1), a.card(2)));
            return (Some(g), Some(fc));
        }
    }
    // return nil
    (None, None)
}
