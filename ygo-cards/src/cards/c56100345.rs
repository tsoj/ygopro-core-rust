//! Cyberdark Wurm
use super::prelude::*;

pub const ID: u32 = 56100345;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_CYBER_DRAGON],
    listed_series: &[SET_CYBER, SET_CYBERNETIC, SET_CYBER_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "Cyber Dragon" while on the field or in the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_CYBER_DRAGON)
    effect::set_value(d, e1, CARD_CYBER_DRAGON as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand or GY then you can add 1 "Cyber" Spell/Trap from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(s.cybermachinefilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler()) end)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.cybermachinefilter(c)
fn cybermachinefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CYBER) and c:IsRace(RACE_MACHINE) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_CYBER as u16]) && card::is_race(d, c, RACE_MACHINE) && card::is_faceup(d, c)
}
// function(e,tp) return Duel.IsExistingMatchingCard(s.cybermachinefilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler()) end
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cybermachinefilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, h.into())
}
// s.spcostfilter(c)
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CYBER_DRAGON) and c:IsMonster() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_CYBER_DRAGON as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
// s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&spcostfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
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
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_CYBER,SET_CYBERNETIC}) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_CYBER as u16, SET_CYBERNETIC as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))), tp, LOCATION_GRAVE, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if group::get_count(&g) > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, &sg);
        }
    }
}
