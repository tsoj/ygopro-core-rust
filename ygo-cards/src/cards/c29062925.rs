//! Face Card Fusion
use super::prelude::*;

pub const ID: u32 = 29062925;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,s.ffilter,nil,s.fextra,nil,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| ffilter(d, c))),
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
/// `s.ffilter(c)`
fn ffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_WARRIOR)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JACK_KNIGHT,CARD_QUEEN_KNIGHT,CARD_KING_KNIGHT),tp,LOCATION_ONFIELD,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX) {
        // local sg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_DECK,0,nil)
        let sg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
        // if #sg>0 then return sg,s.fcheck end
        if !sg.is_empty() {
            return (Some(sg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=1
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 1
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
