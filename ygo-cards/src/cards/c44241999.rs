//! Goblin Biker Troika Griare
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 44241999;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,3,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Your "Goblin" monsters can attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_GOBLIN))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GOBLIN as u16]))));
    // e1:SetCondition(function() return Duel.GetOverlayCount(0,1,1)>=3 end)
    effect::set_condition(d, e1, Some(condition(direct_cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the position of 1 monster that is summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetCondition(s.poscond)
    effect::set_condition(d, e2, Some(condition(poscond)));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate the attack of your opponent's monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(function(_,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e4, Some(condition(attack_cond)));
    // e4:SetTarget(s.negatktg)
    effect::set_target(d, e4, Some(cost_target(negatktg)));
    // e4:SetOperation(s.negatkop)
    effect::set_operation(d, e4, Some(operation(negatkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function() return Duel.GetOverlayCount(0,1,1)>=3 end`
fn direct_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_overlay_count(d, 0, 1, 1, None) >= 3
}
/// `function(_,tp) return Duel.GetAttacker():IsControler(1-tp) end`
fn attack_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
}
/// `s.cfilter(c,e)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanChangePosition() and c:IsLocation(LOCATION_MZONE) and c:IsCanBeEffectTarget(e)
    card::is_can_change_position(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.poscond(e,tp,eg,ep,ev,re,r,rp)`
fn poscond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,e) and not eg:IsContains(e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e), 1, NO_EX) && !eg.iter().any(|x| x == h)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.cfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return eg.iter().any(|x| x == chkc) && cfilter(d, chkc, a.e);
    }
    // if chk==0 then return eg:IsExists(s.cfilter,1,nil,e) and Duel.CheckRemoveOverlayCard(tp,1,1,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e), 1, NO_EX) && duel::check_remove_overlay_card(d, tp, 1, 1, 1, REASON_EFFECT, None);
    }
    // local tc=nil
    let tc: CardId;
    // if #eg==1 then
    if eg.len() == 1 {
        // tc=eg:GetFirst()
        tc = group::get_first(d, &eg).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
    // else
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        // tc=eg:FilterSelect(tp,s.cfilter,1,1,nil,e)
        let sel = group::filter_select(d, &eg, tp, &|d, c| cfilter(d, c, a.e), 1, 1, false, NO_EX).nil();
        tc = group::get_first(d, &sel).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.RemoveOverlayCard(tp,1,1,1,1,REASON_EFFECT)>0 and tc:IsRelateToEffect(e) then
    if duel::remove_overlay_card(d, tp, 1, 1, 1, 1, REASON_EFFECT, None) > 0 && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(
            d,
            tc.nil(),
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEDOWN_DEFENSE as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        );
    }
}
/// `s.negatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negatktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckRemoveOverlayCard(tp,1,1,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 1, 1, REASON_EFFECT, None);
    }
    true
}
/// `s.negatkop(e,tp,eg,ep,ev,re,r,rp)`
fn negatkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.RemoveOverlayCard(tp,1,1,1,1,REASON_EFFECT)>0 then
    if duel::remove_overlay_card(d, a.tp, 1, 1, 1, 1, REASON_EFFECT, None) > 0 {
        // Duel.NegateAttack()
        duel::negate_attack(d);
    }
}
