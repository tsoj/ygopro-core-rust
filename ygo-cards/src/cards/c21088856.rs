//! Spy-C-Spy
use super::prelude::*;

pub const ID: u32 = 21088856;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetFieldGroup(1-tp,LOCATION_EXTRA,0)
    let g = duel::get_field_group(d, 1 - tp, LOCATION_EXTRA, 0);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:RandomSelect(tp,1):GetFirst()
    let rs = group::random_select(d, &g, tp, 1);
    let tc = group::get_first(d, &rs).nil();
    // Duel.ConfirmCards(tp,tc)
    duel::confirm_cards(d, tp, tc);
    // local atk=tc:GetAttack()
    let mut atk = card::get_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if atk>=2000 then
    if atk >= 2000 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
        if !card::is_relate_to_effect(d, c, e) || card::is_facedown(d, c) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else {
        // Duel.Recover(tp,atk,REASON_EFFECT)
        duel::recover(d, tp, atk as i64, REASON_EFFECT, false, None);
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - tp);
}
