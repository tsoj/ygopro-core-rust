//! Marincess Current
use super::prelude::*;

pub const ID: u32 = 84430165;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MARINCESS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inflict Damage 400x the Link Rating of monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MARINCESS) and c:IsLinkMonster() and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c) && card::is_controler(d, c, tp)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return s.cfilter(eg:GetFirst(),tp)
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    cfilter(d, first, a.tp)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:GetFirst():IsRelateToBattle() end
    if a.chk == Some(0) {
        let first = group::get_first(d, &eg).nil();
        return card::is_relate_to_battle(d, first);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,eg:GetFirst():GetLink()*400)
    let first = group::get_first(d, &eg).nil();
    let amount = card::get_link(d, first) as i32 * 400;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, amount);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ac=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let ac = group::get_first(d, &eg).nil();
    // Duel.Damage(p,ac:GetLink()*400,REASON_EFFECT)
    let amount = card::get_link(d, ac) as i64 * 400;
    duel::damage(d, p, amount, REASON_EFFECT, false, None);
    // if Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil,tp):IsExists(Card.IsLinkAbove,1,nil,2)
    let tp = a.tp;
    let mg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    if group::is_exists(d, &mg, &|d, c| card::is_link_above(d, c, 2), 1, NO_EX) {
        // and ac:GetBattleTarget():IsLinkMonster() and ac:GetLink()>0 then
        let bt = card::get_battle_target(d, ac).nil();
        if card::is_link_monster(d, bt) && card::get_link(d, ac) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(p,ac:GetBattleTarget():GetLink()*500,REASON_EFFECT)
            let bt2 = card::get_battle_target(d, ac).nil();
            let amount2 = card::get_link(d, bt2) as i64 * 500;
            duel::damage(d, p, amount2, REASON_EFFECT, false, None);
        }
    }
}
/// `s.actfilter(c)`
fn actfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MARINCESS) and c:IsLinkMonster() and c:IsLinkAbove(3)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c) && card::is_link_above(d, c, 3)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.actfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&actfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
