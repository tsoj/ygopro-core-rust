//! Call of the Earthbound
use super::prelude::*;

pub const ID: u32 = 65743242;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ag,da=eg:GetFirst():GetAttackableTarget()
        let eg = a.eg.clone().nil();
        let tc = group::get_first(d, &eg).nil();
        let (ag, da) = card::get_attackable_target(d, tc);
        // local at=Duel.GetAttackTarget()
        let at = duel::get_attack_target(d);
        // return ag:IsExists(aux.TRUE,1,at) or (at~=nil and da)
        return group::is_exists(d, &ag, &|_, _| true, 1, at.into()) || (at.is_some() && da);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ag,da=eg:GetFirst():GetAttackableTarget()
    let eg = a.eg.clone().nil();
    let tc0 = group::get_first(d, &eg).nil();
    let (ag, da) = card::get_attackable_target(d, tc0);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if da and at~=nil then
    if da && at.is_some() {
        // Duel.Hint(HINT_SELECTMSG,tp,31)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, 31);
        // if ag:IsExists(aux.TRUE,1,at) then sel=Duel.SelectOption(tp,1213,1214) else sel=Duel.SelectOption(tp,1213) end
        let sel = if group::is_exists(d, &ag, &|_, _| true, 1, at.into()) {
            duel::select_option(d, a.tp, true, &[1213, 1214])
        } else {
            duel::select_option(d, a.tp, true, &[1213])
        };
        // if sel==0 then Duel.ChangeAttackTarget(nil) return end
        if sel == 0 {
            duel::change_attack_target(d, None, false);
            return;
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACKTARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACKTARGET as u64);
    // local g=ag:Select(tp,1,1,at)
    let g = group::select(d, &ag, a.tp, 1, 1, false, at.into()).nil();
    // local tc=g:GetFirst()
    // if tc then Duel.ChangeAttackTarget(tc) end
    if let Some(tc) = group::get_first(d, &g) {
        duel::change_attack_target(d, Some(tc), false);
    }
}
