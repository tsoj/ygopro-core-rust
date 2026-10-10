//! PSY-Framegear Delta
use super::prelude::*;

pub const ID: u32 = 74203495;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_PSYFRAME_DRIVER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // Must be special summoned by a card effect
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
    // Special summon itself and "PSY-Frame Driver", negate the activation of a spell card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se:IsHasType(EFFECT_TYPE_ACTIONS)
    let se = a.e(1);
    effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and re:IsSpellEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    //     and (Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 or Duel.IsPlayerAffectedByEffect(tp,CARD_PSYFRAME_LAMBDA))
    let re = a.re.nil();
    a.ep != a.tp
        && effect::is_spell_effect(d, re)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
        && (duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0 || duel::is_player_affected_by_effect(d, a.tp, CARD_PSYFRAME_LAMBDA).is_some())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_PSYFRAME_DRIVER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_PSYFRAME_DRIVER]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsStatus(STATUS_CHAINING)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let e = a.e;
        let tp = a.tp;
        return !card::is_status(d, h, STATUS_CHAINING)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
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
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nf = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local c=e:GetHandler()
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
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
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
