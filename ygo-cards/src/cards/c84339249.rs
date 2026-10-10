//! Protecting Spirit Loagaeth
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 84339249;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply a "it cannot be destroyed by battle this turn" effect on 1 monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.indestg)
    effect::set_target(d, e1, Some(cost_target(indestg)));
    // e1:SetOperation(s.indesop)
    effect::set_operation(d, e1, Some(operation(indesop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 face-up card your opponent controls and change 1 Attack Position monster you control to Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and r&(REASON_BATTLE|REASON_EFFECT)>0 end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD) && (a.r & (REASON_BATTLE | REASON_EFFECT)) > 0
}
/// `aux.NOT(Card.IsHasEffect)` with `EFFECT_INDESTRUCTABLE_BATTLE`
fn not_indestructable_battle(d: &mut Duel, c: CardId) -> bool {
    card::is_has_effect(d, c, EFFECT_INDESTRUCTABLE_BATTLE, None).is_empty()
}
/// `s.indestg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn indestg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(aux.NOT(Card.IsHasEffect),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,EFFECT_INDESTRUCTABLE_BATTLE) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&not_indestructable_battle), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,aux.NOT(Card.IsHasEffect),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,EFFECT_INDESTRUCTABLE_BATTLE)
    duel::select_target(d, a.tp, Some(&not_indestructable_battle), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.indesop(e,tp,eg,ep,ev,re,r,rp)`
fn indesop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // It cannot be destroyed by battle this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3000)
        effect::set_description(d, e1, 3000);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_race,trig_loc,trig_ctrl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_RACE,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_CONTROLER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_RACE, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CONTROLER]);
    let (trig_race, trig_loc, trig_ctrl) = (info[0].int(), info[1].int(), info[2].int());
    // return trig_race and trig_race&RACE_FAIRY>0 and trig_loc==LOCATION_MZONE and trig_ctrl==tp
    trig_race != 0 && (trig_race & RACE_FAIRY as i64) > 0 && trig_loc == LOCATION_MZONE as i64 && trig_ctrl == a.tp as i64
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return ((c:IsControler(1-tp) and c:IsAbleToRemove() and c:IsFaceup())
    //     or (c:IsControler(tp) and c:IsAttackPos() and c:IsCanChangePosition()))
    //     and c:IsCanBeEffectTarget(e)
    ((card::is_controler(d, c, 1 - tp) && card::is_able_to_remove(d, c, None, None, None) && card::is_faceup(d, c))
        || (card::is_controler(d, c, tp) && card::is_attack_pos(d, c) && card::is_can_change_position(d, c)))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_ONFIELD,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_ONFIELD, NO_EX);
    let dp = aux::dpcheck(card_controler_value);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local rmg,posg=tg:Split(Card.IsControler,nil,1-tp)
    let (rmg, posg) = group::split(d, &tg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    // e:SetLabelObject(rmg:GetFirst())
    let first = group::get_first(d, &rmg);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rmg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rmg), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,posg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(posg), 1, tp, 0);
    true
}
/// `Card.GetControler` as the value of `aux.dpcheck`
fn card_controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local rm_tc=e:GetLabelObject()
    let rm_tc = effect::get_label_object(d, a.e).as_card().nil();
    // if rm_tc:IsRelateToEffect(e) and rm_tc:IsControler(1-tp) and Duel.Remove(rm_tc,POS_FACEUP,REASON_EFFECT)>0
    //     and rm_tc:IsLocation(LOCATION_REMOVED) then
    if card::is_relate_to_effect(d, rm_tc, a.e)
        && card::is_controler(d, rm_tc, 1 - a.tp)
        && duel::remove(d, CardSet::from_card(rm_tc), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, rm_tc, LOCATION_REMOVED)
    {
        // local pos_tc=(tg-rm_tc):GetFirst()
        let mut rest = tg.clone();
        group::remove_card(&mut rest, &CardSet::from_card(rm_tc));
        let pos_tc = group::get_first(d, &rest);
        // if pos_tc and pos_tc:IsRelateToEffect(e) and pos_tc:IsControler(tp) then
        if let Some(pos_tc) = pos_tc {
            if card::is_relate_to_effect(d, pos_tc, a.e) && card::is_controler(d, pos_tc, a.tp) {
                // Duel.ChangePosition(pos_tc,POS_FACEUP_DEFENSE)
                duel::change_position(d, CardSet::from_card(pos_tc), POS_FACEUP_DEFENSE as u8, None, None, None, false);
            }
        }
    }
}
