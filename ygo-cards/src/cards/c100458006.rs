//! Beresennet Em Heru
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 100458006;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZENET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local ritual_params={...} (see ritual_params)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(Ritual.Target(ritual_params))
    let tg = ritual::target(d, ritual_params());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(Ritual.Operation(ritual_params))
    let op = ritual::operation(d, ritual_params());
    effect::set_operation(d, e1, Some(op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
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
/// `s.drconfilter(c,tp)`
fn drconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and not c:IsType(TYPE_TOKEN) and c:IsSummonPlayer(tp) and c:IsFaceup()
    card::is_type(d, c, TYPE_NORMAL) && !card::is_type(d, c, TYPE_TOKEN) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.drconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| drconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
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
/// `extrafil(e,tp,mg)`: `Duel.GetMatchingGroup(Card.IsOriginalType,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,nil,TYPE_NORMAL)`
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
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        let nv = aux::necro_valley_filter(|d, c| eqfilter(d, c, tp));
        if duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
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
