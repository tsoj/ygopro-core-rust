//! Rebound
use super::prelude::*;

pub const ID: u32 = 983995;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate an activated effect, and if you do, send 1 card from your opponent's hand (at random) or from their side of the field to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not Duel.IsChainNegatable(ev) then return false end
    if a.rp == a.tp || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // if re:IsHasCategory(CATEGORY_NEGATE)
    //   and Duel.GetChainInfo(ev-1,CHAININFO_TRIGGERING_EFFECT):IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    let re = a.re.nil();
    if effect::is_has_category(d, re, CATEGORY_NEGATE) {
        let te = duel::get_chain_info(d, (a.ev - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        if effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) {
            return false;
        }
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_TOHAND)
    // return ex and tg~=nil and tc+tg:FilterCount(Card.IsOnField,nil)-#tg>0
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_TOHAND) else {
        return false;
    };
    let Some(tg) = info.op_cards else {
        return false;
    };
    let on_field = group::filter_count(d, &tg, &|d, c| card::is_on_field(d, c), NO_EX) as i64;
    info.op_count as i64 + on_field - tg.len() as i64 > 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD|LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, (LOCATION_ONFIELD | LOCATION_HAND) as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.NegateEffect(ev) then return end
    if !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_HAND,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // local opt=0
    // if #g1>0 and #g2>0 then opt=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
    // elseif #g1>0 then opt=0
    // elseif #g2>0 then opt=1
    // else return end
    let opt = if !g1.is_empty() && !g2.is_empty() {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)])
    } else if !g1.is_empty() {
        0
    } else if !g2.is_empty() {
        1
    } else {
        return;
    };
    // local sg=nil
    let sg = if opt == 0 {
        // sg=g1:RandomSelect(tp,1)
        group::random_select(d, &g1, tp, 1)
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // sg=g2:Select(tp,1,1,nil)
        group::select(d, &g2, tp, 1, 1, false, NO_EX).nil()
    };
    // if #sg~=0 then Duel.SendtoGrave(sg,REASON_EFFECT) end
    if !sg.is_empty() {
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (r&REASON_DESTROY)~=0 and rp==1-tp and c:GetPreviousControler()==tp
    //   and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    (a.r & REASON_DESTROY) != 0
        && a.rp == 1 - a.tp
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEDOWN)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
