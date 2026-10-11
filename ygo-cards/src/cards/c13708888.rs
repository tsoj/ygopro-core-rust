//! Evil HERO Neos Lord
use super::prelude::*;

pub const ID: u32 = 13708888;

pub const INFO: ScriptInfo = ScriptInfo {
    dark_calling: true,
    listed_names: &[CARD_NEOS, CARD_DARK_FUSION],
    listed_series: &[SET_HERO, SET_EVIL_HERO],
    material_setcode: &[SET_ELEMENTAL_HERO, SET_HERO, SET_NEOS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Elemental HERO Neos" or 1 Fusion Monster that mentions it + 1 Effect Monster on the field
    // Fusion.AddProcMix(c,true,true,{CARD_NEOS,s.neosfusionmatfilter},s.effectmatfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Any(vec![
                fusion::Mat::Code(CARD_NEOS),
                fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| neosfusionmatfilter(d, c, fc, st, tp))),
            ]),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| effectmatfilter(d, c, fc, st, tp))),
        ],
    );
    // c:AddMustBeSpecialSummonedByDarkFusion()
    aux::add_must_be_special_summoned_by_dark_fusion(d, c);
    // Take control of 1 face-up monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.ctrltg)
    effect::set_target(d, e1, Some(cost_target(ctrltg)));
    // e1:SetOperation(s.ctrlop)
    effect::set_operation(d, e1, Some(operation(ctrlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.ctrlcon)
    effect::set_condition(d, e2, Some(condition(ctrlcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by battle or card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.neosfusionmatfilter(c,fc,sumtype,tp)
fn neosfusionmatfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION,fc,sumtype,tp) and c:ListsCode(CARD_NEOS)
    card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp)) && card::lists_code(d, c, &[CARD_NEOS])
}
// function s.effectmatfilter(c,fc,sumtype,tp)
fn effectmatfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_EFFECT,fc,sumtype,tp) and c:IsOnField()
    card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), sumtype, Some(tp)) && card::is_on_field(d, c)
}
// the target filter of e1 with `aux.FaceupFilter(Card.IsControlerCanBeChanged)`
fn ctrl_target_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, Some(0xff))
}
// function s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsControlerCanBeChanged() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_controler_can_be_changed(d, chkc, false, Some(0xff))
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctrl_target_filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&ctrl_target_filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, a.tp, 0);
    true
}
// function s.ctrlop(e,tp,eg,ep,ev,re,r,rp)
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.GetControl(tc,tp)
        duel::get_control(d, tc, a.tp, 0, 0, Some(0xff), None);
    }
}
// function s.ctrlconfilter(c,tp)
fn ctrlconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsMonster()
    card::is_controler(d, c, 1 - tp) && card::is_monster(d, c)
}
// function s.ctrlcon(e,tp,eg,ep,ev,re,r,rp)
fn ctrlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.ctrlconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| ctrlconfilter(d, c, tp), 1, NO_EX)
}
