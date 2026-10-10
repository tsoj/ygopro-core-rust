//! Dingirsu, the Orcust of the Evening Star
use super::prelude::*;

pub const ID: u32 = 93854893;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_ORCUST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --You can only Special Summon "Dingirsu, the Orcust of the Evening Star(s)" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // --Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,8,2,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        None,
        Some(8),
        2,
        Some(xyz::alter_filter(|d, c, tp, xyzc| ovfilter(d, c, tp, xyzc))),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // --Detach 1 material from this card instead of a card(s) you control being destroyed by battle or card effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(function(e,_c) return s.repfilter(_c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e1, lua_fn(repval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_LINK,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsSetCard(SET_ORCUST,xyzc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c)
        && card::is_type_ctx(d, c, TYPE_LINK, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_set_card_ctx(d, c, &[SET_ORCUST as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsOnField() and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_controler(d, c, tp) && card::is_on_field(d, c) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && !card::is_reason(d, c, REASON_REPLACE)
}
/// `function(e,_c) return s.repfilter(_c,e:GetHandlerPlayer()) end`
fn repval(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let p = effect::get_handler_player(d, a.e(0));
    repfilter(d, c, p)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.matfilter(c,tp)`
fn matfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsFaceup() and c:IsCanBeXyzMaterial(nil,tp,REASON_EFFECT)
    card::is_race(d, c, RACE_MACHINE) && card::is_faceup(d, c) && card::is_can_be_xyz_material(d, c, None, Some(tp), Some(REASON_EFFECT))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=e:GetHandler():IsType(TYPE_XYZ) and Duel.IsExistingMatchingCard(s.matfilter,tp,LOCATION_REMOVED,0,1,nil,tp)
    let h = effect::get_handler(d, a.e).nil();
    let b2 = card::is_type(d, h, TYPE_XYZ) && duel::is_existing_matching_card(d, Some(&|d, x| matfilter(d, x, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_ONFIELD)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --Send 1 card your opponent controls to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // --Attach 1 of your banished Machine monsters to this card as material
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not (c:IsRelateToEffect(e) and c:IsType(TYPE_XYZ)) then return end
        if !(card::is_relate_to_effect(d, c, a.e) && card::is_type(d, c, TYPE_XYZ)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
        // local g=Duel.SelectMatchingCard(tp,s.matfilter,tp,LOCATION_REMOVED,0,1,1,nil,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| matfilter(d, x, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Overlay(c,g)
            duel::overlay(d, c, g, false);
        }
    }
}
