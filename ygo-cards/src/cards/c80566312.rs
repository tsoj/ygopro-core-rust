//! Hymn of Light
use super::prelude::*;

pub const ID: u32 = 80566312;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,6,nil,56350972)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(6)), None, &[56350972]);
    // destroy replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, lua_fn(repval));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsType(TYPE_RITUAL)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT|REASON_BATTLE)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_type(d, c, TYPE_RITUAL)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return e:GetHandler():IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, h, None, None, None)
            && group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return s.repfilter(c,e:GetHandlerPlayer())
    let hp = effect::get_handler_player(d, a.e(0));
    repfilter(d, c, hp)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_EFFECT, None, None);
}
