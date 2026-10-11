//! E.M.R.
use super::prelude::*;

pub const ID: u32 = 17427333;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute and destroy based on og ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_ATTACK,TIMINGS_CHECK_MONSTER_E|TIMING_ATTACK)
    effect::set_hint_timing(d, e1, TIMING_ATTACK, Some(TIMINGS_CHECK_MONSTER_E | TIMING_ATTACK));
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desact)
    effect::set_operation(d, e1, Some(operation(desact)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,tc)`
fn costfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_MACHINE) and c:GetTextAttack()>=1000
    //   and Duel.IsExistingTarget(nil,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,Group.FromCards(tc,c))
    if !(card::is_monster(d, c) && card::is_race(d, c, RACE_MACHINE as u64) && card::get_text_attack(d, c) >= 1000) {
        return false;
    }
    let mut ex = CardSet::from_card(tc);
    group::add_card(&mut ex, &CardSet::from_card(c));
    duel::is_existing_target(d, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,c)
        return duel::check_release_group_cost(d, tp, Some(&|d, x| costfilter(d, x, c)), 1, 1, false, None, NO_EX);
    }
    // local tg=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,c)
    let tg = duel::select_release_group_cost(d, tp, Some(&|d, x| costfilter(d, x, c)), 1, 1, false, None, NO_EX);
    // local mtgc=tg:GetFirst():GetTextAttack()//1000
    let first = group::get_first(d, &tg).nil();
    let mtgc = card::get_text_attack(d, first).div_euclid(1000);
    // Duel.Release(tg,REASON_COST)
    duel::release(d, tg, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,mtgc,c)
    let g = duel::select_target(d, tp, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, mtgc as u16, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desact(e,tp,eg,ep,ev,re,r,rp)`
fn desact(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
