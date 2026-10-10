//! By Order of the Emperor
use super::prelude::*;

pub const ID: u32 = 35011819;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate1)
    effect::set_operation(d, e1, Some(operation(activate1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // instant
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate2)
    effect::set_operation(d, e2, Some(operation(activate2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local ct=Duel.GetCurrentChain()
    let ct = duel::get_current_chain(d, false);
    // if ct==1 then return end
    if ct == 1 {
        return true;
    }
    // local te=Duel.GetChainInfo(ct-1,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, (ct - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // if te:GetCode()==EVENT_SUMMON_SUCCESS and te:IsMonsterEffect() and Duel.IsChainNegatable(ct-1)
    //     and Duel.IsPlayerCanDraw(tc:GetControler(),1) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if effect::get_code(d, te) == EVENT_SUMMON_SUCCESS
        && effect::is_monster_effect(d, te)
        && duel::is_chain_negatable(d, (ct - 1) as u8)
        && duel::is_player_can_draw(d, card::get_controler(d, tc), 1)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // Duel.SetOperationInfo(0,CATEGORY_NEGATE,tc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_NEGATE, Some(CardSet::from_card(tc)), 1, 0, 0);
        // Duel.SetTargetPlayer(tc:GetControler())
        let p = card::get_controler(d, tc);
        duel::set_target_player(d, p);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tc:GetControler(),1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, p, 1);
        // e:GetHandler():RegisterFlagEffect(0,RESET_CHAIN,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, 0, RESET_CHAIN, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    }
    true
}
/// `s.activate1(e,tp,eg,ep,ev,re,r,rp)`
fn activate1(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()~=1 then return end
    if effect::get_label(d, a.e) != 1 {
        return;
    }
    // local ct=Duel.GetCurrentChain()
    let ct = duel::get_current_chain(d, false);
    // Duel.NegateActivation(ct-1)
    duel::negate_activation(d, (ct - 1) as u8);
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and re:GetCode()==EVENT_SUMMON_SUCCESS and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && effect::get_code(d, re) == EVENT_SUMMON_SUCCESS && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(rp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.rp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetTargetPlayer(rp)
    duel::set_target_player(d, a.rp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,rp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.rp, 1);
    true
}
/// `s.activate2(e,tp,eg,ep,ev,re,r,rp)`
fn activate2(d: &mut Duel, a: &EffArgs) {
    // if not Duel.NegateActivation(ev) then return end
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
