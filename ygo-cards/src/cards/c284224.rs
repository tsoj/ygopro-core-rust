//! Dual Avatar - Empowered Mitsu-Jaku
//! 双天将 密迹
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 284224;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_DUAL_AVATAR], listed_series: &[SET_DUAL_AVATAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion summon procedure
    // Fusion.AddProcMixN(c,true,true,11759079,1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DUAL_AVATAR),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::Code(11759079), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DUAL_AVATAR as u16], Some(fc), st, Some(tp)))), 2),
        ],
    );
    // Each "Dual Avatar" fusion monster get protected once by battle, each turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| indtg(d, c))));
    // e1:SetValue(s.indct)
    effect::set_value_fn(d, e1, lua_fn(|_d, a| indct(a)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return all of opponent's spells/traps to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy opponent's monster, that activated its effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.indtg(e,c)
fn indtg(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsSetCard(SET_DUAL_AVATAR)
    card::is_type(d, c, TYPE_FUSION) && card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16])
}
// function s.indct(e,re,r,rp)
fn indct(a: Args<'_>) -> i64 {
    // if r&REASON_BATTLE==REASON_BATTLE then
    if (a.int(2) as u32) & REASON_BATTLE == REASON_BATTLE {
        // return 1
        1
    } else {
        // else return 0 end
        0
    }
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,#sg,0,0)
    let n = group::get_count(&sg) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), n, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp,chk)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SendtoHand(sg,nil,REASON_EFFECT)
    duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsOnField() and re:IsMonsterEffect() and ep==1-tp
    //   and Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_FUSION),tp,LOCATION_MZONE,0,nil)>=2
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if !card::is_on_field(d, rh) {
        return false;
    }
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if a.ep != 1 - a.tp {
        return false;
    }
    duel::get_matching_group_count(d, Some(&|d: &mut Duel, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION)), a.tp, LOCATION_MZONE, 0, NO_EX) >= 2
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
}
