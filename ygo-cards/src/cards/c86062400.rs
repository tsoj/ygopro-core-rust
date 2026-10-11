//! Xyz Avenger
use super::prelude::*;

pub const ID: u32 = 86062400;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetCondition(s.econ)
    effect::set_condition(d, e1, Some(condition(econ)));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    //
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.econ(e)
fn econ(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsActiveType(TYPE_XYZ)
    let te = a.e(1);
    effect::is_active_type(d, te, TYPE_XYZ)
}
// function s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE)
    //     and bc:IsFaceup() and bc:IsRelateToBattle() and bc:IsType(TYPE_XYZ)
    if !(card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE)) {
        return false;
    }
    let bc = card::get_battle_target(d, c).nil();
    card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) && card::is_type(d, bc, TYPE_XYZ)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rk=e:GetHandler():GetBattleTarget():GetRank()
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    let rk = card::get_rank(d, bt);
    // if rk<5 then
    if rk < 5 {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_EXTRA as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,rk,1-tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, rk as u8, 1 - a.tp, LOCATION_EXTRA as i32);
    }
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // if bc:IsFaceup() and bc:IsRelateToBattle() then
    if card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) {
        // local rk=bc:GetRank()
        let rk = card::get_rank(d, bc);
        // local g=nil
        // local tg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,nil)
        let tg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_EXTRA, NO_EX);
        // if #tg==0 then return end
        if tg.is_empty() {
            return;
        }
        let g;
        // if rk<4 then
        if rk < 4 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
            // g=tg:Select(1-tp,1,1,nil)
            g = group::select(d, &tg, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // elseif rk==4 then
        } else if rk == 4 {
            // Duel.ConfirmCards(tp,tg)
            duel::confirm_cards(d, a.tp, &tg);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // g=tg:Select(tp,1,1,nil)
            g = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
        // else
        } else {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
            // g=tg:Select(1-tp,rk,rk,nil)
            g = group::select(d, &tg, 1 - a.tp, rk as u16, rk as u16, false, NO_EX).nil();
        }
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
        // if rk==4 then
        if rk == 4 {
            // Duel.ShuffleExtra(1-tp)
            duel::shuffle_extra(d, 1 - a.tp);
        }
    }
}
