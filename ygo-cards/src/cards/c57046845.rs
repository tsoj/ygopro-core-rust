//! Gearfried the Swordmaster
use super::prelude::*;

pub const ID: u32 = 57046845;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75417459], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Destroy 1 monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_EQUIP)
    effect::set_code(d, e1, EVENT_EQUIP);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == 1 - a.tp && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsControler(1-tp) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::get_controler(d, tc) == 1 - a.tp {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
