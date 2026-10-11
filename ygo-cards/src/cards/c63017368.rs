//! Wedju Temple
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 63017368;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MILLENNIUM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 monster in the Spell/Trap Zone as a Continuous Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place "Millennium" monsters that are destroyed in the S/T zone instead of sending them to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // e3:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) and c:HasFlagEffect(id) end)
    effect::set_value_fn(d, e3, effect_card_value(repvalue));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.hplfilter(c,tp)
fn hplfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_monster(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
// function s.dplfilter(c,tp)
fn dplfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MILLENNIUM) and s.hplfilter(c,tp)
    card::is_set_card(d, c, &[SET_MILLENNIUM as u16]) && hplfilter(d, c, tp)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>1
    if a.chk == Some(0) {
        // and Duel.IsExistingMatchingCard(s.hplfilter,tp,LOCATION_HAND,0,1,nil,tp)
        // and Duel.IsExistingMatchingCard(s.dplfilter,tp,LOCATION_DECK,0,1,nil,tp) end
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| hplfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| dplfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local hc=Duel.SelectMatchingCard(tp,s.hplfilter,tp,LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let hg = duel::select_matching_card(d, tp, Some(&|d, c| hplfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let hc = group::get_first(d, &hg);
    // if hc and s.place(hc,c,tp) then
    if let Some(hc) = hc {
        if place(d, hc, c, tp) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
            // local dc=Duel.SelectMatchingCard(tp,s.dplfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
            let dg = duel::select_matching_card(d, tp, Some(&|d, c| dplfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            let dc = group::get_first(d, &dg);
            // if not dc then return end
            let Some(dc) = dc else {
                return;
            };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // s.place(dc,c,tp)
            place(d, dc, c, tp);
        }
    }
}
// function s.place(c,rc,tp)
fn place(d: &mut Duel, c: CardId, rc: CardId, tp: u8) -> bool {
    // if not Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then return false end
    if !duel::move_to_field(d, c, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        return false;
    }
    // Treated as a Continuous Spell
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // return true
    true
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MILLENNIUM) and c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT|REASON_BATTLE)
    card::is_set_card(d, c, &[SET_MILLENNIUM as u16])
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
// e3:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) and c:HasFlagEffect(id) end)
fn repvalue(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    (repfilter(d, c, p) && card::has_flag_effect(d, c, ID, None)) as i64
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=eg:Filter(s.repfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return ft>0 and #g>0 and not e:GetHandler():IsStatus(STATUS_DESTROY_CONFIRMED) end
    if a.chk == Some(0) {
        return ft > 0 && !g.is_empty() && !card::is_status(d, h, STATUS_DESTROY_CONFIRMED);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // if #g>1 then
        if g.len() > 1 {
            // ft=math.min(ft,#g)
            ft = ft.min(g.len() as i32);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
            // g=g:Select(tp,1,ft,nil)
            g = group::select(d, &g, tp, 1, ft as u16, false, NO_EX).nil();
        }
        // for sc in g:Iter() do
        for sc in group::iter(d, &g) {
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD,0,1)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 0, 1, 0, 0);
        }
        // e:SetLabelObject(g)
        // g:KeepAlive()
        effect::set_label_object(d, a.e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // (tc:IsMonsterCard() is the last argument of the MoveToField below)
        let mon = card::is_monster_card(d, tc);
        // if Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
        if duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, mon, None) {
            // Treated as a Continuous Spell
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
