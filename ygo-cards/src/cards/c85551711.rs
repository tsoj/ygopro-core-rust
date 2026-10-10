//! Ebon High Magician
//! 虚空の黒魔導師
use super::prelude::*;

pub const ID: u32 = 85551711;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),7,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, sc, st, Some(tp)))), Some(7), 2, None, None, None, None, false, None);
    // You can activate a Quick-Play Spell or a Trap from your hand during your opponent's turn by detaching 1 material from this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e1, EFFECT_QP_ACT_IN_NTPHAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCondition(s.handcon)
    effect::set_condition(d, e1, Some(condition(handcon)));
    // e1:SetValue(function(e,rc,re) re:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,0,e:GetHandler():GetFieldID()) end)
    effect::set_value_fn(d, e1, lua_fn(qp_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Detach 1 material on activation
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_ACTIVATE_COST)
    effect::set_code(d, e3, EFFECT_ACTIVATE_COST);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetTarget(s.costtg)
    effect::set_target(d, e3, Some(lua_fn(costtg)));
    // e3:SetOperation(s.costop)
    effect::set_operation(d, e3, Some(operation(costop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 DARK Spellcaster monster from your hand or Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.handcon(e)`
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    // return Duel.IsTurnPlayer(1-tp) and e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    duel::is_turn_player(d, 1 - tp) && card::check_remove_overlay_card(d, h, tp, 1, REASON_EFFECT)
}
/// `function(e,rc,re) re:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,0,e:GetHandler():GetFieldID()) end`
fn qp_value(d: &mut Duel, a: Args) {
    // re:GetHandler()
    let re = a.e(2);
    let rh = effect::get_handler(d, re).nil();
    // e:GetHandler():GetFieldID()
    let e = a.e(0);
    let eh = effect::get_handler(d, e).nil();
    let fid = card::get_field_id(d, eh) as i64;
    // RegisterFlagEffect(id,RESET_CHAIN,0,0,label)
    card::register_flag_effect(d, rh, ID, RESET_CHAIN, 0, 0, fid, 0);
}
/// `s.costtg(e,te,tp)`
fn costtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, a.e(1)).nil();
    let tp = a.p(2);
    // return Duel.IsTurnPlayer(1-tp) and tc:IsLocation(LOCATION_HAND) and tc:HasFlagEffect(id)
    //     and tc:GetFlagEffectLabel(id)==e:GetHandler():GetFieldID()
    if !duel::is_turn_player(d, 1 - tp) || !card::is_location(d, tc, LOCATION_HAND) || card::get_flag_effect(d, tc, ID) == 0 {
        return false;
    }
    let label = card::get_flag_effect_label(d, tc, ID).first().copied();
    let eh = effect::get_handler(d, e).nil();
    label == Some(card::get_field_id(d, eh) as i64)
}
/// `s.costop(e,tp,eg,ep,ev,re,r,rp)`
fn costop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // e:GetHandler():RemoveOverlayCard(tp,1,1,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::remove_overlay_card(d, h, a.tp, 1, 1, REASON_EFFECT);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsPreviousLocation(LOCATION_MZONE) and c:IsXyzSummoned()) then return false end
    if !(card::is_previous_location(d, c, LOCATION_MZONE) && card::is_xyz_summoned(d, c)) {
        return false;
    }
    // return c:IsReason(REASON_BATTLE) or (rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp))
    card::is_reason(d, c, REASON_BATTLE) || (a.rp == 1 - a.tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_controler(d, c, a.tp))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if #dg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
        if dg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=dg:Select(tp,1,1,nil)
        let sg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
