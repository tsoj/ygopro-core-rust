//! G - Ballpark
use super::prelude::*;

pub const ID: u32 = 58012707;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent damage and send 1 Insect monster from the Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Insect Normal monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetBattleDamage(tp)>0 or Duel.GetBattleDamage(1-tp)>0
    duel::get_battle_damage(d, a.tp) > 0 || duel::get_battle_damage(d, 1 - a.tp) > 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_INSECT) and c:IsAbleToGrave()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_INSECT) && card::is_able_to_grave(d, c, None)
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
/// `s.filter(c,e,tp,cd)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cd: u32) -> bool {
    // return c:IsCode(cd) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[cd]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_GRAVE) and tc:IsType(TYPE_NORMAL) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0
            && card::is_location(d, tc, LOCATION_GRAVE)
            && card::is_type(d, tc, TYPE_NORMAL)
        {
            // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_HAND,0,nil,e,tp,tc:GetCode())
            let code = card::get_code(d, tc).first().copied().unwrap_or(0);
            let nvf = aux::necro_valley_filter(move |d, c| filter(d, c, e, tp, code));
            let g = duel::get_matching_group(d, Some(&nvf), tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_HAND, 0, NO_EX);
            // local ct=Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and math.min(1,Duel.GetLocationCount(tp,LOCATION_MZONE),#g)
            //     or math.min(#g,Duel.GetLocationCount(tp,LOCATION_MZONE))
            let loc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
            let ct = if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
                1i64.min(loc).min(g.len() as i64)
            } else {
                (g.len() as i64).min(loc)
            };
            // if ct>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if ct > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,ct,nil)
                let sg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_EFFECT)
    //     and c:GetReasonPlayer()~=tp
    card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) != tp
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsType(TYPE_NORMAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_INSECT) && card::is_type(d, c, TYPE_NORMAL) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nvf = aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nvf), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
