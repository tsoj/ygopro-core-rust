//! Bujintei Kagutsuchi
use super::prelude::*;

pub const ID: u32 = 1855932;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BUJIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_BEASTWARRIOR),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_BEASTWARRIOR, sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,5)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 5);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,5,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 5, REASON_EFFECT);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsSetCard,nil,SET_BUJIN)
        let operated = duel::get_operated_group(d);
        let ct = group::filter_count(d, &operated, &|d, x| card::is_set_card(d, x, &[SET_BUJIN as u16]), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(ct*100)
            effect::set_value(d, e1, (ct * 100) as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_BUJIN) and c:IsRace(RACE_BEASTWARRIOR)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_BUJIN as u16])
        && card::is_race(d, c, RACE_BEASTWARRIOR)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsDestructable(e) and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_destructable(d, c, Some(a.e)) && group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX);
    }
    // if c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) and Duel.SelectEffectYesNo(tp,c,96) then
    if card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT) && duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT);
        // local g=eg:Filter(s.repfilter,nil,tp)
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, x| repfilter(d, x, tp), NO_EX);
        // if #g==1 then
        if g.len() == 1 {
            // e:SetLabelObject(g:GetFirst())
            let first = group::get_first(d, &g).nil();
            effect::set_label_object(d, a.e, Value::Card(first));
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // e:SetLabelObject(cg:GetFirst())
            let first = group::get_first(d, &cg).nil();
            effect::set_label_object(d, a.e, Value::Card(first));
        }
        return true;
    }
    false
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
