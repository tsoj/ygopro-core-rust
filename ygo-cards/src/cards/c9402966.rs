//! Superheavy Samurai Battleball
//! 超重武者タマ－C
use super::prelude::*;

pub const ID: u32 = 9402966;

// s.listed_series={SET_SUPERHEAVY_SAMURAI}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.sccon)
    effect::set_condition(d, e1, Some(condition(sccon)));
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsSetCard(SET_SUPERHEAVY_SAMURAI)
    card::is_facedown(d, c) || !card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])
}
/// `s.filter(c,e,tp,lv)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsFaceup() and c:GetLevel()>0
    //     and Duel.IsExistingMatchingCard(s.scfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv+c:GetOriginalLevel())
    card::is_faceup(d, c) && card::get_level(d, c) > 0 && {
        let sum = lv + card::get_original_level(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| scfilter(d, x, e, tp, sum)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
    }
}
/// `s.scfilter(c,e,tp,lv)`
fn scfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsSetCard(SET_SUPERHEAVY_SAMURAI) and c:IsLevel(lv) and c:IsType(TYPE_SYNCHRO)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])
        && card::is_level(d, c, &[lv])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.sccon(e,tp,eg,ep,ev,re,r,rp)`
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_GRAVE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local lv=e:GetHandler():GetOriginalLevel()
    let h = effect::get_handler(d, a.e).nil();
    let lv = card::get_original_level(d, h);
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
        // return #pg<=0 and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,e,tp,lv) end
        return pg.is_empty()
            && duel::is_existing_target(d, Some(&|d, x| filter(d, x, a.e, tp, lv)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,e,tp,lv)
    let mut g = duel::select_target(d, tp, Some(&|d, x| filter(d, x, a.e, tp, lv)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
    // if not c:IsRelateToEffect(e) or not tc or not tc:IsRelateToEffect(e) or #pg>0 then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) || !pg.is_empty() {
        return;
    }
    // local g=Group.FromCards(c,tc)
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(tc));
    // if Duel.SendtoGrave(g,REASON_EFFECT)==2 and c:GetLevel()>0 and c:IsLocation(LOCATION_GRAVE)
    //     and tc:GetLevel()>0 and tc:IsLocation(LOCATION_GRAVE) then
    if duel::send_to_grave(d, g, REASON_EFFECT, None, None) == 2
        && card::get_level(d, c) > 0
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::get_level(d, tc) > 0
        && card::is_location(d, tc, LOCATION_GRAVE)
    {
        // local lv=c:GetLevel()+tc:GetLevel()
        let lv = card::get_level(d, c) + card::get_level(d, tc);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.scfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv)
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| scfilter(d, x, a.e, tp, lv)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // local tc=sg:GetFirst()
        // if tc then
        if let Some(sel) = group::get_first(d, &sg) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(tc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sel, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None);
            // tc:CompleteProcedure()
            card::complete_procedure(d, sel);
        }
    }
}
