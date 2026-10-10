//! Galaxy-Eyes Full Armor Photon Dragon
use super::prelude::*;

pub const ID: u32 = 39030163;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_GALAXY_EYES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 8 monsters OR 1 "Galaxy-Eyes" Xyz Monster you control, except "Galaxy-Eyes Full Armor Photon Dragon"
    // Xyz.AddProcedure(c,nil,8,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(d, c, None, Some(8), 3, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), None, None, false, None);
    // Attach up to 2 Equip Cards equipped to this card to this card as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.HintSelectedEffect)
    effect::set_cost(d, e1, Some(cost::hint_selected_effect()));
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 face-up card your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.AND(Cost.DetachFromSelf(1),Cost.HintSelectedEffect))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost::hint_selected_effect()])));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsSetCard(SET_GALAXY_EYES,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp)
    //     and not c:IsSummonCode(xyzc,SUMMON_TYPE_XYZ,tp,id) and c:IsFaceup()
    card::is_set_card_ctx(d, c, &[SET_GALAXY_EYES as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && !card::is_summon_code(d, c, Some(xyzc), SUMMON_TYPE_XYZ as u64, tp, &[ID])
        && card::is_faceup(d, c)
}
/// `s.attachfilter(c,xyzc,e,tp)`
fn attachfilter(d: &mut Duel, c: CardId, xyzc: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT) and c:IsCanBeEffectTarget(e)
    card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT)) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local eqg=c:GetEquipGroup()
    let eqg = card::get_equip_group(d, c);
    // if chkc then return eqg:IsContains(chkc) and s.attachfilter(chkc,c,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eqg, chkc) && attachfilter(d, chkc, c, e, tp);
    }
    // if chk==0 then return eqg:IsExists(s.attachfilter,1,nil,c,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eqg, &|d, x| attachfilter(d, x, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tg=eqg:FilterSelect(tp,s.attachfilter,1,2,nil,c,e,tp)
    let tg = group::filter_select(d, &eqg, tp, &|d, x| attachfilter(d, x, c, e, tp), 1, 2, false, NO_EX).nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Match(aux.NOT(Card.IsImmuneToEffect),nil,e)
    let targets = duel::get_target_cards(d, a.e).nil();
    let tg = group::filter(d, &targets, &|d, x| !card::is_immune_to_effect(d, x, a.e), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Overlay(c,tg)
        duel::overlay(d, c, tg, false);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
