//! Cataclysmic Crusted Calcifida
//! scripted by Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 9028399;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon from hand, then mill a monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
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
// Player's EARTH monster destroyed by opponent
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:GetPreviousControler()==tp and c:GetPreviousAttributeOnField() & ATTRIBUTE_EARTH ~=0
    //     and c:IsReason(REASON_DESTROY) and ((c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp) or c:IsReason(REASON_BATTLE))
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::get_previous_controler(d, c) == tp
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_EARTH) != 0
        && card::is_reason(d, c, REASON_DESTROY)
        && ((card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp) || card::is_reason(d, c, REASON_BATTLE))
}
// If it ever happened
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// Activation legality
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
// Check for a monster
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
// Special summon from hand, then can mill a monster
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
}
