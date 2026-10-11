//! Fairy Tail - Sleeper
use super::prelude::*;

pub const ID: u32 = 42921475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // FLIP: You can Special Summon 1 monster from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the activated effect of an opponent's Normal Spell/Trap to "Change 1 face-up monster your opponent controls to face-down Defense Position"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.chcon)
    effect::set_condition(d, e2, Some(condition(chcon)));
    // e2:SetCost(s.chcost)
    effect::set_cost(d, e2, Some(cost_target(chcost)));
    // e2:SetTarget(s.chtg)
    effect::set_target(d, e2, Some(cost_target(chtg)));
    // e2:SetOperation(s.chop)
    effect::set_operation(d, e2, Some(operation(chop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // return rp==1-tp and rc:IsNormalSpellTrap() and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    a.rp == 1 - a.tp && card::is_normal_spell_trap(d, rc) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
}
/// `s.chcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,c) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,c)
    let g = duel::select_release_group_cost(d, a.tp, None, 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsCanTurnSet,rp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_turn_set(d, c)), a.rp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, lua_fn(repop));
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`, the replacement chain operation
fn repop(d: &mut Duel, a: Args) {
    let (e, tp) = (a.e(0), a.p(1));
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsNormalSpellTrap() then c:CancelToGrave(false) end
    if card::is_normal_spell_trap(d, c) {
        card::cancel_to_grave(d, c, false);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| card::is_can_turn_set(d, cc)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE) end
    if !g.is_empty() {
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
