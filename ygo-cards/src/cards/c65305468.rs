//! Number F0: Utopic Future
use super::prelude::*;

pub const ID: u32 = 65305468;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(0), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.xyzfilter,nil,2,nil,nil,nil,nil,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| xyzfilter(d, c, sc, st, tp))),
        None,
        2,
        None,
        None,
        None,
        None,
        false,
        Some(xyz::ex_chk(xyzcheck)),
    );
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // avoid damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e4, EFFECT_NO_BATTLE_DAMAGE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // control
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e6, CATEGORY_CONTROL);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e6, EVENT_DAMAGE_STEP_END);
    // e6:SetTarget(s.cttg)
    effect::set_target(d, e6, Some(cost_target(cttg)));
    // e6:SetOperation(s.ctop)
    effect::set_operation(d, e6, Some(operation(ctop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // destroy replace
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e7:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e7:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e7, EFFECT_DESTROY_REPLACE);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetTarget(s.reptg)
    effect::set_target(d, e7, Some(cost_target(reptg)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.xyzfilter(c,xyz,sumtype,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ,xyz,sumtype,tp) and not c:IsSetCard(SET_NUMBER,xyz,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_XYZ, xyz, sumtype, Some(tp)) && !card::is_set_card_ctx(d, c, &[SET_NUMBER as u16], xyz, sumtype, Some(tp))
}
/// `s.xyzcheck(g,tp,xyz)`
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    // local mg=g:Filter(function(c) return not c:IsHasEffect(EFFECT_EQUIP_SPELL_XYZ_MAT) end,nil)
    let mg = group::filter(d, g, &|d, c| card::is_has_effect(d, c, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
    // return mg:GetClassCount(Card.GetRank)==1
    group::get_class_count(d, &mg, &|d, c| card::get_rank(d, c) as i64) == 1
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if chk==0 then return tc and tc:IsRelateToBattle() and tc:IsControlerCanBeChanged() end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_relate_to_battle(d, tc) && card::is_controler_can_be_changed(d, tc, false, None),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h).nil();
    // if tc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tc) {
        // Duel.GetControl(tc,tp,PHASE_BATTLE,1)
        duel::get_control(d, tc, a.tp, PHASE_BATTLE as u16, 1, None, None);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT)
    //     and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_reason(d, c, REASON_EFFECT)
            && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
