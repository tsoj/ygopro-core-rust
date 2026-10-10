//! Shaddoll Fusion
use super::prelude::*;

pub const ID: u32 = 44394295;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_SHADDOLL),nil,s.fextra,nil,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_SHADDOLL as u16]))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsExistingMatchingCard(Card.IsSummonLocation,tp,0,LOCATION_MZONE,1,nil,LOCATION_EXTRA) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
