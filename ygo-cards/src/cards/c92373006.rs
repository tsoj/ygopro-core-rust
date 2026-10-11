//! Test Tiger
use super::prelude::*;

pub const ID: u32 = 92373006;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon rule
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.sprcon)
    effect::set_condition(d, e1, Some(lua_fn(sprcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GLADIATOR),tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GLADIATOR as u16])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tgfilter(c,ft)`
fn tgfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GLADIATOR) and c:IsAbleToDeck() and (ft>-1 or c:GetSequence()<5)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_able_to_deck(d, c) && (ft > -1 || card::get_sequence(d, c) < 5)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and c:IsCanBeSpecialSummoned(e,130,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_can_be_special_summoned(d, c, e, 130, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, ft);
    }
    // if chk==0 then return ft>-2 and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,ft)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > -2
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,ft)
    duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local rc=Duel.GetFirstTarget()
    // if not rc or not rc:IsFaceup() or not rc:IsRelateToEffect(e) then return end
    let Some(rc) = duel::first_target(d) else {
        return;
    };
    if !card::is_faceup(d, rc) || !card::is_relate_to_effect(d, rc, a.e) {
        return;
    }
    // local rt=Duel.SendtoDeck(rc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let rt = duel::send_to_deck(d, rc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // if rt==0 then return end
    if rt == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,130,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 130, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // tc:RegisterFlagEffect(tc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
            let code = card::get_original_code(d, tc);
            card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        }
    }
}
