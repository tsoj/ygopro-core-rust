//! Ahrima, the Wicked Warden
use super::prelude::*;

pub const ID: u32 = 86377375;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[59160188], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.drcost)
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter1(c)`
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(59160188) and c:IsAbleToHand()
    card::is_code(d, c, &[59160188]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter1), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetFirstMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,nil)
    let tg = duel::get_first_matching_card(d, Some(&thfilter1), a.tp, LOCATION_DECK, 0, NO_EX);
    // if tg then
    if let Some(tg) = tg {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - a.tp, tg);
    }
}
/// `s.thfilter2(c)`
fn thfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsDefenseAbove(2000) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_defense_above(d, c, 2000) && card::is_able_to_hand(d, c, None)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return ((Duel.CheckReleaseGroupCost(tp,Card.IsAttribute,1,false,nil,nil,ATTRIBUTE_DARK) and Duel.IsPlayerCanDraw(tp,1))
    //     or (Duel.CheckReleaseGroupCost(tp,Card.IsAttribute,1,false,nil,c,ATTRIBUTE_DARK) and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil))) end
    if a.chk == Some(0) {
        return (duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 1, 1, false, None, NO_EX)
            && duel::is_player_can_draw(d, a.tp, 1))
            || (duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 1, 1, false, None, c.into())
                && duel::is_existing_matching_card(d, Some(&thfilter2), a.tp, LOCATION_DECK, 0, 1, NO_EX));
    }
    // local sg=nil
    // if Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil) and not Duel.IsPlayerCanDraw(tp,1) then
    let sg = if duel::is_existing_matching_card(d, Some(&thfilter2), a.tp, LOCATION_DECK, 0, 1, NO_EX) && !duel::is_player_can_draw(d, a.tp, 1) {
        // sg=Duel.SelectReleaseGroupCost(tp,Card.IsAttribute,1,1,false,nil,c,ATTRIBUTE_DARK)
        duel::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 1, 1, false, None, c.into())
    } else {
        // sg=Duel.SelectReleaseGroupCost(tp,Card.IsAttribute,1,1,false,nil,nil,ATTRIBUTE_DARK)
        duel::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 1, 1, false, None, NO_EX)
    };
    // e:SetLabelObject(sg:GetFirst())
    let first = group::get_first(d, &sg);
    effect::set_label_object(d, a.e, Value::card(first));
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if e:GetLabelObject()~=e:GetHandler() then
    let h = effect::get_handler(d, a.e).nil();
    if effect::get_label_object(d, a.e).as_card() != Some(h) {
        // e:SetCategory(CATEGORY_DRAW+CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_DRAW + CATEGORY_TOHAND + CATEGORY_SEARCH);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject()~=e:GetHandler() and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil)
    //     and (not Duel.IsPlayerCanDraw(tp,1) or Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
    let h = effect::get_handler(d, a.e).nil();
    let is_other = effect::get_label_object(d, a.e).as_card() != Some(h);
    if is_other
        && duel::is_existing_matching_card(d, Some(&thfilter2), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && (!duel::is_player_can_draw(d, a.tp, 1) || duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter2), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if group::get_count(&g) > 0 {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    } else {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    }
}
