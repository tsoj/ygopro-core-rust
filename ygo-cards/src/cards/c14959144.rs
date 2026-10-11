//! Ultimate Bright Knight Ursatron Alpha
//! 天極輝士－熊斗竜巧α
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 14959144;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC, SET_DRYTRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Normal Summoned/Set
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // Must be Special Summoned by a card effect
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(function(e,se) return se:IsHasType(EFFECT_TYPE_ACTIONS) end)
    effect::set_value_fn(d, e0, lua_fn(|d, a: Args| effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIONS)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon itself from the hand and search 1 "Ursarctic" or "Drytron" Spell/Trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_URSARCTIC,SET_DRYTRON}) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_URSARCTIC as u16, SET_DRYTRON as u16]) && !card::is_code(d, c, &[ID])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_URSARCTIC,SET_DRYTRON}) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_URSARCTIC as u16, SET_DRYTRON as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local sg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let mut sg = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !sg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // sg=sg:Select(tp,1,1,nil)
            sg = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    }
}
