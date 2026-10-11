//! Runick Golden Droplet
use super::prelude::*;

pub const ID: u32 = 20618850;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RUNICK],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateMysteruneQPEffect(c,id,CATEGORY_DRAW,s.drtg,s.drop,4,EFFECT_FLAG_PLAYER_TARGET)
    let (_, e1) = aux::mysterune::create_mysterune_qp_effect(
        d,
        c,
        ID,
        Some(CATEGORY_DRAW),
        Some(cost_target(drtg)),
        Some(condition(drop_op)),
        Some(aux::RmCount::N(4)),
        Some(EFFECT_FLAG_PLAYER_TARGET),
        None,
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(1-tp,1) and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>4 end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - tp, 1) && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) > 4;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`: its return value is read by the Mysterune helper
fn drop_op(d: &mut Duel, _a: &EffArgs) -> bool {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // return Duel.Draw(p,1,REASON_EFFECT)>0
    duel::draw(d, p, 1, REASON_EFFECT) > 0
}
