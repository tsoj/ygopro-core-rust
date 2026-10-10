//! Panther Warrior
use super::prelude::*;

pub const ID: u32 = 42035044;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack cost
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_ATTACK_COST);
    effect::set_cost(d, e1, Some(lua_fn(atcost)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
}
/// `s.atcost(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    // return Duel.CheckReleaseGroup(tp,nil,1,e:GetHandler())
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::check_release_group(d, a.p(2), None, 1, duel::ReleaseArgs::default(), h.into(), false)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsAttackCostPaid()~=2 and e:GetHandler():IsLocation(LOCATION_MZONE) then
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_MZONE) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
        // local tc=Duel.GetReleaseGroup(tp):Filter(aux.TRUE,e:GetHandler()):SelectUnselect(Group.CreateGroup(),tp,Duel.IsAttackCostPaid()==0, Duel.IsAttackCostPaid()==0)
        let rg = duel::get_release_group(d, a.tp, false, false, None);
        let rg = group::filter(d, &rg, &|_d, _c| true, effect::get_handler(d, a.e).nil().into());
        let finishable = duel::is_attack_cost_paid(d) == 0;
        let cancelable = duel::is_attack_cost_paid(d) == 0;
        let g2 = CardSet::new();
        let tc = group::select_unselect(d, &rg, Some(&g2), a.tp, finishable, cancelable, None, None);
        // if tc then
        if let Some(tc) = tc {
            // Duel.Release(tc,REASON_COST)
            duel::release(d, CardSet::from_card(tc), REASON_COST, None);
            // Duel.AttackCostPaid()
            duel::attack_cost_paid(d, None);
        } else {
            // Duel.AttackCostPaid(2)
            duel::attack_cost_paid(d, Some(2));
        }
    }
}
