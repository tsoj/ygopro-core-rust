//! Appliancer Celtopus
//! Anime version scripted by pyrQ, updated by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 78225596;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_APPLIANCER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_APPLIANCER),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_APPLIANCER as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // cannot be targeted by attacks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.imcon)
    effect::set_condition(d, e1, Some(condition(imcon)));
    // e1:SetValue(aux.imval1)
    effect::set_value_fn(d, e1, aux::imval1());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be targeted by effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // increase ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // co-linked before destruction check
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.checkcon)
    effect::set_condition(d, e4, Some(condition(checkcon)));
    // e4:SetOperation(s.checkop)
    effect::set_operation(d, e4, Some(operation(checkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Draw
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e5, CATEGORY_DRAW);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e5, EVENT_DESTROYED);
    // e5:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(s.drcon)
    effect::set_condition(d, e5, Some(condition(drcon)));
    // e5:SetTarget(s.drtg)
    effect::set_target(d, e5, Some(cost_target(drtg)));
    // e5:SetOperation(s.drop)
    effect::set_operation(d, e5, Some(operation(drop_op)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.imcon(e)
fn imcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinked()
    let c = effect::get_handler(d, a.e).nil();
    card::is_linked(d, c)
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local a,b=Duel.GetAttacker(),Duel.GetAttackTarget()
    let at = duel::get_attacker(d);
    let bt = duel::get_attack_target(d);
    // if not (a and b) then return false end
    let (Some(mut ca), Some(mut cb)) = (at, bt) else {
        return false;
    };
    // if a:IsControler(1-tp) then
    if card::is_controler(d, ca, 1 - tp) {
        // a,b=b,a
        std::mem::swap(&mut ca, &mut cb);
    }
    // local mg=a:GetMutualLinkedGroup()
    let mg = card::get_mutual_linked_group(d, ca);
    // local octg=e:GetHandler():GetMutualLinkedGroup()
    let octg = card::get_mutual_linked_group(d, h);
    // return a and a:IsControler(tp) and a:IsLinkMonster() and a:IsSetCard(SET_APPLIANCER)
    //     and a~=e:GetHandler() and b and b:IsControler(1-tp)
    //     and mg:IsContains(e:GetHandler()) and octg:IsContains(a)
    card::is_controler(d, ca, tp)
        && card::is_link_monster(d, ca)
        && card::is_set_card(d, ca, &[SET_APPLIANCER as u16])
        && ca != h
        && card::is_controler(d, cb, 1 - tp)
        && mg.contains(h)
        && octg.contains(ca)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local g=e:GetHandler():GetMutualLinkedGroup()
    let g = card::get_mutual_linked_group(d, h);
    // if not g then return end
    // (a group is never nil here)
    // local atkct=#g:Filter(Card.IsMonster,nil)
    let atkct = group::filter(d, &g, &|d, c| card::is_monster(d, c), NO_EX).len();
    // local a,b=Duel.GetAttacker(),Duel.GetAttackTarget()
    let mut a_c = duel::get_attacker(d);
    let b_c = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then
    if card::is_controler(d, a_c.nil(), 1 - tp) {
        // a,b=b,a
        a_c = b_c;
    }
    // if a and a:IsRelateToBattle() and a:IsFaceup() and a:IsControler(tp) and atkct>0 then
    if let Some(a_c) = a_c {
        if card::is_relate_to_battle(d, a_c) && card::is_faceup(d, a_c) && card::is_controler(d, a_c, tp) && atkct > 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // e1:SetValue(atkct*1000)
            effect::set_value(d, e1, atkct as i64 * 1000);
            // a:RegisterEffect(e1)
            card::register_effect(d, a_c, e1, false);
        }
    }
}
// function s.checkfilter(c,e,tp)
fn checkfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let h = effect::get_handler(d, e).nil();
    // local mg=c:GetMutualLinkedGroup()
    let mg = card::get_mutual_linked_group(d, c);
    // local octg=e:GetHandler():GetMutualLinkedGroup()
    let octg = card::get_mutual_linked_group(d, h);
    // return c:IsSetCard(SET_APPLIANCER) and c:IsLinkMonster() and c:IsControler(tp) and c:IsReason(REASON_DESTROY)
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not (mg:IsContains(e:GetHandler()) and octg:IsContains(c))
    card::is_set_card(d, c, &[SET_APPLIANCER as u16])
        && card::is_link_monster(d, c)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !(mg.contains(h) && octg.contains(c))
}
// function s.checkcon(e,tp,eg,ep,ev,re,r,rp)
fn checkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.checkfilter,1,e:GetHandler(),e,tp)
    !eg.contains(h) && group::is_exists(d, &eg, &|d, c| checkfilter(d, c, a.e, a.tp), 1, h.into())
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
