//! Fairy Tale Tails
//! Logical Nonsense and DyXel
use super::prelude::*;

pub const ID: u32 = 91957038;

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
    // Normal Summon/Set 1 Spellcaster with 1850 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.nscost)
    effect::set_cost(d, e2, Some(cost_target(nscost)));
    // e2:SetTarget(s.nstg)
    effect::set_target(d, e2, Some(cost_target(nstg)));
    // e2:SetOperation(s.nsop)
    effect::set_operation(d, e2, Some(operation(nsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // The first time each turn you would take damage while you control a Spellcaster with 1850 ATK, you take no damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_DAMAGE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(lua_fn(damcon)));
    // e3:SetValue(s.damval)
    effect::set_value_fn(d, e3, lua_fn(damval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e4, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.nsfilter(c,tp)`
fn nsfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttack(1850) and c:IsRace(RACE_SPELLCASTER) and c:CanSummonOrSet(true,nil) and not c:IsPublic()
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,c:GetCode()),tp,LOCATION_MZONE,0,1,nil)
    if !(card::is_attack(d, c, &[1850]) && card::is_race(d, c, RACE_SPELLCASTER) && card::can_summon_or_set(d, c, true, None, 0, None) && !card::is_public(d, c)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(move |d, x| card::is_code(d, x, &[code]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.nscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| nsfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| nsfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SummonOrSet(tp,tc,true,nil)
        duel::summon_or_set(d, a.tp, tc, true, None, 0, None);
    }
}
/// `s.damcon(e)`
fn damcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return not e:GetHandler():HasFlagEffect(id) and Duel.IsExistingMatchingCard(s.damfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let h = effect::get_handler(d, e).nil();
    if card::has_flag_effect(d, h, ID, None) {
        return false;
    }
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&damfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.damfilter(c)`
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetBaseAttack()==1850 and c:IsRace(RACE_SPELLCASTER) and c:IsFaceup()
    card::get_base_attack(d, c) == 1850 && card::is_race(d, c, RACE_SPELLCASTER) && card::is_faceup(d, c)
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if r&(REASON_BATTLE|REASON_EFFECT)>0 then
    if r & (REASON_BATTLE | REASON_EFFECT) > 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // return 0
        return 0;
    }
    // return val
    val
}
