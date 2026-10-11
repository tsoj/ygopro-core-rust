//! Onikuji
//! 鬼くじ
use super::prelude::*;

pub const ID: u32 = 73820802;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_STANDBY_PHASE,0)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, Some(0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // instant(chain)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CARDTYPE as u64);
    // local op=Duel.SelectOption(1-tp,70,71,72)
    let opt = duel::select_option(d, 1 - a.tp, true, &[70, 71, 72]);
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, a.tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if (op==0 and tc:IsMonster()) or (op==1 and tc:IsSpell()) or (op==2 and tc:IsTrap()) then
    if (opt == 0 && card::is_monster(d, tc.nil()))
        || (opt == 1 && card::is_spell(d, tc.nil()))
        || (opt == 2 && card::is_trap(d, tc.nil()))
    {
        // Duel.Draw(1-tp,1,REASON_EFFECT)
        duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    } else {
        // local hg=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0)
        let hg = duel::get_field_group(d, 1 - a.tp, LOCATION_HAND, 0);
        // if #hg>0 then
        if !hg.is_empty() {
            // local sg=hg:RandomSelect(1-tp,1)
            let sg = group::random_select(d, &hg, 1 - a.tp, 1);
            // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
        }
    }
    // Duel.MoveSequence(tc,1)
    duel::move_sequence(d, tc.nil(), 1, None);
}
