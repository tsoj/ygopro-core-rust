//! D/D/D Vice King Requiem
use super::prelude::*;

pub const ID: u32 = 25857977;

// s.listed_series={SET_DDD,SET_DARK_CONTRACT}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DDD, SET_DARK_CONTRACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // You can target any number of "Dark Contract" cards you control; destroy them, and if you do, Special Summon this card from your Pendulum Zone, then you can increase or decrease its Level by the number destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You cannot Special Summon monsters, except "D/D/D" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| splimit(d, c))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // A "D/D/D" monster that was Fusion, Synchro, Xyz, or Link Summoned using this card on the field as material gains this effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_EVENT_PLAYER)
    effect::set_property(d, e3, EFFECT_FLAG_EVENT_PLAYER, 0);
    // e3:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.efcon)
    effect::set_condition(d, e3, Some(condition(efcon)));
    // e3:SetOperation(s.efop)
    effect::set_operation(d, e3, Some(operation(efop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsSetCard(SET_DDD)
    !card::is_set_card(d, c, &[SET_DDD as u16])
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARK_CONTRACT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
}
/// `s.desfilter2(c,e)`
fn desfilter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return s.desfilter(c) and c:IsCanBeEffectTarget(e)
    desfilter(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && desfilter(d, chkc);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter2,tp,LOCATION_ONFIELD,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| desfilter2(d, c, a.e)), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and aux.SelectUnselectGroup(g,e,tp,1,#g,aux.ChkfMMZ(1),0) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(g.len()), Some(&aux::chkf_mmz(1)), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,#g,aux.ChkfMMZ(1),1,tp,HINTMSG_DESTROY)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(g.len()), Some(&aux::chkf_mmz(1)), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let mut ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>=0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) >= 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        if card::get_level(d, c) == 1 {
            // Duel.SelectOption(tp,aux.Stringid(id,2))
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]);
        } else {
            // sel=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
            // (sel is a Lua global: when the Level is 1 it keeps its previous value)
            let sel = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
            set_lua_global(d, "sel", Value::Int(sel as i64));
        }
        // if sel==1 then ct=-ct end
        if matches!(lua_global(d, "sel"), Value::Int(1)) {
            ct = -ct;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(ct)
        effect::set_value(d, e1, ct as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.efcon(e,tp,...)`
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local p=e:GetHandler()
    let p = effect::get_handler(d, a.e).nil();
    // return r&(REASON_FUSION|REASON_SYNCHRO|REASON_XYZ|REASON_LINK)>0 and p:IsPreviousLocation(LOCATION_ONFIELD)
    //   and p:GetReasonCard():IsSetCard(SET_DDD)
    (a.r & (REASON_FUSION | REASON_SYNCHRO | REASON_XYZ | REASON_LINK)) > 0
        && card::is_previous_location(d, p, LOCATION_ONFIELD)
        && card::is_set_card(d, card::get_reason_card(d, p).nil(), &[SET_DDD as u16])
}
/// `s.efop(e,tp,...)`
fn efop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // Provide the effect to the summoned monster
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_RECOVER+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_RECOVER + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // Register a client hint
    // local e2=Effect.CreateEffect(rc)
    let e2 = effect::create_effect(d, rc);
    // e2:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e2, aux::stringid(ID, 5));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e2,true)
    card::register_effect(d, rc, e2, true);
    // If the monster was not an effect monster already, it becomes one
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e3, EFFECT_ADD_TYPE);
        // e3:SetValue(TYPE_EFFECT)
        effect::set_value(d, e3, TYPE_EFFECT as i64);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e3,true)
        card::register_effect(d, rc, e3, true);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARK_CONTRACT) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16]) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,...,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //   and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_ONFIELD as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1000);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_GRAVE|LOCATION_ONFIELD,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // if Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //   and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_DECK | LOCATION_EXTRA)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && duel::destroy(d, CardSet::from_card(tc.nil()), REASON_EFFECT, None, None) > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,1000,REASON_EFFECT)
        duel::recover(d, a.tp, 1000, REASON_EFFECT, false, None);
    }
}
