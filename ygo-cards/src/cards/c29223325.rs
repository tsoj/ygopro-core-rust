//! Artifact Ignition
use super::prelude::*;

pub const ID: u32 = 29223325;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTIFACT as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMING_EQUIP));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // skip
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARTIFACT) and c:IsMonster() and c:IsSSetable(true)
    card::is_set_card(d, c, &[SET_ARTIFACT as u16]) && card::is_monster(d, c) && card::is_s_setable(d, c, true, None)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
fn desfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:GetSequence()<5
    card::is_spell_trap(d, c) && card::get_sequence(d, c) < 5
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()  (read by the closures below)
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        // if not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) then return false end
        if !duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 1, NO_EX) {
            return false;
        }
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
        if card::is_location(d, c, LOCATION_HAND) {
            ft -= 1;
        }
        if ft < 0 {
            return false;
        } else if ft > 0 {
            return duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
        } else {
            return duel::is_existing_target(d, Some(&desfilter2), tp, LOCATION_ONFIELD, 0, 1, c.into());
        }
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local g=nil
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = if ft > 0 {
        // g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
        duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into())
    } else {
        // g=Duel.SelectTarget(tp,s.desfilter2,tp,LOCATION_ONFIELD,0,1,1,e:GetHandler())
        duel::select_target(d, tp, Some(&desfilter2), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into())
    };
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g:GetFirst())
            let first = group::get_first(d, &g).nil();
            duel::s_set(d, tp, first, None, true);
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and e:GetHandler():IsPreviousControler(tp)
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SKIP_BP);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    // if Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase() then
    if duel::is_turn_player(d, 1 - tp) && duel::is_battle_phase(d, None) {
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, e1, &[turn as i64]);
        // e1:SetCondition(s.skipcon)
        effect::set_condition(d, e1, Some(lua_fn(skipcon)));
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_OPPO_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_OPPO_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_OPPO_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_OPPO_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e) return Duel.GetTurnCount()~=e:GetLabel() end`
fn skipcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, e)
}
