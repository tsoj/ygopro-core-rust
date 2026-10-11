//! Hydra Viper
//! トリプル・ヴァイパー
use super::prelude::*;

pub const ID: u32 = 37869028;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // multiattack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(2)
    effect::set_value(d, e1, 2);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attack cost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_COST)
    effect::set_code(d, e2, EFFECT_ATTACK_COST);
    // e2:SetCost(s.atcost)
    effect::set_cost(d, e2, Some(lua_fn(atcost)));
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atcost(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    // return Duel.CheckReleaseGroupCost(tp,Card.IsRace,1,false,nil,e:GetHandler(),RACE_AQUA)
    let tp = a.p(2);
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_AQUA)), 1, 1, false, None, h.into())
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if Duel.IsAttackCostPaid()~=2 and e:GetHandler():IsLocation(LOCATION_MZONE) then
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, h, LOCATION_MZONE) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        // local tc=Duel.GetReleaseGroup(tp):Filter(Card.IsRace,e:GetHandler(),RACE_AQUA):SelectUnselect(Group.CreateGroup(),tp,Duel.IsAttackCostPaid()==0, Duel.IsAttackCostPaid()==0)
        let rg = duel::get_release_group(d, tp, false, false, None);
        let mg = group::filter(d, &rg, &|d, c| card::is_race(d, c, RACE_AQUA), h.into());
        let finishable = duel::is_attack_cost_paid(d) == 0;
        let tc = group::select_unselect(d, &mg, Some(&CardSet::new()), tp, finishable, finishable, None, None);
        // if tc then
        if let Some(tc) = tc {
            // Duel.Release(tc,REASON_COST)
            duel::release(d, tc, REASON_COST, None);
            // Duel.AttackCostPaid()
            duel::attack_cost_paid(d, None);
        } else {
            // Duel.AttackCostPaid(2)
            duel::attack_cost_paid(d, Some(2));
        }
    }
}
