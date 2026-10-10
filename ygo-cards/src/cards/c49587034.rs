//! Lightforce Sword
use super::prelude::*;

pub const ID: u32 = 49587034;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil,tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)),
            tp,
            0,
            LOCATION_HAND,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, 1 - a.tp, LOCATION_HAND, 0);
    // local rs=g:RandomSelect(1-tp,1)
    let rs = group::random_select(d, &g, 1 - a.tp, 1);
    // local card=rs:GetFirst()
    let sel = group::get_first(d, &rs);
    // if card==nil then return end
    let Some(sel) = sel else {
        return;
    };
    // if Duel.Remove(card,POS_FACEDOWN,REASON_EFFECT)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if duel::remove(d, sel, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetRange(LOCATION_REMOVED)
        effect::set_range(d, e1, LOCATION_REMOVED);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,5)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 5);
        // e1:SetCondition(s.thcon)
        effect::set_condition(d, e1, Some(condition(thcon)));
        // e1:SetOperation(s.thop)
        effect::set_operation(d, e1, Some(operation(thop)));
        // e1:SetLabel(0)
        effect::set_label(d, e1, &[0]);
        // card:RegisterEffect(e1)
        card::register_effect(d, sel, e1, false);
        // card:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, sel, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // local descnum=tp==c:GetOwner() and 0 or 1
        let descnum = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetDescription(aux.Stringid(id,descnum))
        effect::set_description(d, e3, aux::stringid(ID, descnum));
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(
            d,
            e3,
            EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE,
            0,
        );
        // e3:SetCode(1082946)
        effect::set_code(d, e3, 1082946);
        // e3:SetLabelObject(e1)
        effect::set_label_object(d, e3, Value::Effect(e1));
        // e3:SetOwnerPlayer(tp)
        effect::set_owner_player(d, e3, a.tp);
        // e3:SetOperation(s.reset)
        effect::set_operation(d, e3, Some(operation(reset)));
        // e3:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_OPPO_TURN,4)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_STANDBY | RESET_OPPO_TURN, 4);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`: the effect e3 of the card; the effect `re` is passed to `s.thop` as `e`'s partner.
fn reset(d: &mut Duel, a: &EffArgs) {
    // if not e:GetLabelObject() then e:Reset() return end
    let label_object = effect::get_label_object(d, a.e);
    if !label_object.truthy() {
        effect::reset(d, a.e);
        return;
    }
    // s.thop(e:GetLabelObject(),tp,eg,ep,ev,e,r,rp)
    let le = label_object.as_effect().nil();
    thop_with(d, le, Some(a.e));
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():GetFlagEffect(id)==0
    duel::is_turn_player(d, a.tp) && card::get_flag_effect(d, effect::get_handler(d, a.e).nil(), ID) == 0
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    thop_with(d, a.e, a.re);
}
fn thop_with(d: &mut Duel, e: EffectId, re: Option<EffectId>) {
    // local ct=e:GetLabel()+1
    let ct = effect::get_label(d, e) + 1;
    // e:GetOwner():SetTurnCounter(ct)
    let owner = effect::get_owner(d, e).nil();
    card::set_turn_counter(d, owner, ct as u16);
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct]);
    // if ct==4 then
    if ct == 4 {
        // Duel.SendtoHand(e:GetHandler(),nil,REASON_EFFECT)
        let handler = effect::get_handler(d, e).nil();
        duel::send_to_hand(d, handler, None, REASON_EFFECT, None);
        // if re then re:Reset() end
        if let Some(re) = re {
            effect::reset(d, re);
        }
    }
}
