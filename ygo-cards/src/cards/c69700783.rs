//! Vampire's Desire
use super::prelude::*;

pub const ID: u32 = 69700783;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAMPIRE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter1(c,tp)`
fn tgfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsFaceup() and Duel.IsExistingMatchingCard(s.tgfilter2,tp,LOCATION_DECK,0,1,nil,lv)
    lv > 0 && card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| tgfilter2(d, x, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.tgfilter2(c,lv)`
fn tgfilter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_VAMPIRE) and not c:IsLevel(lv) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_VAMPIRE as u16]) && !card::is_level(d, c, &[lv]) && card::is_able_to_grave(d, c, None)
}
/// `s.spfilter1(c,tp)`
fn spfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToGrave() and Duel.GetMZoneCount(tp,c)>0
    card::is_able_to_grave(d, c, None) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VAMPIRE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_VAMPIRE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if e:GetLabel()==0 then
        if effect::get_label(d, a.e) == 0 {
            // return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter1(chkc,tp)
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter1(d, chkc, a.tp);
        } else {
            // return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter2(chkc,e,tp)
            return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter2(d, chkc, a.e, a.tp);
        }
    }
    // local b1=Duel.IsExistingTarget(s.tgfilter1,tp,LOCATION_MZONE,0,1,nil,tp)
    let b1 = duel::is_existing_target(d, Some(&|d, c| tgfilter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and Duel.IsExistingTarget(s.spfilter2,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| spfilter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_target(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    // if b1 and b2 then
    let op: i64 = if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)]) as i64
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) as i64 + 1
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 0 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // Duel.SelectTarget(tp,s.tgfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
        let _g = duel::select_target(d, a.tp, Some(&|d, c| tgfilter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    } else {
        // e:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter2,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_MZONE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local tc=Duel.GetFirstTarget()
        let tc0 = duel::first_target(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter2,tp,LOCATION_DECK,0,1,1,nil,tc:GetLevel())
        let tc = tc0.nil();
        let lv = card::get_level(d, tc);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter2(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)~=0 and g:GetFirst():IsLocation(LOCATION_GRAVE)
        //     and tc:IsRelateToEffect(e) and tc:IsFaceup() then
        if !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) != 0 {
            let first = group::get_first(d, &g).nil();
            if card::is_location(d, first, LOCATION_GRAVE) && card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
                // local lv=g:GetFirst():GetLevel()
                let glv = card::get_level(d, first);
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CHANGE_LEVEL)
                effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
                // e1:SetValue(lv)
                effect::set_value(d, e1, glv as i64);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    } else {
        // local tc=Duel.GetFirstTarget()
        let tc0 = duel::first_target(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)~=0 and g:GetFirst():IsLocation(LOCATION_GRAVE)
        //     and tc:IsRelateToEffect(e) then
        if !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) != 0 {
            let first = group::get_first(d, &g).nil();
            if card::is_location(d, first, LOCATION_GRAVE) && card::is_relate_to_effect(d, tc0.nil(), a.e) {
                // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, tc0.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
