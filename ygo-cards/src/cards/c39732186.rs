//! Prohibit Snake
use super::prelude::*;

pub const ID: u32 = 39732186;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // return to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.rthcon)
    effect::set_condition(d, e1, Some(condition(rthcon)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.rthtg)
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    // e1:SetOperation(s.rthop)
    effect::set_operation(d, e1, Some(operation(rthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // recover
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rthcon(e,tp,eg,ep,ev,re,r,rp)
fn rthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if not bc then return false end
    let Some(bc) = bc else {
        return false;
    };
    // if tc:IsControler(1-tp) then tc,bc=bc,tc end
    let (tc, bc) = if card::is_controler(d, tc, 1 - a.tp) { (bc, tc) } else { (tc, bc) };
    // if tc:IsFaceup() and tc:IsRace(RACE_CYBERSE) and tc:IsLinkMonster() and bc:IsAbleToHand() then
    if card::is_faceup(d, tc) && card::is_race(d, tc, RACE_CYBERSE) && card::is_link_monster(d, tc) && card::is_able_to_hand(d, bc, None) {
        // e:SetLabelObject(bc)
        effect::set_label_object(d, a.e, Value::Card(bc));
        // return true
        true
    } else {
        // else return false end
        false
    }
}
// function s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.rthop(e,tp,eg,ep,ev,re,r,rp)
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // Duel.SendtoHand(bc,nil,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(bc), None, REASON_EFFECT, None);
        }
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // return #eg==1
    //   and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
    //   and bc:IsRelateToBattle() and bc:IsControler(tp) and bc:IsRace(RACE_CYBERSE)
    eg.len() == 1
        && card::is_location(d, tc, LOCATION_GRAVE)
        && card::is_reason(d, tc, REASON_BATTLE)
        && card::is_relate_to_battle(d, bc.nil())
        && card::is_controler(d, bc.nil(), a.tp)
        && card::is_race(d, bc.nil(), RACE_CYBERSE)
}
// function s.thcfilter(c,tp)
fn thcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //   and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,c)
    card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, c.into())
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_CYBERSE) and c:IsAbleToHand()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_CYBERSE) && card::is_able_to_hand(d, c, None)
}
// function s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thcfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thcfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thcfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sg=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let sg = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), n, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
