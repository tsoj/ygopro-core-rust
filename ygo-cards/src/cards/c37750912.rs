//! Stardust Illumination
//! スターダスト・イルミネイト
use super::prelude::*;

pub const ID: u32 = 37750912;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], listed_series: &[SET_STARDUST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Stardust" monster from your Deck to the GY, or if you control "Stardust Dragon" or a Synchro Monster that mentions it, you can Special Summon it instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase or decrease the Level of 1 "Stardust" monster you control by 1 until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_STARDUST_DRAGON) or (c:ListsCode(CARD_STARDUST_DRAGON) and c:IsType(TYPE_SYNCHRO) and c:IsMonster())) and c:IsFaceup()
    (card::is_code(d, c, &[CARD_STARDUST_DRAGON])
        || (card::lists_code(d, c, &[CARD_STARDUST_DRAGON]) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_monster(d, c)))
        && card::is_faceup(d, c)
}
/// `s.tgfilter(c,e,tp,stardust_mzone_chk)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, stardust_mzone_chk: bool) -> bool {
    // return c:IsSetCard(SET_STARDUST) and c:IsMonster() and (c:IsAbleToGrave() or (stardust_mzone_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_STARDUST as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_grave(d, c, None)
            || (stardust_mzone_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `local stardust_mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_ONFIELD,0,1,nil)`
fn stardust_mzone_chk(d: &mut Duel, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local stardust_mzone_chk=...
        let chk = stardust_mzone_chk(d, a.tp);
        // return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,e,tp,stardust_mzone_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp, chk)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local stardust_mzone_chk=...
    let chk = stardust_mzone_chk(d, a.tp);
    // local hintmsg=stardust_mzone_chk and aux.Stringid(id,2) or HINTMSG_TOGRAVE
    let hintmsg = if chk { aux::stringid(ID, 2) } else { HINTMSG_TOGRAVE as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hintmsg)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, hintmsg);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,stardust_mzone_chk):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter(d, c, a.e, a.tp, chk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // local op=1
    let mut op: Option<usize> = Some(1);
    // if stardust_mzone_chk then
    if chk {
        // local b1=sc:IsAbleToGrave()
        let b1 = card::is_able_to_grave(d, sc, None);
        // local b2=sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b2 = card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None);
        // op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,3)}, {b2,aux.Stringid(id,4)})
        op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    }
    // if op==1 then
    if op == Some(1) {
        // Duel.SendtoGrave(sc,REASON_EFFECT)
        duel::send_to_grave(d, sc, REASON_EFFECT, None, None);
    } else if op == Some(2) {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_STARDUST) and c:HasLevel() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_STARDUST as u16]) && card::has_level(d, c) && card::is_faceup(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.lvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,tc,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, tc.map(CardSet::from_card), 1, a.tp, 1);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:HasLevel() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::has_level(d, tc) {
        // local op=tc:IsLevelAbove(2) and Duel.SelectOption(tp,aux.Stringid(id,5),aux.Stringid(id,6)) or 0
        let op = if card::is_level_above(d, tc, 2) {
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 5), aux::stringid(ID, 6)])
        } else {
            0
        };
        // local lv=op==0 and 1 or -1
        let lv = if op == 0 { 1 } else { -1 };
        // Increase or decrease its Level by 1 until the end of this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
