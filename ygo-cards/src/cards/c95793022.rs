//! World Chalice Guardragon Almarduke
use super::prelude::*;

pub const ID: u32 = 95793022;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_LINK),3)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_LINK, Some(fc), st, Some(tp)))), 3)],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,nil,nil,nil,false)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
    // attack all
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e1, EFFECT_ATTACK_ALL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy and damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION or e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let st = a.int(3) as u32;
    if (st & SUMMON_TYPE_FUSION) == SUMMON_TYPE_FUSION {
        return true;
    }
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, c) != LOCATION_EXTRA
}
// function s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetReleaseGroup(tp)
    duel::get_release_group(d, tp, false, false, None)
}
// function s.contactop(g)
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g.clone(), REASON_COST | REASON_MATERIAL, None);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let dd = card::get_battle_target(d, att);
    // local tc=nil
    // if a==c then tc=d elseif d and d==c then tc=a end
    let tc = if att == c {
        dd
    } else if dd == Some(c) {
        Some(att)
    } else {
        None
    };
    // return tc and tc:IsLinkMonster() and tc:IsControler(1-tp)
    match tc {
        Some(tc) => card::is_link_monster(d, tc) && card::is_controler(d, tc, 1 - a.tp),
        None => false,
    }
}
// function s.descfilter(c,lk)
fn descfilter(d: &mut Duel, c: CardId, lk: u32) -> bool {
    // return c:IsLinkMonster() and c:GetLink()==lk and c:IsAbleToRemoveAsCost()
    card::is_link_monster(d, c) && card::get_link(d, c) == lk && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.descost(e,tp,eg,ep,ev,re,r,rp,chk)
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if chk==0 then return tc and tc:IsLinkMonster() and Duel.IsExistingMatchingCard(s.descfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tc:GetLink()) end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => {
                let lk = card::get_link(d, tc);
                card::is_link_monster(d, tc)
                    && duel::is_existing_matching_card(d, Some(&|d, c| descfilter(d, c, lk)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
            }
            None => false,
        };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.descfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tc:GetLink())
    let tc = tc.nil();
    let lk = card::get_link(d, tc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| descfilter(d, c, lk)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, c) {
        return;
    }
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c).nil();
    // if tc:IsRelateToBattle() and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if card::is_relate_to_battle(d, tc) && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0 {
        // Duel.Damage(1-tp,tc:GetBaseAttack(),REASON_EFFECT)
        let atk = card::get_base_attack(d, tc) as i64;
        duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
    }
}
