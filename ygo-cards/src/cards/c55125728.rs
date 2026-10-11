//! Vera the Vernusylph Goddess
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 55125728;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VERNUSYLPH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate activated effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take control of 1 opponent monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 EARTH monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev) and e:GetHandler():GetFlagEffect(id)==0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_EARTH),tp,LOCATION_MZONE,0,5,nil)
    if a.rp != 1 - a.tp || !effect::is_monster_effect(d, a.re.nil()) || !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) == 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)),
            a.tp,
            LOCATION_MZONE,
            0,
            5,
            NO_EX,
        )
}

/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // if Duel.GetFlagEffectLabel(tp,id)==cid or not Duel.SelectEffectYesNo(tp,c) then return end
    if duel::get_flag_effect_label(d, tp, ID).first().copied() == Some(cid) {
        return;
    }
    if !duel::select_effect_yes_no(d, tp, c, None) {
        return;
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let _ = card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1,cid)
    let _ = duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, cid as u32);
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, rc, REASON_EFFECT, None, None);
    }
}

/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_faceup(d, chkc)
            && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
            && duel::is_existing_target(d, Some(&control_filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&control_filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    true
}

/// `aux.FaceupFilter(Card.IsControlerCanBeChanged)`
fn control_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}

/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.GetControl(tc,tp) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // GetControl returns a number, which is always true in Lua
        duel::get_control(d, tc, tp, 0, 0, None, None);
        // Attribute becomes EARTH
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(ATTRIBUTE_EARTH)
        effect::set_value(d, e1, ATTRIBUTE_EARTH as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}

/// `s.spfilter(c,e,tp)`: `c:IsAttribute(ATTRIBUTE_EARTH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}

/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
