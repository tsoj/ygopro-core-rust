//! Performapal Ladyange
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 58938528;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 21949879], listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Reduce ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // Place in Pendulum Zone
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(pencon)));
    effect::set_target(d, e3, Some(cost_target(pentg)));
    effect::set_operation(d, e3, Some(operation(penop)));
    card::register_effect(d, c, e3, false);
}
/// `s.atkcostfilter(c)`: `c:IsDiscardable() and c:IsType(TYPE_PENDULUM)`
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None) && card::is_type(d, c, TYPE_PENDULUM)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.atkcostfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&atkcostfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc0,bc1=Duel.GetBattleMonster(tp)
    let (bc0, bc1) = duel::get_battle_monster(d, a.tp);
    if a.chk == Some(0) {
        // return bc0 and bc1 and bc1:IsFaceup()
        return match bc1 {
            Some(bc1) => bc0.is_some() && card::is_faceup(d, bc1),
            None => false,
        };
    }
    // e:SetLabelObject(bc1)
    effect::set_label_object(d, a.e, bc1.map_or(Value::Nil, Value::Card));
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(1-tp) then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
            // Update ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-1000)
            effect::set_value(d, e1, -1000);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.drcostfilter(c)`: `c:IsSetCard(SET_PERFORMAPAL) and c:IsMonster() and not c:IsCode(id) and c:IsDiscardable()`
fn drcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_discardable(d, c, None, None)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_discardable(d, c, None, None) && duel::is_existing_matching_card(d, Some(&drcostfilter), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.drcostfilter,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, a.tp, Some(&drcostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(g+c,REASON_DISCARD|REASON_COST)
    let mut sg = g.clone();
    group::add_card(&mut sg, &CardSet::from_card(c));
    duel::send_to_grave(d, sg, REASON_DISCARD | REASON_COST, None, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
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
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.penconfilter(c)`: `c:IsFaceup() and (c:IsSetCard(SET_ODD_EYES) or c:IsCode(21949879))`
fn penconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_ODD_EYES as u16]) || card::is_code(d, c, &[21949879]))
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&penconfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.CheckPendulumZones(tp) then
    if card::is_relate_to_effect(d, c, a.e) && duel::check_pendulum_zones(d, a.tp) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
