//! WAKE CUP! Mocha
//! ＷＡＫＥ ＣＵＰ！ モカ
use super::prelude::*;

pub const ID: u32 = 91818544;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 monster on the field gain 1000 ATK, but send it to the GY during the End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand, and if you do, change that monster to Attack Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change this card to face-down Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g.clone()), 1, a.tp, 1000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, a.tp, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc:IsRelateToEffect(e) then
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // if tc:IsFaceup() then
        if card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(1000)
            effect::set_value(d, e1, 1000);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,
        //     function(dg) Duel.SendtoGrave(dg,REASON_EFFECT) end,
        //     nil,0,1,aux.Stringid(id,3),aux.Stringid(id,4))
        aux::delayed_operation(
            d,
            CardSet::from_card(tc),
            PHASE_END,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(send_to_grave_oper)),
            None,
            Some(0),
            Some(1),
            Some(aux::stringid(ID, 3)),
            Some(aux::stringid(ID, 4)),
        );
    }
}
/// `function(dg) Duel.SendtoGrave(dg,REASON_EFFECT) end`
fn send_to_grave_oper(d: &mut Duel, dg: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_grave(d, dg.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsDefensePos() and c:IsCanChangePosition()
    card::is_facedown(d, c) && card::is_defense_pos(d, c) && card::is_can_change_position(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.posfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && posfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,0)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(0), None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, cc| posfilter(d, cc)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, cc| posfilter(d, cc)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEUP_ATTACK)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, a.tp, POS_FACEUP_ATTACK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
    {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangePosition(tc,POS_FACEUP_ATTACK)
            duel::change_position(d, CardSet::from_card(tc), POS_FACEUP_ATTACK as u8, None, None, None, false);
        }
    }
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, CardSet::from_card(c), POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
