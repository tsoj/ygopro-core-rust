//! Chicken Game
use super::prelude::*;

pub const ID: u32 = 67616300;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // The player with the lowest LP takes no damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_condition(d, e2, Some(lua_fn(damcon1)));
    effect::set_value(d, e2, 0);
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_NO_EFFECT_DAMAGE);
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    effect::set_target_range(d, e4, 0, 1);
    effect::set_condition(d, e4, Some(lua_fn(damcon2)));
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_NO_EFFECT_DAMAGE);
    card::register_effect(d, c, e5, false);
    // Activate 1 of these effects
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 0));
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e6, EFFECT_FLAG_BOTH_SIDE, 0);
    effect::set_range(d, e6, LOCATION_FZONE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_cost(d, e6, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e6, Some(cost_target(efftg)));
    effect::set_operation(d, e6, Some(operation(effop)));
    card::register_effect(d, c, e6, false);
}
fn damcon1(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLP(tp)<Duel.GetLP(1-tp)
    duel::get_lp(d, tp) < duel::get_lp(d, 1 - tp)
}
fn damcon2(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLP(1-tp)<Duel.GetLP(tp)
    duel::get_lp(d, 1 - tp) < duel::get_lp(d, tp)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.IsPlayerCanDraw(tp,1)
    let b1 = duel::is_player_can_draw(d, tp, 1);
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {true,aux.Stringid(id,2)}, {true,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (true, aux::stringid(ID, 2)), (true, aux::stringid(ID, 3))],
    )
    .nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // e:SetProperty(EFFECT_FLAG_BOTH_SIDE+EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_BOTH_SIDE + EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_BOTH_SIDE)
        effect::set_property(d, a.e, EFFECT_FLAG_BOTH_SIDE, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, tp, 0);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_RECOVER)
        effect::set_category(d, a.e, CATEGORY_RECOVER);
        // e:SetProperty(EFFECT_FLAG_BOTH_SIDE+EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_BOTH_SIDE + EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - tp);
        // Duel.SetTargetParam(1000)
        duel::set_target_param(d, 1000);
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,1-tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - tp, 1000);
    }
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    } else if op == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.Destroy(c,REASON_EFFECT)
            duel::destroy(d, c, REASON_EFFECT, None, None);
        }
    } else if op == 3 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Recover(p,d,REASON_EFFECT)
        duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    }
}
