//! D Cubed
use super::prelude::*;

pub const ID: u32 = 99357565;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_DESTINY_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // setcode
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(scop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // to grave
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(tgcon)));
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
}
// function s.scop(e,tp,eg,ep,ev,re,r,rp)
fn scop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_ADD_SETCODE)
        effect::set_code(d, e1, EFFECT_ADD_SETCODE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(SET_DESTINY_HERO)
        effect::set_value(d, e1, SET_DESTINY_HERO as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.discardable(c) (Card.IsDiscardable)
fn is_discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&is_discardable), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local ct=2
    let mut ct: i64 = 2;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // ct=math.min(ct,Duel.GetLocationCount(tp,LOCATION_MZONE),Duel.GetMatchingGroupCount(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,nil,e,tp))
    let loc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let cnt = duel::get_matching_group_count(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, NO_EX) as i64;
    ct = ct.min(loc).min(cnt);
    // local cg=Duel.DiscardHand(tp,Card.IsDiscardable,1,ct,REASON_COST|REASON_DISCARD,nil)
    let cg = duel::discard_hand(d, tp, Some(&is_discardable), 1, ct as u16, REASON_COST | REASON_DISCARD, NO_EX);
    // e:SetLabel(cg)
    effect::set_label(d, e, &[cg as i64]);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,e:GetLabel(),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK)
    let label = effect::get_label(d, e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, label as u8, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_DESTINY_HERO)
    !card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,chk)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, e);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // if ft>=ct then
    if ft >= ct {
        // if ft>1 and ct>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
        if ft > 1 && ct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,ct,ct,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK,
            0,
            ct as u16,
            ct as u16,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
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
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e3 = effect::create_effect(d, h);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
// function s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_DESTINY_HERO) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_able_to_grave(d, c, None)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
