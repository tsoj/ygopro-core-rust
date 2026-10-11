//! Cross Over
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 53241226;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 face-up monster your opponent controls and 1 Warrior monster you control; equip that opponent's face-up monster to your monster as an Equip Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// aux.FaceupFilter(Card.IsRace,RACE_WARRIOR)
fn warrior_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local f1=aux.FaceupFilter(Card.IsRace,RACE_WARRIOR)
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(f1,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    //   and (ft>1 or (ft>0 and e:GetHandler():IsLocation(LOCATION_SZONE))) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&warrior_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && (ft > 1 || (ft > 0 && card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_SZONE)));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,f1,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&warrior_faceup), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, Value::card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetCards(e)
    // if not g or #g<2 then return end
    let Some(g) = duel::get_target_cards(d, a.e) else { return };
    if g.len() < 2 {
        return;
    }
    // local tc=g:GetFirst()
    // local oc=g:GetNext()
    let order = group::iter(d, &g);
    let mut tc = order[0];
    let mut oc = order[1];
    // if oc==e:GetLabelObject() then tc,oc=oc,tc end
    if Some(oc) == effect::get_label_object(d, a.e).as_card() {
        std::mem::swap(&mut tc, &mut oc);
    }
    // if tc:IsFaceup() and tc:IsLocation(LOCATION_MZONE) and oc:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and tc:IsAbleToChangeControler() and Duel.Equip(tp,oc,tc,false) then
    if card::is_faceup(d, tc)
        && card::is_location(d, tc, LOCATION_MZONE)
        && card::is_faceup(d, oc)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::is_able_to_change_controler(d, tc)
        && duel::equip(d, tp, oc, tc, false, false)
    {
        // Until the end of this turn, your opponent takes no damage from battles involving that monster you control
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_NO_BATTLE_DAMAGE)
        effect::set_code(d, e1, EFFECT_NO_BATTLE_DAMAGE);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // If the equipped monster would be destroyed by battle or card effect, destroy the monster equipped to it by this effect, instead
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_DESTROY_SUBSTITUTE)
        effect::set_code(d, e2, EFFECT_DESTROY_SUBSTITUTE);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // e2:SetValue(s.repval)
        effect::set_value_fn(d, e2, lua_fn(repval));
        // oc:RegisterEffect(e2)
        card::register_effect(d, oc, e2, false);
        // Equip limit
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetValue(s.eqlimit)
        effect::set_value_fn(d, e3, effect_card_fn(eqlimit));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // e3:SetLabelObject(tc)
        effect::set_label_object(d, e3, Value::Card(tc));
        // oc:RegisterEffect(e3)
        card::register_effect(d, oc, e3, false);
    }
}
// function s.eqlimit(e,c) return c==e:GetLabelObject() end
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    Some(c) == effect::get_label_object(d, e).as_card()
}
// function s.repval(e,re,r,rp) return (r&REASON_BATTLE+REASON_EFFECT)~=0 end
fn repval(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2) as u32;
    (r & (REASON_BATTLE + REASON_EFFECT)) != 0
}
