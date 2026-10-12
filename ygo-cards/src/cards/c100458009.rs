//! Am Duasennet
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 100458009;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZENET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(Ritual.Target(ritual_params))
    let tg = ritual::target(d, ritual_params());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(Ritual.Operation(ritual_params))
    let op = ritual::operation(d, ritual_params());
    effect::set_operation(d, e1, Some(op));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(destroy_replace_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e2, Some(operation(destroy_replace_op)));
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let p = effect::get_handler_player(d, e);
        repfilter(d, c, p)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The parameters of `Ritual.Target(ritual_params)` / `Ritual.Operation(ritual_params)`.
fn ritual_params() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        // filter=function(c) return c:IsSetCard(SET_ZENET) end
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_ZENET as u16]))),
        location: Some(LOCATION_HAND | LOCATION_GRAVE),
        // matfilter=function(c) return c:IsOriginalType(TYPE_NORMAL) and (c:IsFaceup() or not c:IsOnField()) and c:IsAbleToGrave() end
        matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| {
            card::is_original_type(d, c, TYPE_NORMAL) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_ONFIELD)) && card::is_able_to_grave(d, c, None)
        }))),
        extratg: Some(cost_target(extratg)),
        // extraop=function(mg,e,tp,eg,ep,ev,re,r,rp) Duel.SendtoGrave(mg,REASON_EFFECT|REASON_MATERIAL|REASON_RITUAL) end
        extraop: Some(Arc::new(extraop)),
        extrafil: Some(Arc::new(extrafil)),
        forcedselection: Some(Arc::new(forcedselection)),
        stage2: Some(Arc::new(stage2)),
        ..Default::default()
    }
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_type(d, c, TYPE_NORMAL) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsOriginalType(TYPE_NORMAL) and c:IsControler(tp) and c:IsOnField() and c:IsFaceup()
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_original_type(d, c, TYPE_NORMAL)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// The target of EFFECT_DESTROY_REPLACE: `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destroy_replace_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// The operation of EFFECT_DESTROY_REPLACE: `Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE)`
fn destroy_replace_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `extraop(mg,e,tp,eg,ep,ev,re,r,rp)`
fn extraop(d: &mut Duel, mg: &CardSet, _a: &EffArgs, _rc: CardId) {
    // Duel.SendtoGrave(mg,REASON_EFFECT|REASON_MATERIAL|REASON_RITUAL)
    duel::send_to_grave(d, mg.clone(), REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None, None);
}
/// `extrafil(e,tp,mg)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    Some(duel::get_matching_group(d, Some(&|d, c| card::is_original_type(d, c, TYPE_NORMAL)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, NO_EX))
}
/// `forcedselection(e,tp,sg,ritc)`: `sg:GetClassCount(Card.GetCode)==#sg`
fn forcedselection(d: &mut Duel, _e: EffectId, _tp: u8, sg: &CardSet, _ritc: CardId) -> (bool, bool) {
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len(), false)
}
/// `stage2(mg,e,tp,eg,ep,ev,re,r,rp,ritc)`
fn stage2(d: &mut Duel, _mg: &CardSet, a: &EffArgs, ritc: CardId) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_GRAVE,0,1,nil,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        let nv = aux::necro_valley_filter(|d, c| eqfilter(d, c, tp));
        if duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
            // local eqc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
            let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if not eqc then return end
            let Some(eqc) = group::get_first(d, &g) else { return };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Equip(tp,eqc,ritc) then
            if duel::equip(d, tp, eqc, ritc, true, false) {
                // local e0=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e0 = effect::create_effect(d, h);
                // e0:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
                // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e0:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
                // e0:SetValue(function(e,c) return c==ritc end)
                effect::set_value_fn(d, e0, effect_card_fn(move |_d, _e, x| x == ritc));
                // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
                // eqc:RegisterEffect(e0)
                card::register_effect(d, eqc, e0, false);
            }
        }
    }
}
