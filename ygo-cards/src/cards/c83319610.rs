//! Zenmaiday
use super::prelude::*;

pub const ID: u32 = 83319610;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WIND_UP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.atchtg)
    effect::set_target(d, e2, Some(cost_target(atchtg)));
    // e2:SetOperation(s.atchop)
    effect::set_operation(d, e2, Some(operation(atchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tfilter(c,tp)
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsRace(RACE_MACHINE)
    //   and Duel.IsExistingMatchingCard(s.atchfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_race(d, c, RACE_MACHINE)
        && duel::is_existing_matching_card(d, Some(&atchfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, c.into())
}
// function s.atchfilter(c)
fn atchfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WIND_UP) and c:IsMonster() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_WIND_UP as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
// function s.atchtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tfilter(d, chkc, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0
    //   and Duel.IsExistingTarget(s.tfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && duel::is_existing_target(d, Some(&|d, cc| tfilter(d, cc, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, cc| tfilter(d, cc, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.atchop(e,tp,eg,ep,ev,re,r,rp)
fn atchop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
    // local atchc=Duel.SelectMatchingCard(tp,s.atchfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,tc):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&atchfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, tc.into()).nil();
    let atchc = group::get_first(d, &g);
    // if atchc and not atchc:IsImmuneToEffect(e) and atchc:IsCanBeXyzMaterial(tc,tp,REASON_EFFECT) then
    if let Some(atchc) = atchc {
        if !card::is_immune_to_effect(d, atchc, a.e) && card::is_can_be_xyz_material(d, atchc, Some(tc), Some(a.tp), Some(REASON_EFFECT)) {
            // Duel.Overlay(tc,atchc,true)
            duel::overlay(d, tc, atchc, true);
        }
    }
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
// function s.spfilter1(c,e,tp)
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsFaceup() and c:IsSetCard(SET_WIND_UP)
    //   and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetRank()+1,pg)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_WIND_UP as u16])
        && {
            let rk = card::get_rank(d, c) + 1;
            duel::is_existing_matching_card(d, Some(&|d, cc| spfilter2(d, cc, e, tp, c, rk)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
// function s.spfilter2(c,e,tp,mc,rk,pg)
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32) -> bool {
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:GetRank()==rk and c:IsSetCard(SET_WIND_UP)
    //   and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and mc:IsCanBeXyzMaterial(c,tp)
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::get_rank(d, c) == rk
        && card::is_set_card(d, c, &[SET_WIND_UP as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.spfilter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && spfilter1(d, chkc, a.e, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsStatus(STATUS_CHAINING) and c:GetFlagEffect(id)==0
    //   and Duel.IsExistingTarget(s.spfilter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return !card::is_status(d, c, STATUS_CHAINING)
            && card::get_flag_effect(d, c, ID) == 0
            && duel::is_existing_target(d, Some(&|d, cc| spfilter1(d, cc, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.spfilter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, a.tp, Some(&|d, cc| spfilter1(d, cc, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let tc_group = match tc {
        Some(t) => CardSet::from_card(t),
        None => CardSet::new(),
    };
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&tc_group), tp, None, None, REASON_XYZ);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e)
    //   or #pg>1 or (#pg==1 and not pg:IsContains(tc)) then return end
    let Some(tc) = tc else { return };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, tc))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetRank()+1,pg)
    let rk = card::get_rank(d, tc) + 1;
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter2(d, cc, a.e, tp, tc, rk)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
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
    }
}
