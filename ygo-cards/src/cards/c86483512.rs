//! Virtual World Roshi - Laolao
//! Scripted by Hel
use super::prelude::*;

pub const ID: u32 = 86483512;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VIRTUAL_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 card to the GY and Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.togravefilter(c,ctype)`
fn togravefilter(d: &mut Duel, c: CardId, ctype: u32) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and not c:IsType(ctype&key) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) && !card::is_type(d, c, ctype & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP)) && card::is_able_to_grave(d, c, None)
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VIRTUAL_WORLD) and Duel.IsExistingMatchingCard(s.togravefilter,tp,LOCATION_DECK,0,1,nil,c:GetType())
    if !card::is_faceup(d, c) || !card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) {
        return false;
    }
    let t = card::get_type(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| togravefilter(d, x, t)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_ONFIELD,0,1,nil,tp) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && {
                let c = effect::get_handler(d, a.e).nil();
                card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.ssummonfilter(c,tgid,e,tp)`
fn ssummonfilter(d: &mut Duel, c: CardId, tgid: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and c:IsMonster() and not c:IsCode(tgid) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[tgid])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not (c:IsLevelAbove(3) or c:IsRankAbove(3))
    !(card::is_level_above(d, c, 3) || card::is_rank_above(d, c, 3))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // Send to GY and Special Summon
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,s.togravefilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetType())
            let t1 = card::get_type(d, tc);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, x| togravefilter(d, x, t1)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 then
            if !g.is_empty() && duel::send_to_grave(d, &g, REASON_EFFECT, None, None) > 0 {
                // local ogc=Duel.GetOperatedGroup():GetFirst()
                let og = duel::get_operated_group(d);
                let ogc = group::get_first(d, &og).nil();
                // if ogc:IsLocation(LOCATION_GRAVE) and c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
                if card::is_location(d, ogc, LOCATION_GRAVE)
                    && card::is_relate_to_effect(d, c, a.e)
                    && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
                {
                    // Special Summon from GY
                    // local gss=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.ssummonfilter),tp,LOCATION_GRAVE,0,nil,g:GetFirst():GetCode(),e,tp)
                    let g1 = group::get_first(d, &g).nil();
                    let code = card::get_code(d, g1)[0];
                    let gss = duel::get_matching_group(
                        d,
                        Some(&aux::necro_valley_filter(|d, x| ssummonfilter(d, x, code, a.e, a.tp))),
                        a.tp,
                        LOCATION_GRAVE,
                        0,
                        NO_EX,
                    );
                    // if #gss>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                    if !gss.is_empty()
                        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
                    {
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                        // local sg=gss:Select(tp,1,1,nil)
                        let sg = group::select(d, &gss, a.tp, 1, 1, false, NO_EX).nil();
                        // if sg:GetFirst() and Duel.SpecialSummonStep(sg:GetFirst(),0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
                        if let Some(sc) = group::get_first(d, &sg) {
                            if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
                                // Negate its effects
                                // local e2=Effect.CreateEffect(c)
                                let e2 = effect::create_effect(d, c);
                                // e2:SetType(EFFECT_TYPE_SINGLE)
                                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                                // e2:SetCode(EFFECT_DISABLE)
                                effect::set_code(d, e2, EFFECT_DISABLE);
                                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                                // sg:GetFirst():RegisterEffect(e2,true)
                                card::register_effect(d, sc, e2, true);
                                // local e3=Effect.CreateEffect(c)
                                let e3 = effect::create_effect(d, c);
                                // e3:SetType(EFFECT_TYPE_SINGLE)
                                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                                // e3:SetCode(EFFECT_DISABLE_EFFECT)
                                effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
                                // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
                                effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
                                // sg:GetFirst():RegisterEffect(e3,true)
                                card::register_effect(d, sc, e3, true);
                                // Duel.SpecialSummonComplete()
                                duel::special_summon_complete(d);
                            }
                        }
                    }
                }
            }
        }
    }
}
