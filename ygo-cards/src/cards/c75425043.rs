//! PSY-Framegear Alpha
use super::prelude::*;

pub const ID: u32 = 75425043;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PSY_FRAME], listed_names: &[CARD_PSYFRAME_DRIVER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // Must be special summoned by card effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself and "PSY-Frame Driver", add 1 "PSY-Frame" card from deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se:IsHasType(EFFECT_TYPE_ACTIONS)
    let se = a.e(1);
    effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    //     and (Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 or Duel.IsPlayerAffectedByEffect(tp,CARD_PSYFRAME_LAMBDA))
    let eg = a.eg.clone().nil();
    let other = 1 - a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, other), 1, NO_EX)
        && (duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0 || duel::is_player_affected_by_effect(d, a.tp, CARD_PSYFRAME_LAMBDA).is_some())
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_PSYFRAME_DRIVER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c)
    card::is_code(d, c, &[CARD_PSYFRAME_DRIVER])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_PSYFRAME_DRIVER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.thfilter0,tp,LOCATION_DECK,0,1,c)
    card::is_code(d, c, &[CARD_PSYFRAME_DRIVER])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&thfilter0), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.thfilter0(c)`
fn thfilter0(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PSY_FRAME) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PSY_FRAME as u16]) && !card::is_code(d, c, &[ID])
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PSY_FRAME) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PSY_FRAME as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter1(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or not e:GetHandler():IsRelateToEffect(e) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 || !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter2),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nf = aux::necro_valley_filter(|d, c| spfilter2(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // g:AddCard(c)
    let mut g = g;
    group::add_card(&mut g, &CardSet::from_card(c));
    // g:KeepAlive()
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid as i64]);
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(g));
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g2 = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.SendtoHand(g2,nil,REASON_EFFECT)
        duel::send_to_hand(d, g2.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g2)
        duel::confirm_cards(d, 1 - tp, g2);
    }
}
/// `s.rmfilter(c,fid)`
fn rmfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // if not g:IsExists(s.rmfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| rmfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // local tg=g:Filter(s.rmfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| rmfilter(d, c, label), NO_EX);
    // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
