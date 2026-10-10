//! Wonder Wand
use super::prelude::*;

pub const ID: u32 = 67775894;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_SPELLCASTER))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_SPELLCASTER))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // increase ATK
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    // send to the gy and draw
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DRAW);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_cost(d, e4, Some(cost_target(drcost)));
    effect::set_target(d, e4, Some(cost_target(drtg)));
    effect::set_operation(d, e4, Some(operation(drop)));
    card::register_effect(d, c, e4, false);
}
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then return c:IsAbleToGraveAsCost() and ec and c:GetControler()==ec:GetControler() and ec:IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        if !card::is_able_to_grave_as_cost(d, c) {
            return false;
        }
        let Some(ec) = ec else { return false };
        if card::get_controler(d, c) != card::get_controler(d, ec) {
            return false;
        }
        return card::is_able_to_grave_as_cost(d, ec);
    }
    // local g=Group.FromCards(c,ec)
    let mut cards = vec![c];
    if let Some(ec) = ec {
        cards.push(ec);
    }
    let g = group::create_group(&cards);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
fn drop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
