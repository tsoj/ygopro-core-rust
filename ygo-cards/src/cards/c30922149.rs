//! Side Effects?
use super::prelude::*;

pub const ID: u32 = 30922149;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // if not Duel.IsPlayerCanDraw(p) then return end
    if !duel::is_player_can_draw(d, p, 0) {
        return;
    }
    // local ct=Duel.GetFieldGroupCount(p,LOCATION_DECK,0)
    let ct = duel::get_field_group_count(d, p, LOCATION_DECK, 0);
    // local ac=1
    let mut ac: u64 = 1;
    // if ct>1 then
    if ct > 1 {
        // Duel.Hint(HINT_SELECTMSG,p,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, p, aux::stringid(ID, 0));
        // if ct==2 then ac=Duel.AnnounceNumber(p,1,2)
        if ct == 2 {
            ac = duel::announce_number(d, p, &[1, 2]).nil().0;
        } else {
            // else ac=Duel.AnnounceNumber(p,1,2,3) end
            ac = duel::announce_number(d, p, &[1, 2, 3]).nil().0;
        }
    }
    // local dr=Duel.Draw(p,ac,REASON_EFFECT)
    let dr = duel::draw(d, p, ac as u32, REASON_EFFECT);
    // if p~=tp and dr~=0 then
    if p != a.tp && dr != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,dr*2000,REASON_EFFECT)
        duel::recover(d, a.tp, dr as i64 * 2000, REASON_EFFECT, false, None);
    }
}
