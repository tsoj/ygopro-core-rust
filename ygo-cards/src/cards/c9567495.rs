//! Dragon Gate
//! ゲート・オブ・ドラゴン
use super::prelude::*;

pub const ID: u32 = 9567495;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 6 monsters OR 1 Rank 3 or lower Xyz Monster you control
    // Xyz.AddProcedure(c,nil,6,2,s.ovfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(6),
        2,
        Some(xyz::alter_filter(|d, c, tp, lc| ovfilter(d, c, tp, lc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Cannot be used as material for an Xyz Summon the turn it was Xyz Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e0:SetCondition(s.cannotxyzmatcon)
    effect::set_condition(d, e0, Some(lua_fn(cannotxyzmatcon)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Make this card able to attack all monsters your opponent controls, once each, also detach any number of materials ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsRankBelow(3) and c:IsType(TYPE_XYZ,lc,SUMMON_TYPE_XYZ,tp) and c:IsFaceup()
    card::is_rank_below(d, c, 3)
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_faceup(d, c)
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    // (Duel.RegisterFlagEffect returns nothing: the Lua value is nil)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    false
}
/// `s.cannotxyzmatcon(e)`
fn cannotxyzmatcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsXyzSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_xyz_summoned(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,c,1,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, tp, 1000);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // For the rest of this turn, this card can attack all monsters your opponent controls, once each
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e1, EFFECT_ATTACK_ALL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if c:RemoveOverlayCard(tp,1,c:GetOverlayCount(),REASON_EFFECT)>0 and c:IsFaceup() then
    let overlay = card::get_overlay_count(d, c) as u16;
    if card::remove_overlay_card(d, c, tp, 1, overlay, REASON_EFFECT) > 0 && card::is_faceup(d, c) {
        // local atk=1000*Duel.GetOperatedGroup():GetClassCount(Card.GetMainCardType)
        let operated = duel::get_operated_group(d);
        let atk = 1000 * group::get_class_count(d, &operated, &|d, x| card::get_main_card_type(d, x) as i64) as i64;
        // It gains 1000 ATK for each card type (Monster, Spell, or Trap) detached
        // if c:UpdateAttack(atk,RESETS_STANDARD_DISABLE_PHASE_END)==atk then
        if card::update_attack(d, c, atk, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None) as i64 == atk {
            // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
            let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
            // for tc in g:Iter() do
            for tc in group::iter(d, &g) {
                // tc:UpdateAttack(-atk,RESETS_STANDARD_PHASE_END,c)
                card::update_attack(d, tc, -atk, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
            }
        }
    }
}
