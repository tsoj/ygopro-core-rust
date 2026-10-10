//! Wannabee!
use super::prelude::*;

pub const ID: u32 = 3248469;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During the End Phase, excavate and Set 1 Trap card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_MZONE|LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.excvttg)
    effect::set_target(d, e1, Some(cost_target(excvttg)));
    // e1:SetOperation(s.excvtop)
    effect::set_operation(d, e1, Some(operation(excvtop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.excvttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn excvttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=5-Duel.GetMatchingGroupCount(nil,tp,0,LOCATION_STZONE,nil)
    let ct = 5 - duel::get_matching_group_count(d, None, a.tp, 0, LOCATION_STZONE, NO_EX) as i32;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=ct end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= ct;
    }
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSSetable()
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.excvtop(e,tp,eg,ep,ev,re,r,rp)`
fn excvtop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local excct=5-Duel.GetMatchingGroupCount(nil,tp,0,LOCATION_STZONE,nil)
    let excct = 5 - duel::get_matching_group_count(d, None, tp, 0, LOCATION_STZONE, NO_EX) as i32;
    // Duel.ConfirmDecktop(tp,excct)
    duel::confirm_decktop(d, tp, excct as u32);
    // local g=Duel.GetDecktopGroup(tp,excct)
    let g = duel::get_decktop_group(d, tp, excct as u32);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=0
    let mut ct = 0;
    // if g:IsExists(s.setfilter,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if group::is_exists(d, &g, &|d, c| setfilter(d, c), 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local tc=g:FilterSelect(tp,s.setfilter,1,1,nil):GetFirst()
        let sel = group::filter_select(d, &g, tp, &|d, c| setfilter(d, c), 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if tc and Duel.SSet(tp,tc) then
        if let Some(tc) = tc {
            if duel::s_set(d, tp, tc, None, true) != 0 {
                // local fid=tc:GetFieldID()
                let fid = card::get_field_id(d, tc);
                // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2,fid)
                card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 2, fid as i64, 0);
                // --Send it to the GY during the next End Phase
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
                effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
                // e1:SetCode(EVENT_PHASE+PHASE_END)
                effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // e1:SetCondition(s.tgcon)
                effect::set_condition(d, e1, Some(condition(tgcon)));
                // e1:SetOperation(s.tgop)
                effect::set_operation(d, e1, Some(operation(tgop)));
                // e1:SetLabel(Duel.GetTurnCount(),fid)
                let turn = duel::get_turn_count(d, None);
                effect::set_label(d, e1, &[turn as i64, fid as i64]);
                // e1:SetLabelObject(tc)
                effect::set_label_object(d, e1, Value::Card(tc));
                // e1:SetReset(RESET_PHASE|PHASE_END,2)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, tp);
            }
        }
        // ct=1
        ct = 1;
    }
    // local ac=#g-ct
    let ac = g.len() as i32 - ct;
    // if ac>0 then
    if ac > 0 {
        // Duel.MoveToDeckBottom(ac,tp)
        duel::move_decktop_to_bottom(d, ac as u32, tp);
        // Duel.SortDeckbottom(tp,tp,ac)
        duel::sort_deckbottom(d, tp, tp, ac as u16);
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local turn,fid=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let turn = labels.first().copied();
    let fid = labels.get(1).copied();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return Duel.GetTurnCount()~=turn and tc:GetFlagEffectLabel(id)==fid
    Some(duel::get_turn_count(d, None) as i64) != turn && card::get_flag_effect_label(d, tc, ID).first().copied() == fid
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetLabelObject(),REASON_EFFECT)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    // e:Reset()
    effect::reset(d, a.e);
}
