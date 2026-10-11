//! Goblin Biker Grand Pileup
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 90946420;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Goblin" Xyz Monster by using an Xyz monster as material, then attach this card to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.xyzsumtg)
    effect::set_target(d, e1, Some(cost_target(xyzsumtg)));
    // e1:SetOperation(s.xyzsumop)
    effect::set_operation(d, e1, Some(operation(xyzsumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 card from the GY to a "Goblin" Xyz Monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.mattg)
    effect::set_target(d, e2, Some(cost_target(mattg)));
    // e2:SetOperation(s.matop)
    effect::set_operation(d, e2, Some(operation(matop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.xyzmatfilter(c,e,tp)`
fn xyzmatfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsFaceup() and c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,pg)
    (pg.is_empty() || (pg.len() == 1 && pg.iter().any(|x| x == c)))
        && card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,mc,pg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, _pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return c:IsSetCard(SET_GOBLIN) and c:IsType(TYPE_XYZ) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and mc:IsCanBeXyzMaterial(c) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_set_card(d, c, &[SET_GOBLIN as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), None, None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.xyzsumtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn xyzsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzmatfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzmatfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzmatfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| xyzmatfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzmatfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| xyzmatfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.xyzsumop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzsumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) or #pg>1
    //     or (#pg==1 and not pg:IsContains(tc)) then return end
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || pg.len() > 1
        || (pg.len() == 1 && !pg.iter().any(|x| x == tc))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,pg):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, tc, &pg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // c:CancelToGrave()
            card::cancel_to_grave(d, c, true);
            // Duel.Overlay(sc,c)
            duel::overlay(d, sc, c, false);
        }
    }
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_GOBLIN)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_GOBLIN as u16])
}
/// `s.atchfilter(c,tp)`
fn atchfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) or c:IsAbleToChangeControler()
    card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c)
}
/// `s.mattg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.atchfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| atchfilter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local xyzc=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let xyzc = group::get_first(d, &sel);
    // e:SetLabelObject(xyzc)
    effect::set_label_object(d, a.e, match xyzc {
        Some(x) => Value::Card(x),
        None => Value::Nil,
    });
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local tc=Duel.SelectTarget(tp,s.atchfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,tp)
    let tc = duel::select_target(d, tp, Some(&|d, c| atchfilter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, tc, 1, 0, 0);
    true
}
/// `s.matop(e,tp,eg,ep,ev,re,r,rp)`
fn matop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local tc=g:GetFirst()
    // local xyzc=g:GetNext()
    let order = group::iter(d, &g);
    let mut tc = order[0];
    let mut xyzc = order[1];
    // if tc==e:GetLabelObject() then tc,xyzc=xyzc,tc end
    if effect::get_label_object(d, a.e).as_card() == Some(tc) {
        std::mem::swap(&mut tc, &mut xyzc);
    }
    // Duel.Overlay(xyzc,tc)
    duel::overlay(d, xyzc, tc, false);
}
