//! Secret Six Samurai - Rihan
use super::prelude::*;

pub const ID: u32 = 33964637;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SIX_SAMURAI],
    material_setcode: &[SET_SIX_SAMURAI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // Fusion.AddProcMixN(c,true,true,s.ffilter,3)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 3)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // Cannot be used as Fusion Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.rmcost)
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destruction replacement for a "Six Samurai" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, effect_card_fn(repval));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.ffilter(c,fc,sumtype,tp,sub,mg,sg)
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return c:IsSetCard(SET_SIX_SAMURAI,fc,sumtype,tp) and c:GetAttribute(fc,sumtype,tp)~=0
    if !card::is_set_card_ctx(d, c, &[SET_SIX_SAMURAI as u16], Some(a.fc), a.sumtype, Some(a.tp)) {
        return false;
    }
    let attr = card::get_attribute_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp));
    if attr == 0 {
        return false;
    }
    // and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetAttribute(fc,sumtype,tp),fc,sumtype,tp))
    match a.sg {
        None => true,
        Some(sg) => !group::is_exists(d, sg, &|d, x| fusfilter(d, x, attr, a.fc, a.sumtype, a.tp), 1, c.into()),
    }
}
// s.fusfilter(c,attr,fc,sumtype,tp)
fn fusfilter(d: &mut Duel, c: CardId, attr: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(attr,fc,sumtype,tp) and not c:IsHasEffect(511002961)
    card::is_attribute_ctx(d, c, attr, Some(fc), sumtype, Some(tp)) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
// s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
// s.contactop(g)
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g, REASON_COST | REASON_MATERIAL, None, None);
}
// s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, h) != LOCATION_EXTRA
}
// s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SIX_SAMURAI) and c:IsAbleToRemoveAsCost()
    //     and ((c:IsOnField() and c:IsFaceup()) or c:IsLocation(LOCATION_HAND))
    //     and Duel.IsExistingTarget(Card.IsAbleToRemove,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && ((card::is_on_field(d, c) && card::is_faceup(d, c)) || card::is_location(d, c, LOCATION_HAND))
        && duel::is_existing_target(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
}
// s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
// s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI) and not c:IsReason(REASON_REPLACE)
    //     and c:IsOnField() and c:IsControler(tp) and c:IsReason(REASON_EFFECT|REASON_BATTLE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_on_field(d, c)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
// s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
// s.repval(e,c)
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    repfilter(d, c, effect::get_handler_player(d, e))
}
// s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
