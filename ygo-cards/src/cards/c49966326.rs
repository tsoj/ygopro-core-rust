//! Virtual World Kirin - Lili
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 49966326;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VIRTUAL_WORLD], ..ScriptInfo::DEFAULT };

/// `local key=TYPE_MONSTER+TYPE_SPELL+TYPE_TRAP`
const KEY: u32 = TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgtfilter(c,tp)`
fn tgtfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VIRTUAL_WORLD) and Duel.IsExistingMatchingCard(s.tgvfilter,tp,LOCATION_DECK,0,1,nil,c:GetType()&key)
    if !card::is_faceup(d, c) || !card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) {
        return false;
    }
    let t = card::get_type(d, c) & KEY;
    duel::is_existing_matching_card(d, Some(&|d, x| tgvfilter(d, x, t)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.tgvfilter(c,type1)`
fn tgvfilter(d: &mut Duel, c: CardId, type1: u32) -> bool {
    // return c:IsAbleToGrave() and c:IsSetCard(SET_VIRTUAL_WORLD) and not c:IsType(type1)
    card::is_able_to_grave(d, c, None) && card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) && !card::is_type(d, c, type1)
}
/// `s.tgvfilter2(c,type1,type2)`
fn tgvfilter2(d: &mut Duel, c: CardId, type1: u32, type2: u32) -> bool {
    // return c:IsAbleToGrave() and c:IsSetCard(SET_VIRTUAL_WORLD) and not c:IsType(type1) and not c:IsType(type2) and not c:IsCode(id)
    card::is_able_to_grave(d, c, None)
        && card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16])
        && !card::is_type(d, c, type1)
        && !card::is_type(d, c, type2)
        && !card::is_code(d, c, &[ID])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(tp) and s.tgtfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, a.tp) && tgtfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgtfilter,tp,LOCATION_ONFIELD,0,1,nil,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgtfilter(d, x, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && {
                let c = effect::get_handler(d, a.e).nil();
                card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.tgtfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, x| tgtfilter(d, x, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,LOCATION_HAND)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e0:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e0, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetTarget(s.splimit)
    effect::set_target(d, e0, Some(effect_card_fn(splimit)));
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // aux.RegisterClientHint(c,EFFECT_FLAG_OATH,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, EFFECT_FLAG_OATH, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local tg=Duel.SelectMatchingCard(tp,s.tgvfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetType()&key)
            let t1 = card::get_type(d, tc) & KEY;
            let tg = duel::select_matching_card(d, a.tp, Some(&|d, x| tgvfilter(d, x, t1)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #tg>0 and Duel.SendtoGrave(tg,REASON_EFFECT)>0 then
            if !tg.is_empty() && duel::send_to_grave(d, &tg, REASON_EFFECT, None, None) > 0 {
                // local ogc=Duel.GetOperatedGroup():GetFirst()
                let og = duel::get_operated_group(d);
                let ogc = group::get_first(d, &og).nil();
                // if ogc:IsLocation(LOCATION_GRAVE) and c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
                if card::is_location(d, ogc, LOCATION_GRAVE)
                    && card::is_relate_to_effect(d, c, a.e)
                    && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
                {
                    // local tg1=tg:GetFirst()
                    let tg1 = group::get_first(d, &tg).nil();
                    // if Duel.IsExistingMatchingCard(s.tgvfilter2,tp,LOCATION_DECK,0,1,nil,tc:GetType()&key,tg1:GetType()&key) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                    let t2 = card::get_type(d, tg1) & KEY;
                    if duel::is_existing_matching_card(d, Some(&|d, x| tgvfilter2(d, x, t1, t2)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
                        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
                    {
                        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
                        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
                        // local tg2=Duel.SelectMatchingCard(tp,s.tgvfilter2,tp,LOCATION_DECK,0,1,1,nil,tc:GetType()&key,tg1:GetType()&key)
                        let tg2 = duel::select_matching_card(d, a.tp, Some(&|d, x| tgvfilter2(d, x, t1, t2)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                        // if #tg2>0 then
                        if !tg2.is_empty() {
                            // Duel.BreakEffect()
                            duel::break_effect(d);
                            // Duel.SendtoGrave(tg2,REASON_EFFECT)
                            duel::send_to_grave(d, &tg2, REASON_EFFECT, None, None);
                        }
                    }
                }
            }
        }
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not (c:IsLevelAbove(3) or c:IsRankAbove(3))
    !(card::is_level_above(d, c, 3) || card::is_rank_above(d, c, 3))
}
