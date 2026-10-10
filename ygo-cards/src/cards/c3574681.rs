//! Golden Rule
use super::prelude::*;

pub const ID: u32 = 3574681;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_CRYSTAL,SET_CRYSTAL_BEAST}
    listed_series: &[SET_CRYSTAL, SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 2 "Crystal Beast" monsters in the Spell/Trap Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy the equipped monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,sg)
    let e = e.nil();
    (
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, sg)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX),
        false,
    )
}
/// `s.spfilter(c,e,tp,sg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and not sg:IsExists(Card.IsCode,1,nil,c:GetCode())
    if !(card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    let codes = card::get_code(d, c);
    !group::is_exists(d, sg, &|d, x| card::is_code(d, x, &codes), 1, NO_EX)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if c:IsLocation(LOCATION_HAND) then ft=ft-1 end
    if card::is_location(d, c, LOCATION_HAND) {
        ft -= 1;
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and ft>1
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && ft > 1
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(c.into()), 1, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOFIELD)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOFIELD as u64), None, None, false)
        .unwrap_or_default();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local chkct=0
    let mut chkct = 0;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
        if duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // chkct=chkct+1
            chkct += 1;
            // Treated as a Continuous Spell
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // if chkct~=2 then return end
    if chkct != 2 {
        return;
    }
    // Duel.RaiseEvent(tg,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, tg.clone(), EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(e), 0, tp, 0, 0);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local eq=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp,tg):GetFirst()
        let sel = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, &tg))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let eq = group::get_first(d, &sel);
        // if not eq then return end
        let Some(eq) = eq else {
            return;
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummon(eq,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.Equip(tp,c,eq) then
        if duel::special_summon(d, eq, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 && duel::equip(d, tp, c, eq, true, false) {
            // Add Equip limit
            // local e1=Effect.CreateEffect(eq)
            let e1 = effect::create_effect(d, eq);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetValue(function(e,c) return e:GetOwner()==c end)
            effect::set_value_fn(d, e1, lua_fn(|d, x: Args| effect::get_owner(d, x.e(0)) == x.card(1)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, c);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
