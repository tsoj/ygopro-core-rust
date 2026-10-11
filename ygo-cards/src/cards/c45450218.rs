//! A・O・J リーサル・ウェポン
use super::prelude::*;

pub const ID: u32 = 45450218;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetOperation(s.checkop)
    effect::set_operation(d, e1, Some(operation(checkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc==c then tc=Duel.GetAttackTarget() end
    if tc == Some(c) {
        tc = duel::get_attack_target(d);
    }
    // if tc and tc:IsAttribute(ATTRIBUTE_LIGHT) then e:SetLabel(1)
    if let Some(tc) = tc {
        if card::is_attribute(d, tc, ATTRIBUTE_LIGHT) {
            effect::set_label(d, a.e, &[1]);
            return;
        }
    }
    // else e:SetLabel(0) end
    effect::set_label(d, a.e, &[0]);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=eg:GetFirst()
    let eg = a.eg.clone().nil();
    // return #eg==1 and dg:IsLocation(LOCATION_GRAVE) and dg:IsReason(REASON_BATTLE)
    //     and dg:IsAttribute(ATTRIBUTE_LIGHT) and dg:GetReasonCard()==e:GetHandler() and e:GetLabelObject():GetLabel()==1
    if eg.len() != 1 {
        return false;
    }
    let dg = group::get_first(d, &eg).nil();
    let h = effect::get_handler(d, a.e).nil();
    let label_e = effect::get_label_object(d, a.e).as_effect().nil();
    card::is_location(d, dg, LOCATION_GRAVE)
        && card::is_reason(d, dg, REASON_BATTLE)
        && card::is_attribute(d, dg, ATTRIBUTE_LIGHT)
        && card::get_reason_card(d, dg) == Some(h)
        && effect::get_label(d, label_e) == 1
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.Draw(tp,1,REASON_EFFECT)
    let ct = duel::draw(d, tp, 1, REASON_EFFECT);
    // if ct==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if ct == 0 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let tc = group::get_first(d, &duel::get_operated_group(d)).nil();
    // if tc:GetLevel()<=4 and tc:IsAttribute(ATTRIBUTE_DARK) and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::get_level(d, tc) <= 4
        && card::is_attribute(d, tc, ATTRIBUTE_DARK)
        && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
