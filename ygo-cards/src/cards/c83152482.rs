//! Union Carrier
use super::prelude::*;

pub const ID: u32 = 83152482;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,nil,s.spcheck)
    link::add_procedure(d, c, None, 2, None, Some(group_check(|d, g, lc, sumtype, tp| spcheck(d, g, lc, sumtype, tp))), None, None);
    // --cannot be used as link material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetCondition(s.lkcon)
    effect::set_condition(d, e1, Some(condition(lkcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Equip
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lkcon(e)`
fn lkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsLinkSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_link_summoned(d, c)
}
/// `s.spcheck(g,lc,sumtype,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckSameProperty(Card.GetRace,lc,sumtype,tp) or g:CheckSameProperty(Card.GetAttribute,lc,sumtype,tp)
    group::check_same_property(d, g, &|d, c| card::get_race(d, c)).0 || group::check_same_property(d, g,&|d, c| card::get_attribute(d, c) as u64).0
}
/// `s.filter(c,tp)`: `tp` is nil when the core checks a chosen target (Lua raises an error then)
fn filter(d: &mut Duel, c: CardId, tp: Option<u8>) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,tp,c)
    card::is_faceup(d, c) && {
        let tp = tp.nil();
        duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, tp, c)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX)
    }
}
/// `s.eqfilter(c,tp,oc)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8, oc: CardId) -> bool {
    // return (c:IsOriginalRace(oc:GetOriginalRace()) or c:IsOriginalAttribute(oc:GetOriginalAttribute()))
    //     and c:CheckUniqueOnField(tp) and c:IsMonster() and not c:IsForbidden()
    (card::is_original_race(d, c, card::get_original_race(d, oc)) || card::is_original_attribute(d, c, card::get_original_attribute(d, oc)))
        && card::check_unique_on_field(d, c, tp, None, None)
        && card::is_monster(d, c)
        && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, None) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, Some(tp))), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let tp = a.tp;
    duel::select_target(d, tp, Some(&|d, x| filter(d, x, Some(tp))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,tp,tc):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| eqfilter(d, x, tp, tc)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if not ec then return end
    let Some(ec) = group::get_first(d, &sel) else { return };
    // local loc=ec:GetLocation()
    let loc = card::get_location(d, ec);
    // if s.equipop(tc,e,tp,ec) and loc==LOCATION_DECK then
    if equipop(d, tc, a.e, tp, ec) && loc == LOCATION_DECK {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetLabel(ec:GetCode())
        let code = card::get_code(d, ec)[0];
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetTarget(s.splimit)
        effect::set_target(d, e1, Some(lua_fn(splimit)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true) then return false end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        return false;
    }
    // local e1=Effect.CreateEffect(tc)
    let e1 = effect::create_effect(d, tc);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetValue(true)
    effect::set_value(d, e2, 1);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // return true
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsCode(e:GetLabel())
    let label = effect::get_label(d, a.e(0));
    card::is_code(d, a.c(1), &[label as u32])
}
