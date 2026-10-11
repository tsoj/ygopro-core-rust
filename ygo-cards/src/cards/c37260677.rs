//! Zoroa, the Magistus Verethragna
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 37260677;

// s.miracle_synchro_fusion=true
// s.listed_series={SET_MAGISTUS}
// s.material_setcode={SET_MAGISTUS}
pub const INFO: ScriptInfo = ScriptInfo {
    miracle_synchro_fusion: true,
    listed_series: &[SET_MAGISTUS],
    material_setcode: &[SET_MAGISTUS as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Magistus" monster + 1 Fusion, Synchro, Xyz, or Link Monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MAGISTUS),aux.FilterBoolFunctionEx(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_MAGISTUS as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK, Some(fc), st, Some(tp)))),
        ],
    );
    // Equip 1 Effect Monster your opponent controls or in your GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,function(ec,c,tp) return ec:IsType(TYPE_EFFECT) end,Card.EquipByEffectAndLimitRegister,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_type(d, a.c(0), TYPE_EFFECT)),
        lua_fn(|d, a| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)),
        e1,
        None,
        None,
        None,
    );
    // Negate a monster effect then you can destroy 1 card your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsMonsterEffect() and Duel.IsChainDisablable(ev) end)
    effect::set_condition(d, e2, Some(condition(e2cond)));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re:IsMonsterEffect() and Duel.IsChainDisablable(ev) end`
fn e2cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_EFFECT) and c:IsFaceup() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return s.eqfilter(chkc,tp) and ((chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp))
    //     or (chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp))) end
    if let Some(chkc) = a.chkc() {
        return eqfilter(d, chkc, a.tp)
            && ((card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp))
                || (card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp)));
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_GRAVE,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_GRAVE,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, a.tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsType(TYPE_EFFECT) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_type(d, tc, TYPE_EFFECT) && card::is_faceup(d, tc)) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then
    if !(card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
    {
        // if tc:IsControler(1-tp) then Duel.SendtoGrave(tc,REASON_RULE) end
        if card::is_controler(d, tc, 1 - a.tp) {
            duel::send_to_grave(d, tc, REASON_RULE, None, None);
        }
        // return
        return;
    } else {
        // c:EquipByEffectAndLimitRegister(e,tp,tc)
        card::equip_by_effect_and_limit_register(d, c, a.e, a.tp, tc, None, None);
    }
}
/// `s.discostfilter(c)`
fn discostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGISTUS) and c:IsMonsterCard() and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_MAGISTUS as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.discostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discostfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.discostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&discostfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, &sg, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        }
    }
}
