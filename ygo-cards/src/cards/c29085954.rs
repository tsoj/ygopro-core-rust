//! Number 78: Number Archive
use super::prelude::*;

pub const ID: u32 = 29085954;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], xyz_number: Some(78), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    // Xyz.AddProcedure(c,nil,1,2)
    xyz::add_procedure(d, c, None, Some(1), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon from the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp,mc,pg)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, pg: &CardSet) -> bool {
    // local m=c:GetMetatable(true)
    // if not m then return false end
    // local no=m.xyz_number
    let no = crate::current_info(d, c).and_then(|m| m.xyz_number);
    // return no and no>=1 and no<=99 and c:IsSetCard(SET_NUMBER) and mc:IsCanBeXyzMaterial(c,tp)
    //     and (#pg<=0 or pg:IsContains(mc)) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    no.is_some_and(|no| (1..=99).contains(&no))
        && card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && (pg.is_empty() || group::is_contains(pg, mc))
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(e:GetHandler()),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(h)), tp, None, None, REASON_XYZ);
        // return #pg<=1 and Duel.GetLocationCountFromEx(tp,tp,e:GetHandler(),TYPE_XYZ)>0
        //     and Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0)>0 end
        return pg.len() <= 1
            && duel::get_location_count_from_ex(d, tp, Some(tp), h.into(), duel::ExtraCard::Type(TYPE_XYZ), None).0 > 0
            && duel::get_field_group_count(d, tp, LOCATION_EXTRA, 0) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 and Duel.GetLocationCountFromEx(tp,tp,c,TYPE_XYZ)>0
    //     and c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e) then
    if !g.is_empty()
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Type(TYPE_XYZ), None).0 > 0
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, e)
        && card::is_controler(d, c, tp)
        && !card::is_immune_to_effect(d, c, e)
    {
        // local tg=g:RandomSelect(1-tp,1)
        let tg = group::random_select(d, &g, 1 - tp, 1);
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - tp, tg.clone());
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
        // if tg:IsExists(s.filter,1,nil,e,tp,c,pg) then
        if group::is_exists(d, &tg, &|d, x| filter(d, x, e, tp, c, &pg), 1, NO_EX) {
            // local tc=tg:GetFirst()
            let tc = group::get_first(d, &tg).nil();
            // tc:SetMaterial(c)
            card::set_material(d, tc, CardSet::from_card(c));
            // Duel.Overlay(tc,c)
            duel::overlay(d, tc, c, false);
            // Duel.SpecialSummon(tc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c) as i64;
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetLabel(fid)
            effect::set_label(d, e1, &[fid]);
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // e1:SetCondition(s.rmcon)
            effect::set_condition(d, e1, Some(condition(rmcon)));
            // e1:SetOperation(s.rmop)
            effect::set_operation(d, e1, Some(operation(rmop)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tc = effect::get_label_object(d, e).as_card().nil();
    // if e:GetLabelObject():GetFlagEffectLabel(id)~=e:GetLabel() then
    let label = effect::get_label(d, e);
    if card::get_flag_effect_label(d, tc, ID).first().copied() != Some(label) {
        // e:Reset()
        effect::reset(d, e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
