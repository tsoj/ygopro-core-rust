//! Star Power!!
//! --一曲集中
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 76500786;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted link monster gains ATK, equal to total levels/ranks of monsters it points to x 400
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsLevelAbove(1) or c:IsRankAbove(1))
    card::is_faceup(d, c) && (card::is_level_above(d, c, 1) || card::is_rank_above(d, c, 1))
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let mut at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let mut df = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a=Duel.GetAttackTarget() d=Duel.GetAttacker() end
    if card::is_controler(d, at.nil(), 1 - tp) {
        at = duel::get_attack_target(d);
        df = duel::get_attacker(d);
    }
    // return a and d and a:IsLinkMonster() and a:GetLinkedGroup():IsExists(s.filter,1,nil)
    match (at, df) {
        (Some(at), Some(_)) => {
            card::is_link_monster(d, at) && {
                let g = card::get_linked_group(d, at);
                group::is_exists(d, &g, &filter, 1, NO_EX)
            }
        }
        _ => false,
    }
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let mut at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let df = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a=Duel.GetAttackTarget() end
    if card::is_controler(d, at.nil(), 1 - tp) {
        at = duel::get_attack_target(d);
    }
    let at = at.nil();
    // if a:IsFaceup() and a:IsRelateToBattle() and d:IsFaceup() and d:IsRelateToBattle() then
    if card::is_faceup(d, at) && card::is_relate_to_battle(d, at) {
        let df = df.nil();
        if card::is_faceup(d, df) && card::is_relate_to_battle(d, df) {
            // local g=a:GetLinkedGroup():Filter(s.filter,nil)
            let lg = card::get_linked_group(d, at);
            let g = group::filter(d, &lg, &filter, NO_EX);
            // if #g==0 then return end
            if g.is_empty() {
                return;
            }
            // local tot=0
            let mut tot: i64 = 0;
            // for tc in aux.Next(g) do tot=tot+math.max(tc:GetLevel(),tc:GetRank()) end
            for tc in group::iter(d, &g) {
                let lv = card::get_level(d, tc) as i64;
                let rk = card::get_rank(d, tc) as i64;
                tot += lv.max(rk);
            }
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(tot*400)
            effect::set_value(d, e1, tot * 400);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // a:RegisterEffect(e1)
            card::register_effect(d, at, e1, false);
        }
    }
}
