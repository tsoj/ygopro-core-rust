//! Toon Barrel Dragon
use super::prelude::*;

pub const ID: u32 = 28112535;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.atklimit)
    effect::set_operation(d, e1, Some(operation(atklimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Direct attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_DIRECT_ATTACK);
    // e4:SetCondition(s.dircon)
    effect::set_condition(d, e4, Some(condition(dircon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy cards
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetCategory(CATEGORY_DESTROY+CATEGORY_COIN)
    effect::set_category(d, e5, CATEGORY_DESTROY + CATEGORY_COIN);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atklimit(e,tp,eg,ep,ev,re,r,rp)`
fn atklimit(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TOON),tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let _g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, tp, 3);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // if Duel.CountHeads(Duel.TossCoin(tp,3))<2 then return end
        let res = duel::toss_coin(d, a.tp, 3);
        if duel::count_heads(&res) < 2 {
            return;
        }
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
