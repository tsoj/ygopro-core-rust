//! 幻奏の音姫スペクタキュラー・バッハ
//! Bacha the Melodious Maestra
use super::prelude::*;

pub const ID: u32 = 56208713;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MELODIOUS],
    material_setcode: &[SET_MELODIOUS],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusiom Summon procedure
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MELODIOUS),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_MELODIOUS as u16], Some(fc), st, Some(tp)))),
            2,
        )],
    );
    // Your "Melodious" Fusion Monsters' activated effects cannot be negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e1, EFFECT_CANNOT_DISEFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(fusion_summoned_cond)));
    // e1:SetValue(s.effval)
    effect::set_value_fn(d, e1, lua_fn(effval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Melodious" monster from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.decksptg)
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    // e2:SetOperation(s.deckspop)
    effect::set_operation(d, e2, Some(operation(deckspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Melodious" monster from your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.gysptg)
    effect::set_target(d, e3, Some(lua_fn(gysptg)));
    // e3:SetOperation(s.gyspop)
    effect::set_operation(d, e3, Some(operation(gyspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fusion_summoned_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.effval(e,ct)`
fn effval(d: &mut Duel, a: Args) -> Value {
    let ct = a.int(1) as u8;
    let p = effect::get_handler_player(d, a.e(0));
    // return Chain.IsTriggeringPlayer(ct,e:GetHandlerPlayer())
    //     and Chain.IsTriggeringType(ct,TYPE_FUSION)
    //     and Chain.IsTriggeringSetcode(ct,SET_MELODIOUS)
    let player = match duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_PLAYER]).first() {
        None | Some(Value::Nil) => Value::Nil,
        Some(v) => Value::Bool(v.int() == p as i64),
    };
    if !player.truthy() {
        return player;
    }
    let typ = chain::is_triggering(d, ct, chain::Prop::Type, &[TYPE_FUSION as i64]);
    if !typ.truthy() {
        return typ;
    }
    chain::is_triggering(d, ct, chain::Prop::SetCard, &[SET_MELODIOUS as i64])
}
/// `s.deckspfilter(c,e,tp)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MELODIOUS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MELODIOUS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| deckspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| deckspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MELODIOUS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_MELODIOUS as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gysptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let chk = a.int(8);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.gyspfilter(chkc,e,tp) end
    if let Some(chkc) = a.card(9) {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && gyspfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if chk == 0 {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| gyspfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| gyspfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
